

// use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub enum Message {
//     Join { node_address: String },
//     Leave { node_address: String },
//     Broadcast { content: String },
//     PeerList { peers: PeerList },
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct PeerList {
//     pub peers: Vec<String>,
// }



// use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Block {
//     pub index: u64,
//     pub timestamp: String,
//     pub data: String,
//     pub previous_hash: String,
//     pub hash: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub enum Message {
//     Join { node_address: String },
//     Leave { node_address: String },
//     Broadcast { content: String },
//     PeerList { peers: Vec<String> },
//     NewBlock { block: Block },
// }



// // messages.rs
// use serde::{Deserialize, Serialize};
// use sha2::Digest;


// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub enum Message {
//     Join { node_address: String },
//     Leave { node_address: String },
//     Broadcast { content: String },
//     PeerList { peers: Vec<String> },
//     NewBlock { block: Block },
//     LeaderAnnouncement { leader: String },
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Block {
//     pub index: u64,
//     pub timestamp: String,
//     pub data: String,
//     pub previous_hash: String,
//     pub hash: String,
// }

// impl Block {
//     pub fn new(index: u64, data: String, previous_hash: String) -> Self {
//         let timestamp = chrono::Utc::now().to_string();
//         let hash = Self::calculate_hash(index, &timestamp, &data, &previous_hash);
//         Block {
//             index,
//             timestamp,
//             data,
//             previous_hash,
//             hash,
//         }
//     }

//     fn calculate_hash(index: u64, timestamp: &str, data: &str, previous_hash: &str) -> String {
//         let mut hasher = sha2::Sha256::new();
//         hasher.update(format!("{}{}{}{}", index, timestamp, data, previous_hash));
//         format!("{:x}", hasher.finalize())
//     }
// }





// use serde::{Deserialize, Serialize};
// use sha2::Digest;

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Block {
//     pub index: u64,
//     pub timestamp: String,
//     pub data: String,
//     pub previous_hash: String,
//     pub hash: String,
// }

// impl Block {
//     pub fn new(index: u64, data: String, previous_hash: String) -> Self {
//         let timestamp = chrono::Utc::now().to_string();
//         let hash = Self::calculate_hash(index, &timestamp, &data, &previous_hash);
//         Block {
//             index,
//             timestamp,
//             data,
//             previous_hash,
//             hash,
//         }
//     }

//     fn calculate_hash(index: u64, timestamp: &str, data: &str, previous_hash: &str) -> String {
//         let mut hasher = sha2::Sha256::new();
//         hasher.update(format!("{}{}{}{}", index, timestamp, data, previous_hash));
//         format!("{:x}", hasher.finalize())
//     }
// }


use serde::{Deserialize, Serialize};
use sha2::Digest;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    Join { node_address: String },
    Leave { node_address: String },
    Broadcast { content: String },
    PeerList { peers: Vec<String> },
    NewBlock { block: Block },
    LeaderAnnouncement { leader: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = chrono::Utc::now().to_string();
        let hash = Self::calculate_hash(index, &timestamp, &data, &previous_hash);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    fn calculate_hash(index: u64, timestamp: &str, data: &str, previous_hash: &str) -> String {
        let mut hasher = sha2::Sha256::new();
        hasher.update(format!("{}{}{}{}", index, timestamp, data, previous_hash));
        format!("{:x}", hasher.finalize())
    }
}
