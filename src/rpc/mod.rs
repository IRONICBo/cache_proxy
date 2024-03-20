/// This module contains the RPC client and server implementations.
/// 
/// 1. Support basic RPC request and response
/// 2. Support file chunk transfer

/// The RPC client
pub mod client;

/// The RPC server
pub mod server;

/// The RPC request
#[derive(Debug, Clone)]
pub struct RPCRequest {
    /// The request id
    pub id: u64,
    /// The request header, contains the version and type
    pub header: u64,
    /// The request body
    pub body: Option<Vec<u8>>,
}

/// The RPC response
#[derive(Debug, Clone)]
pub struct RPCResponse {
    /// The response id
    pub id: u64,
    /// The request header, contains the version and type
    pub header: u64,
    /// The response msg
    pub msg: Option<Vec<u8>>,
    /// The request body
    pub body: Option<Vec<u8>>,
}