
// //---------------

// use std::sync::{Arc, Mutex};
// use tokio::net::TcpStream;
// use tokio::io::{AsyncWriteExt};
// use serde_json;
// use crate::messages::Message;

// #[derive(Clone)]
// pub struct PeerManager {
//     peers: Arc<Mutex<Vec<String>>>, // Shared, thread-safe list of peers
//     discovered_peer: Arc<Mutex<bool>>
// }

// impl PeerManager {
//     /// Create a new PeerManager with an empty peer list
//     pub fn new() -> Self {
//         PeerManager {
//             peers: Arc::new(Mutex::new(Vec::new())),
//             discovered_peer: Arc::new(Mutex::new(false))
//         }
//     }

//     /// Add a new peer to the list if it doesn't already exist
//     pub fn add_peer(&self, peer: String) {
//         let mut peers = self.peers.lock().unwrap();
//         if !peers.contains(&peer) {
//             peers.push(peer.clone()); // Clone the peer for insertion
//             println!("Peer added: {}", peer); // Use the original peer for printing
//         } else {
//             println!("Peer already exists: {}", peer);
//         }
//     }
    
//     pub fn set_discovered_peer(&self) {
//         let mut discovered_peer = self.discovered_peer.lock().unwrap();
//         *discovered_peer = true;
//     }

//     pub fn has_discovered_peer(&self) -> bool {
//         let discovered_peer = self.discovered_peer.lock().unwrap();
//         *discovered_peer
//     }

//     /// Remove a peer from the list by its address
//     pub fn remove_peer(&self, peer: &str) {
//         let mut peers = self.peers.lock().unwrap();
//         let initial_len = peers.len();
//         peers.retain(|p| p != peer);
//         if peers.len() < initial_len {
//             println!("Peer removed: {}", peer);
//         } else {
//             println!("Peer not found: {}", peer);
//         }
//     }

//     /// Get a list of all peers
//     pub fn get_peers(&self) -> Vec<String> {
//         let peers = self.peers.lock().unwrap();
//         peers.clone()
//     }
// }

// // Send message to a peer
// pub async fn send_message(peer_address: &str, message: Message) -> Result<(), String> {
//     match TcpStream::connect(peer_address).await {
//         Ok(mut stream) => {
//             let message_str = serde_json::to_string(&message).unwrap();
//             if let Err(e) = stream.write_all(message_str.as_bytes()).await {
//                let err_msg = format!("Failed to send message to {}: {}", peer_address, e);
//                println!("{}", err_msg);
//                return Err(err_msg)
//             }
//             Ok(())
//         }
//         Err(e) => {
//             let err_msg = format!("Failed to connect to {}: {}", peer_address, e);
//             println!("{}", err_msg);
//             Err(err_msg)
//         }
//     }
// }




// peer.rs
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct PeerManager {
    peers: Arc<Mutex<Vec<String>>>,
}

impl PeerManager {
    pub fn new() -> Self {
        PeerManager {
            peers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_peer(&self, peer: String) {
        let mut peers = self.peers.lock().unwrap();
        if !peers.contains(&peer) {
            peers.push(peer);
        }
    }

    pub fn remove_peer(&self, peer: &str) {
        let mut peers = self.peers.lock().unwrap();
        peers.retain(|p| p != peer);
    }

    pub fn get_peers(&self) -> Vec<String> {
        self.peers.lock().unwrap().clone()
    }
}