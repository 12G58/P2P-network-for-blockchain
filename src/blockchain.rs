// use super::messages::Block;
// use chrono::Utc;

// pub struct Blockchain {
//     chain: Vec<Block>,
// }

// impl Blockchain {
//     pub fn new() -> Self {
//         let genesis_block = Block {
//             index: 0,
//             timestamp: Utc::now().to_rfc3339(),
//             data: "Genesis Block".to_string(),
//             previous_hash: "0".to_string(),
//             hash: "0".to_string(),
//         };

//         Blockchain {
//             chain: vec![genesis_block],
//         }
//     }

//     pub fn add_block(&mut self, data: String) {
//         let last_block = self.chain.last().unwrap();
//         let new_block = Block {
//             index: last_block.index + 1,
//             timestamp: Utc::now().to_rfc3339(),
//             data,
//             previous_hash: last_block.hash.clone(),
//             hash: "TODO".to_string(),
//         };
//         self.chain.push(new_block);
//     }

//     pub fn latest_block(&self) -> &Block {
//         self.chain.last().unwrap()
//     }
// }


// blockchain.rs
use crate::messages::Block;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Blockchain {
    chain: Arc<Mutex<Vec<Block>>>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        Blockchain {
            chain: Arc::new(Mutex::new(vec![genesis_block])),
        }
    }

    pub fn add_block(&self, block: Block) {
        let mut chain = self.chain.lock().unwrap();
        if let Some(last_block) = chain.last() {
            if block.previous_hash == last_block.hash {
                chain.push(block);
            }
        }
    }

    pub fn get_chain(&self) -> Vec<Block> {
        self.chain.lock().unwrap().clone()
    }
}