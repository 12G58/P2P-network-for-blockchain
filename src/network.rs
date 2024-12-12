
//--------

// use std::sync::Arc;
// use tokio::io::{AsyncReadExt};
// use tokio::net::{TcpListener, TcpStream};
// use crate::messages::{Message, PeerList};
// use crate::peer::{PeerManager, send_message};

// /// Handle the incoming connection and process messages.
// pub async fn handle_connection(mut socket: TcpStream, peer_manager: Arc<PeerManager>) {
//     let peer_address = match socket.peer_addr() {
//         Ok(addr) => addr.to_string(),
//         Err(e) => {
//             eprintln!("Failed to get peer address: {}", e);
//             return;
//         }
//     };
//     println!("New Connection from {}", peer_address);

//     peer_manager.add_peer(peer_address.clone());

//     let mut buffer = [0; 1024];
//     loop {
//         match socket.read(&mut buffer).await {
//             Ok(size) if size > 0 => {
//                 let received = String::from_utf8_lossy(&buffer[..size]);
//                 match serde_json::from_str::<Message>(&received) {
//                     Ok(message) => {
//                         println!("Received message: {:?}", message);
//                         process_message(message, &peer_address, &peer_manager).await;
//                     }
//                     Err(e) => {
//                         eprintln!("Error parsing message from {}: {}", peer_address, e);
//                     }
//                 }
//             }
//             Ok(_) => {
//                 println!("{} disconnected.", peer_address);
//                 peer_manager.remove_peer(&peer_address);
//                 break;
//             }
//             Err(e) => {
//                 eprintln!("Failed to read from socket ({}): {}", peer_address, e);
//                 peer_manager.remove_peer(&peer_address);
//                 break;
//             }
//         }
//     }
// }

// /// Process a received message.
// async fn process_message(message: Message, sender: &str, peer_manager: &Arc<PeerManager>) {
//     match message {
//         Message::Join { node_address } => {
//             peer_manager.add_peer(node_address.clone());
//             println!("Node joined: {}", node_address);
//            // After joining, request the peer list
//             let peer_list_request = Message::PeerList {
//                 peers: PeerList {
//                     peers: Vec::new(),
//                 },
//             };
//              if let Err(e) = send_message(sender, peer_list_request).await {
//                  eprintln!("Failed to send peer list to {}: {}", sender, e);
//              }
//         }
//         Message::Leave { node_address } => {
//             peer_manager.remove_peer(&node_address);
//             println!("Node left: {}", node_address);
//         }
//         Message::Broadcast { content } => {
//             println!("Broadcasting message from {}: {}", sender, content);
//             for peer in peer_manager.get_peers() {
//                 if peer != sender {
//                      if let Err(e) = send_message(&peer, Message::Broadcast { content: content.clone() }).await {
//                         eprintln!("Failed to send broadcast to {}: {}", peer, e);
//                      }
//                 }
//             }
//         }
//         Message::PeerList { peers } => {
//            for peer in &peers.peers {
//                peer_manager.add_peer(peer.clone());
//             }
//             peer_manager.set_discovered_peer();
//             println!("Updated peer list from {}: {:?}", sender, peers.peers);

//              let peer_list = PeerList {
//                peers: peer_manager.get_peers(),
//              };
//              if let Err(e) = send_message(sender, Message::PeerList { peers: peer_list }).await {
//                  eprintln!("Failed to send peer list to {}: {}", sender, e);
//              }
//              println!("Current peer list: {:?}", peer_manager.get_peers());
//         }
//     }
// }
// /// Run the server, listening for connections.
// pub async fn run_server(address: &str, peer_manager: Arc<PeerManager>) {
//     let listener = match TcpListener::bind(address).await {
//         Ok(listener) => {
//             println!("Server running at {}", address);
//             listener
//         }
//         Err(e) => {
//             eprintln!("Failed to bind to {}: {}", address, e);
//             return;
//         }
//     };

//     loop {
//         match listener.accept().await {
//             Ok((socket, _)) => {
//                 let peer_manager = Arc::clone(&peer_manager);
//                 tokio::spawn(async move {
//                     handle_connection(socket, peer_manager).await;
//                 });
//             }
//             Err(e) => {
//                 eprintln!("Failed to accept connection: {}", e);
//             }
//         }
//     }
// }














// // network.rs
// use std::sync::Arc;
// use tokio::io::AsyncReadExt;
// use tokio::net::{TcpListener, TcpStream};
// use crate::messages::{Message, Block};
// use crate::peer::PeerManager;
// use crate::blockchain::Blockchain;
// use crate::leader::LeaderManager;

