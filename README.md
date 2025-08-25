# P2PChat

Encrypted peer-to-peer chat application written in Rust.

---

## ⚡ Open Source Notice

This code is **free to modify, improve, and adapt**.  
Feel free to enhance it, change it, or use it in your projects.  
It is **completely free and open-source**.

---

## 📦 Project Structure

| File / Module       | Purpose / Functionality |
|--------------------|-----------------------|
| `src/main.rs`       | Entry point of the application. Handles CLI arguments and orchestrates the chat workflow. |
| `src/args.rs`       | Defines CLI argument parsing with `clap`. |
| `src/input.rs`      | Handles reading user input from the terminal. |
| `src/message.rs`    | Defines message types (AboutMe, ChatMessage) and serialization. |
| `src/ticket.rs`     | Handles TopicId, Ticket creation, and serialization for joining rooms. |
| `Cargo.toml`        | Rust package configuration and dependency definitions. |

---

## 📖 Dependencies

```toml
anyhow = "1.0.95"
clap = { version = "4.5.27", features = ["derive"] }
data-encoding = "2.7.0"
futures-lite = "2.6.0"
iroh = "0.31.0"
iroh-gossip = "0.31.0"
rand = "0.8.5"
serde = { version = "1.0.217", features = ["derive"] }
serde_json = "1.0.137"
tokio = "1.43.0"

---

🔗 How P2P Connection Works

1. **Open a Chat Room**
cargo run -- open

This command creates a new chat room and generates a ticket.
Share this ticket with your friends so they can join the same room.
Your terminal will display a TopicId and a ticket string.

2. **Join a Chat Room**
cargo run -- join <ticket>

Replace <ticket> with the ticket received from the room opener.
Connects your client to the existing chat room using encrypted P2P communication.
Once connected, you can send and receive messages directly with other peers.

3. **Send Messages**

Type your message in the terminal and press Enter to broadcast it to all connected peers.
All messages are encrypted and transmitted peer-to-peer without a central server.
