

// // ------

// use std::{env, sync::Arc};
// use tokio::time::sleep;
// use std::time::Duration;

// mod network;
// mod peer;
// mod messages;

// use network::run_server;
// use peer::PeerManager;
// use messages::{Message};

// #[tokio::main]
// async fn main() {

//     let args: Vec<String> = env::args().collect();
//     if args.len() != 2 {
//         eprintln!("Usage: {} <port>", args[0]);
//         return;
//     }

//     let port = match args[1].parse::<u16>() {
//         Ok(port) => port,
//         Err(_) => {
//             eprintln!("Invalid port number");
//             return;
//         }
//     };

//     let server_address = format!("127.0.0.1:{}", port);

//     // Create a shared peer manager
//     let peer_manager = Arc::new(PeerManager::new());

//     // Spawn the server
//     let peer_manager_clone = Arc::clone(&peer_manager);
//     tokio::spawn(async move {
//         run_server(&server_address, peer_manager_clone).await;
//     });


//     // Simulate sending a broadcast message
//     let peer_manager_clone_2 = Arc::clone(&peer_manager);
//     tokio::spawn(async move {
//         loop {
//             sleep(Duration::from_secs(7)).await;
            
//             // Move the condition inside the loop to check every iteration
//            if peer_manager_clone_2.has_discovered_peer(){
//              let message_content = "Hello, peers!".to_string();
//             println!("Broadcasting message: {}", message_content);

//             for peer in peer_manager_clone_2.get_peers() {
//                 if let Err(e) =  peer::send_message(
//                     &peer,
//                     Message::Broadcast {
//                         content: message_content.clone(),
//                     },
//                 ).await
//                  {
//                     eprintln!("Broadcast failed: {}", e);
//                 }
//              }
//            }
//         }
//     });

//     // Keep the main thread alive
//     loop {
//         sleep(Duration::from_secs(3600)).await;
//     }
// }



// // main.rs
// use std::{env, sync::Arc};
// use tokio::time::{sleep, Duration};

// mod network;
// mod peer;
// mod messages;
// mod blockchain;
// mod leader;

// use network::run_server;
// use peer::PeerManager;
// use messages::Message;
// use blockchain::Blockchain;
// use leader::{LeaderManager, start_leader_election};

// #[tokio::main]
// async fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 2 {
//         eprintln!("Usage: {} <port>", args[0]);
//         return;
//     }

//     let port = match args[1].parse::<u16>() {
//         Ok(port) => port,
//         Err(_) => {
//             eprintln!("Invalid port number");
//             return;
//         }
//     };

//     let server_address = format!("127.0.0.1:{}", port);

//     // Create shared managers
//     let peer_manager = Arc::new(PeerManager::new());
//     let blockchain = Arc::new(Blockchain::new());
//     let leader_manager = Arc::new(LeaderManager::new());

//     // Start the server
//     let peer_manager_clone = Arc::clone(&peer_manager);
//     let blockchain_clone = Arc::clone(&blockchain);
//     let leader_manager_clone = Arc::clone(&leader_manager);

//     tokio::spawn(async move {
//         run_server(&server_address, peer_manager_clone, blockchain_clone, leader_manager_clone).await;
//     });

//     // Periodic tasks
//     let peer_manager_clone = Arc::clone(&peer_manager);
//     let leader_manager_clone = Arc::clone(&leader_manager);

//     tokio::spawn(async move {
//         loop {
//             sleep(Duration::from_secs(10)).await;
//             if !leader_manager_clone.is_leader() {
//                 println!("Checking for leader...");
//                 start_leader_election(peer_manager_clone.clone(), leader_manager_clone.clone()).await;
//             }
//         }
//     });

//     // Keep the main thread alive
//     loop {
//         sleep(Duration::from_secs(3600)).await;
//     }
// }




use std::{env, sync::Arc};
use tokio::time::{sleep, Duration};

mod network;
mod peer;
mod messages;
mod blockchain;
mod leader;

use peer::PeerManager;
use messages::Message;
use blockchain::Blockchain;
use leader::{LeaderManager, start_leader_election};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <port>", args[0]);
        return;
    }

    let port = match args[1].parse::<u16>() {
        Ok(port) => port,
        Err(_) => {
            eprintln!("Invalid port number");
            return;
        }
    };

    let server_address = format!("127.0.0.1:{}", port);

    // Create shared managers
    let peer_manager = Arc::new(PeerManager::new());
    let blockchain = Arc::new(Blockchain::new());
    let leader_manager = Arc::new(LeaderManager::new());

    // Start the server
    let peer_manager_clone = Arc::clone(&peer_manager);
    let blockchain_clone = Arc::clone(&blockchain);
    let leader_manager_clone = Arc::clone(&leader_manager);

    tokio::spawn(async move {
        network::run_server(&server_address, peer_manager_clone, blockchain_clone, leader_manager_clone).await;
    });

    // Periodic tasks
    let peer_manager_clone = Arc::clone(&peer_manager);
    let leader_manager_clone = Arc::clone(&leader_manager);

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(10)).await;
            if !leader_manager_clone.is_leader() {
                println!("Checking for leader...");
                start_leader_election(peer_manager_clone.clone(), leader_manager_clone.clone()).await;
            }
        }
    });

    // Keep the main thread alive
    loop {
        sleep(Duration::from_secs(3600)).await;
    }
}





