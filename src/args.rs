// Command-line argument definitions for the chat app
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(long)]
    pub no_relay: bool, // disable relay mode

    #[clap(short, long)]
    pub name: Option<String>, // optional username

    #[clap(subcommand)]
    pub command: MyCommand, // open or join
}

#[derive(Subcommand, Debug)]
pub enum MyCommand {
    Open, // start new chat
    Join { ticket: String }, // join with ticket
}
