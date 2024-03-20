use super::{RPCRequest, RPCResponse};

/// RPC client module
#[derive(Debug)]
#[allow(dead_code)]
pub struct RPCClient {
    server_ip: String,
    server_port: u16,
    timeout: u64,
    close: bool,
}

impl RPCClient {
    /// Create a new RPC client
    pub fn new(server_ip: String, server_port: u16, timeout: u64) -> Self {
        Self {
            server_ip,
            server_port,
            timeout,
            close: false,
        }
    }

    /// Send a request to the server
    pub async fn send_request(&self, _request: RPCRequest) -> anyhow::Result<RPCResponse> {
        todo!()
    }
}