// pub async fn handle_connection(
//     mut socket: TcpStream,
//     peer_manager: Arc<PeerManager>,
//     blockchain: Arc<Blockchain>,
//     leader_manager: Arc<LeaderManager>,
// ) {
//     let peer_address = match socket.peer_addr() {
//         Ok(addr) => addr.to_string(),
//         Err(_) => return,
//     };

//     peer_manager.add_peer(peer_address.clone());
//     let mut buffer = vec![0; 1024];

//     pub async fn run_server(
//         address: &str,
//         peer_manager: Arc<PeerManager>,
//         blockchain: Arc<Blockchain>,
//         leader_manager: Arc<LeaderManager>,
//     ) {
//         let listener = TcpListener::bind(address).await.unwrap();
//         println!("Server running on {}", address);
    
//         loop {
//             let (socket, _) = listener.accept().await.unwrap();
//             let peer_manager_clone = Arc::clone(&peer_manager);
//             let blockchain_clone = Arc::clone(&blockchain);
//             let leader_manager_clone = Arc::clone(&leader_manager);
    
//             tokio::spawn(async move {
//                 handle_connection(socket, peer_manager_clone, blockchain_clone, leader_manager_clone).await;
//             });
//         }
//     }

//     loop {
//         match socket.read(&mut buffer).await {
//             Ok(size) if size > 0 => {
//                 if let Ok(message) = serde_json::from_slice::<Message>(&buffer[..size]) {
//                     match message {
//                         Message::Join { node_address } => peer_manager.add_peer(node_address),
//                         Message::Broadcast { content } => println!("Broadcast received: {}", content),
//                         _ => {}
//                     }
//                 }
//             }
//             _ => break,
//         }
//     }
// }





// use std::sync::Arc;
// use tokio::io::AsyncReadExt;
// use tokio::net::{TcpListener, TcpStream};
// use crate::messages::{Message, Block};
// use crate::peer::PeerManager;
// use crate::blockchain::Blockchain;
// use crate::leader::LeaderManager;
// use serde_json;

// pub async fn handle_connection(
//     mut socket: TcpStream,
//     peer_manager: Arc<PeerManager>,
//     blockchain: Arc<Blockchain>,
//     leader_manager: Arc<LeaderManager>,
// ) {
//     let peer_address = match socket.peer_addr() {
//         Ok(addr) => addr.to_string(),
//         Err(_) => return,
//     };

//     peer_manager.add_peer(peer_address.clone());
//     let mut buffer = vec![0; 1024];

//     loop {
//         match socket.read(&mut buffer).await {
//             Ok(size) if size > 0 => {
//                 if let Ok(message) = serde_json::from_slice::<Message>(&buffer[..size]) {
//                     match message {
//                         Message::Join { node_address } => peer_manager.add_peer(node_address),
//                         Message::Broadcast { content } => println!("Broadcast received: {}", content),
//                         _ => {}
//                     }
//                 }
//             }
//             _ => break,
//         }
//     }
// }








use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use crate::messages::{Message, Block};
use crate::peer::PeerManager;
use crate::blockchain::Blockchain;
use crate::leader::LeaderManager;
use serde_json;

pub async fn run_server(
    address: &str,
    peer_manager: Arc<PeerManager>,
    blockchain: Arc<Blockchain>,
    leader_manager: Arc<LeaderManager>,
) {
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Server running on {}", address);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let peer_manager_clone = Arc::clone(&peer_manager);
        let blockchain_clone = Arc::clone(&blockchain);
        let leader_manager_clone = Arc::clone(&leader_manager);

        tokio::spawn(async move {
            handle_connection(socket, peer_manager_clone, blockchain_clone, leader_manager_clone).await;
        });
    }
}

pub async fn handle_connection(
    mut socket: TcpStream,
    peer_manager: Arc<PeerManager>,
    blockchain: Arc<Blockchain>,
    leader_manager: Arc<LeaderManager>,
) {
    let peer_address = match socket.peer_addr() {
        Ok(addr) => addr.to_string(),
        Err(_) => return,
    };

    peer_manager.add_peer(peer_address.clone());
    let mut buffer = vec![0; 1024];

    loop {
        match socket.read(&mut buffer).await {
            Ok(size) if size > 0 => {
                if let Ok(message) = serde_json::from_slice::<Message>(&buffer[..size]) {
                    match message {
                        Message::Join { node_address } => peer_manager.add_peer(node_address),
                        Message::Broadcast { content } => println!("Broadcast received: {}", content),
                        _ => {}
                    }
                }
            }
            _ => break,
        }
    }
}



