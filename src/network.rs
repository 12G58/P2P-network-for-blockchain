use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt};
use crate::messages::{Message};
use crate::peer::PeerManager;
use serde_json;

pub async fn run_server(address: &str, peer_manager: PeerManager) {
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Server running at {}", address);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let peer_manager = peer_manager.clone();
        tokio::spawn(async move {
            handle_connection(socket, peer_manager).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream, peer_manager: PeerManager) {
    let mut buffer = [0; 1024];
    if let Ok(size) = socket.read(&mut buffer).await {
        if size > 0 {
            let received = String::from_utf8_lossy(&buffer[..size]);
            if let Ok(message) = serde_json::from_str::<Message>(&received) {
                match message {
                    Message::Join { node_address } => {
                        peer_manager.add_peer(node_address.clone());
                        println!("Node joined: {}", node_address);
                    }
                    Message::Leave { node_address } => {
                        peer_manager.remove_peer(&node_address);
                        println!("Node left: {}", node_address);
                    }
                    Message::Broadcast { content } => {
                        println!("Broadcast message: {}", content);
                    }
                }
            }
        }
    }
}
