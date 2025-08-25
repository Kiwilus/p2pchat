// Chat message types and serialization
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    AboutMe {
        node_id: iroh::NodeId,
        name: String,
    },
    ChatMessage {
        node_id: iroh::NodeId,
        text: String,
    },
}

impl Message {
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("serde_json::to_vec is infallible")
    }
}
