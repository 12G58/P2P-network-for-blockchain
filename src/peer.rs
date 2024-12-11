use std::sync::{Arc, Mutex};

#[derive(Clone)] // Derive Clone for PeerManager
pub struct PeerManager {
    peers: Arc<Mutex<Vec<String>>>, // Shared, thread-safe list of peers
}

impl PeerManager {
    /// Create a new PeerManager with an empty peer list
    pub fn new() -> Self {
        PeerManager {
            peers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Add a new peer to the list if it doesn't already exist
    pub fn add_peer(&self, peer: String) {
        let mut peers = self.peers.lock().unwrap();
        if !peers.contains(&peer) {
            peers.push(peer.clone()); // Clone the peer for insertion
            println!("Peer added: {}", peer); // Use the original peer for printing
        } else {
            println!("Peer already exists: {}", peer);
        }
    }

    /// Remove a peer from the list by its address
    pub fn remove_peer(&self, peer: &str) {
        let mut peers = self.peers.lock().unwrap();
        let initial_len = peers.len();
        peers.retain(|p| p != peer);
        if peers.len() < initial_len {
            println!("Peer removed: {}", peer);
        } else {
            println!("Peer not found: {}", peer);
        }
    }

    /// Get a list of all peers
    pub fn get_peers(&self) -> Vec<String> {
        let peers = self.peers.lock().unwrap();
        peers.clone()
    }
}
