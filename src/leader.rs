// leader.rs
use std::sync::{Arc, Mutex};
use rand::Rng;
use crate::peer::PeerManager;

#[derive(Clone)]
pub struct LeaderManager {
    leader: Arc<Mutex<Option<String>>>,
}

impl LeaderManager {
    pub fn new() -> Self {
        LeaderManager {
            leader: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_leader(&self, leader: String) {
        let mut current_leader = self.leader.lock().unwrap();
        *current_leader = Some(leader);
    }

    pub fn get_leader(&self) -> Option<String> {
        self.leader.lock().unwrap().clone()
    }

    pub fn is_leader(&self) -> bool {
        self.get_leader().is_some()
    }
}

pub async fn start_leader_election(peer_manager: Arc<PeerManager>, leader_manager: Arc<LeaderManager>) {
    let peers = peer_manager.get_peers();
    if !peers.is_empty() {
        let new_leader = peers[rand::thread_rng().gen_range(0..peers.len())].clone();
        leader_manager.set_leader(new_leader.clone());
        println!("New leader elected: {}", new_leader);
    }
}