// Reads lines from stdin and sends them to the async runtime
use tokio::sync::mpsc;

pub fn input_loop(tx: mpsc::Sender<String>) {
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if tx.blocking_send(line).is_err() {
                break;
            }
        }
    }
}
