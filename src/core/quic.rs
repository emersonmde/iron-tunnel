use quinn::{Endpoint, ServerConfig, TransportConfig};
use std::net::SocketAddr;
use tokio::prelude::*;
use log::info;
use quinn::IncomingBiStreams;
use std::sync::Arc;

pub struct QuicServer {
    incoming: quinn::Incoming,
}

impl QuicServer {
    pub async fn new(bind_addr: &SocketAddr) -> Result<Self, quinn::EndpointError> {
        let mut server_config = ServerConfig::default();
        let transport_config = Arc::new(TransportConfig::default());
        server_config.transport = transport_config;

        let mut endpoint_builder = Endpoint::builder();
        endpoint_builder.listen(server_config);

        let (endpoint, incoming) = endpoint_builder.bind(bind_addr)?;

        Ok(Self { incoming })
    }

    pub async fn run(self) -> Result<(), quinn::ConnectionError> {
        info!("QUIC server running on {}", self.incoming.endpoint().local_addr().unwrap());
        self.accept_connections().await
    }

    async fn accept_connections(mut self) -> Result<(), quinn::ConnectionError> {
        while let Some(conn) = self.incoming.next().await {
            let new_conn = conn.await?;
            info!("Accepted connection from {}", new_conn.connection.remote_address());
            let (mut send, recv) = new_conn.bi_streams.next().await.unwrap()?;

            // TODO: handle connection
            let (mut send, mut recv) = new_conn.bi_streams.next().await.unwrap()?;
            let data = recv.read_to_end(usize::MAX).await?;
            info!("Received data: {:?}", data);
        }
        Ok(())
    }
}
