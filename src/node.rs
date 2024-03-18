/// Physical node struct
/// 
/// physical node is the node in the slot mapping
#[derive(Debug)]
pub struct Node {
    /// The id of the node
    id: u64,
    /// The ip of the node
    ip: String,
    /// The port of the node
    port: u16,
    /// The weight of the node
    weight: u32,
}

impl Node {
    /// Create a new node
    pub fn new(id: u64, ip: String, port: u16, weight: u32) -> Self {
        Self {
            id,
            ip,
            port,
            weight,
        }
    }

    /// Get the id of the node
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get the ip of the node
    pub fn ip(&self) -> &str {
        &self.ip
    }

    /// Get the port of the node
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Get the weight of the node
    pub fn weight(&self) -> u32 {
        self.weight
    }
}