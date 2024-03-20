use std::sync::{atomic::AtomicU64, Arc, Mutex};

use tokio::net::TcpStream;

/// The RPC server
#[derive(Debug)]
#[allow(dead_code)]
pub struct RPCServer {
    /// The server ip
    server_ip: String,
    /// The server port
    server_port: u16,
    /// The server connections
    connections: Arc<Mutex<Vec<TcpStream>>>,
    /// connection count
    connection_count: Arc<AtomicU64>,
}

impl RPCServer {
    /// Create a new RPC server
    pub fn new(server_ip: String, server_port: u16) -> Self {
        Self {
            server_ip,
            server_port,
            connections: Arc::new(Mutex::new(Vec::new())),
            connection_count: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Start the RPC server
    pub async fn start(&self) -> anyhow::Result<()> {
        todo!()
    }

    /// Stop the RPC server
    pub async fn stop(&self) -> anyhow::Result<()> {
        todo!()
    }
}

impl Drop for RPCServer {
    fn drop(&mut self) {
        todo!()
    }
}