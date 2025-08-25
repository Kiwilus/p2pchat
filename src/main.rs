use std::{
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
    thread,
};

use anyhow::Result;
use clap::Parser;
use futures_lite::StreamExt;
use iroh::{Endpoint, RelayMode, SecretKey};
use iroh_gossip::{
    net::{Event, Gossip, GossipEvent, GossipReceiver, GOSSIP_ALPN},
    proto::TopicId,
};
use tokio::sync::mpsc;

mod args;
mod input;
mod message;
mod ticket;

use args::{Args, MyCommand};
use input::input_loop;
use message::Message;
use ticket::Ticket;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let args = Args::parse();

    // Decide whether we open a new chat or join an existing one
    let (topic, peers) = match &args.command {
        MyCommand::Open => {
            let topic = TopicId::from_bytes(rand::random());
            println!("> opening chat room for topic {}", topic);
            (topic, vec![])
        }
        MyCommand::Join { ticket } => {
            let ticket = Ticket::from_str(ticket)?;
            println!("> joining chat room for topic {}", ticket.topic);
            (ticket.topic, ticket.peers)
        }
    };

    // Relay mode depends on flag
    let relay_mode = if args.no_relay {
        RelayMode::Disabled
    } else {
        RelayMode::Default
    };

    // Create endpoint with random secret key
    let secret_key = SecretKey::generate(rand::rngs::OsRng);
    let endpoint = Endpoint::builder()
        .secret_key(secret_key)
        .relay_mode(relay_mode)
        .bind_addr_v4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0))
        .bind()
        .await?;

    println!("> our node id: {}", endpoint.node_id());

    // Start gossip service
    let gossip = Gossip::builder().spawn(endpoint.clone()).await?;

    // Build ticket with existing peers + our own address
    let me = endpoint.node_addr().await?;
    let mut all_peers = peers.clone();
    all_peers.push(me.clone());
    let ticket = Ticket { topic, peers: all_peers };
    println!("> ticket to join us: {}", ticket);

    // Start protocol router
    let router = iroh::protocol::Router::builder(endpoint.clone())
        .accept(GOSSIP_ALPN, gossip.clone())
        .spawn()
        .await?;

    // Connect to peers if we have any
    if peers.is_empty() {
        println!("> waiting for peers to join us...");
    } else {
        println!("> trying to connect to {} peers", peers.len());
        for peer in peers.iter() {
            endpoint.add_node_addr(peer.clone())?;
        }
    }

    // Subscribe to gossip topic
    let peer_ids = peers.iter().map(|p| p.node_id).collect::<Vec<_>>();
    let (sender, receiver) = gossip.subscribe_and_join(topic, peer_ids).await?.split();
    println!("> connected!!!");

    // Share our name if provided
    if let Some(name) = args.name {
        let message = Message::AboutMe {
            node_id: endpoint.node_id(),
            name,
        };
        sender.broadcast(message.to_bytes().into()).await?;
    }

    // Listen for incoming messages in background
    tokio::spawn(subscribe_loop(receiver));

    // Channel for stdin lines
    let (line_tx, mut line_rx) = mpsc::channel::<String>(32);
    thread::spawn(move || input_loop(line_tx));

    println!("> type a message and hit enter to broadcast...");
    while let Some(text) = line_rx.recv().await {
        let message = Message::ChatMessage {
            node_id: endpoint.node_id(),
            text: text.clone(),
        };
        sender.broadcast(message.to_bytes().into()).await?;
        println!("> sent {}", text);
    }

    router.shutdown().await?;
    Ok(())
}

// Handles incoming gossip messages
async fn subscribe_loop(mut receiver: GossipReceiver) -> Result<()> {
    while let Some(event) = receiver.try_next().await? {
        if let Event::Gossip(gossip_event) = event {
            match gossip_event {
                GossipEvent::Received(msg) => {
                    if let Ok(decoded) = serde_json::from_slice::<Message>(&msg.content) {
                        match decoded {
                            Message::ChatMessage { node_id, text } => {
                                println!("[{}] {}", node_id, text);
                            }
                            Message::AboutMe { node_id, name } => {
                                println!("{} joined the chat! (id: {})", name, node_id);
                            }
                        }
                    } else {
                        println!("(unknown message) {:?}", msg);
                    }
                }
                _ => println!("got event: {:?}", &gossip_event),
            }
        }
    }
    Ok(())
}

