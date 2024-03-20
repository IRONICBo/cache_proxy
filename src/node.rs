use std::sync::{Arc, Mutex};

/// Physical node struct
/// 
/// physical node is the node in the slot mapping
#[derive(Debug, Clone)]
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

/// Node list
/// 
/// Node list is used to manage the physical nodes
#[derive(Debug)]
pub struct NodeList {
    inner: Arc<Mutex<Vec<Node>>>,
}

impl NodeList {
    /// Create a new node list
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Add a node to the list
    pub fn add(&self, node: Node) {
        self.inner.lock().unwrap().push(node);
    }

    /// Get the node list
    pub fn list(&self) -> Vec<Node> {
        self.inner.lock().unwrap().clone()
    }

    /// Remove a node from the list
    pub fn remove(&self, id: u64) {
        let mut list = self.inner.lock().unwrap();
        list.retain(|node| node.id() != id);
    }

    /// Get the node by id
    pub fn get(&self, id: u64) -> Option<Node> {
        let list = self.inner.lock().unwrap();
        list.iter().find(|node| node.id() == id).cloned()
    }
}