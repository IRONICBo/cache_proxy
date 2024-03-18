use std::{collections::HashMap, sync::Arc};

/// Slot
/// 
/// This struct is used to represent the slot in the hashring.
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Slot {
    /// The id of the slot
    id: u64,
    /// Backend node id
    backend_node_id: u64,
    /// The slot is migrating or not
    is_migrating: bool,
}

impl Slot {
    /// Create a new slot
    pub fn new(id: u64, backend_node_id: u64) -> Self {
        Self {
            id,
            backend_node_id,
            is_migrating: false,
        }
    }

    /// Get the id of the slot
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get the backend node id
    pub fn backend_node_id(&self) -> u64 {
        self.backend_node_id
    }

    /// Get the is_migrating
    pub fn is_migrating(&self) -> bool {
        self.is_migrating
    }
}

/// Slot mapping
/// 
/// This struct is used to manage the slot mapping.
#[derive(Debug)]
pub struct SlotMapping {
    /// The slot mapping
    inner: Arc<HashMap<u64, Slot>>,
}

impl SlotMapping {
    /// Create a new slot mapping
    pub fn new() -> Self {
        Self {
            inner: Arc::new(HashMap::new()),
        }
    }

    /// Get the slot mapping
    pub fn inner(&self) -> Arc<HashMap<u64, Slot>> {
        self.inner.clone()
    }

    /// Set the slot mapping
    pub fn set_inner(&mut self, inner: HashMap<u64, Slot>) {
        self.inner = Arc::new(inner);
    }
}