use std::{sync::{Arc, Mutex}};

/// Default slot size
const SLOT_SIZE: u64 = 1024;

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

    /// Set the is_migrating
    pub fn set_migrating(&mut self, is_migrating: bool) {
        self.is_migrating = is_migrating;
    }

    /// Set the backend node id
    pub fn set_backend_node_id(&mut self, backend_node_id: u64) {
        self.backend_node_id = backend_node_id;
    }
}

/// Slot mapping
/// 
/// This struct is used to manage the slot mapping.
#[derive(Debug)]
pub struct SlotMapping {
    /// The slot mapping
    inner: Arc<Mutex<Vec<Slot>>>,
}

impl SlotMapping {
    /// Create a new slot mapping
    pub fn default() -> Self {
        // Create a slots mapping with SLOT_SIZE
        let slots = (0..SLOT_SIZE)
            .map(|id| Slot::new(id, 0))
            .collect::<Vec<Slot>>();

        // Try to load mapping from meta data
        // TODO: load from meta data

        Self {
            inner: Arc::new(Mutex::new(slots)),
        }
    }

    /// Get the slot mapping
    pub fn inner(&self) -> Vec<Slot> {
        self.inner.lock().unwrap().clone()
    }

    /// Get the slot by id
    pub fn get_slot(&self, id: u64) -> Option<Slot> {
        let slots = self.inner.lock().unwrap();
        slots.get(id as usize).map(|slot| slot.clone())
    }

    /// Get the available slot
    pub fn available_slot(&self) -> Vec<Slot> {
        let slots = self.inner.lock().unwrap();
        slots.iter().filter(|slot| !slot.is_migrating()).cloned().collect()
    }
}