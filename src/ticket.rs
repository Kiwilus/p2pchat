// Room invitation containing topic and peer addresses
use std::{fmt, str::FromStr};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use data_encoding::BASE32_NOPAD;
use iroh::NodeAddr;
use iroh_gossip::proto::TopicId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub topic: TopicId,
    pub peers: Vec<NodeAddr>,
}

impl Ticket {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(serde_json::from_slice(bytes)?)
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("serde_json::to_vec is infallible")
    }
}

impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text = BASE32_NOPAD.encode(&self.to_bytes()[..]);
        text.make_ascii_lowercase();
        write!(f, "{}", text)
    }
}

impl FromStr for Ticket {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = BASE32_NOPAD.decode(s.to_ascii_uppercase().as_bytes())?;
        Self::from_bytes(&bytes)
    }
}
