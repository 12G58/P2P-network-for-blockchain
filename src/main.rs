mod network;
mod peer;
mod messages;

use network::run_server;
use peer::PeerManager;

#[tokio::main]
async fn main() {
    let peer_manager = PeerManager::new();

    // Run server on port 8080
    tokio::spawn(async move {
        run_server("127.0.0.1:8080", peer_manager).await;
    });

    println!("Press Ctrl+C to stop the server.");
    tokio::signal::ctrl_c().await.unwrap();
}
