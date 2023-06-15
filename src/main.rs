use iron_tunnel::core::quic::QuicServer;
use std::net::SocketAddr;
use log::LevelFilter;
use env_logger::Builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    Builder::new()
        .filter(None, LevelFilter::Info)
        .init();

    let bind_addr = SocketAddr::from(([0, 0, 0, 0], 4433));
    let server = QuicServer::new(&bind_addr).await?;
    server.run().await;
    Ok(())
}
