use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Join { node_address: String },
    Leave { node_address: String },
    Broadcast { content: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PeerList {
    peers: Vec<String>,
}
