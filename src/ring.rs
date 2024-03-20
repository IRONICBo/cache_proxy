use std::sync::{Arc, Mutex};
use std::fmt::Debug;

use hashring::DefaultHashBuilder;

use crate::slot::Slot;

/// HashRing
/// 
/// This struct is used to manage the hashring.
pub struct HashRing {
    /// The version of the hashring
    version: u64,
    /// The hashring data
    inner: Arc<Mutex<hashring::HashRing<Slot, DefaultHashBuilder>>>,
}

impl Debug for HashRing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HashRing {{ version: {} }}", self.version)
    }
}

impl HashRing {
    /// Create a new hashring
    pub fn new(slots: Vec<Slot>) -> Self {
        // Init hashring with slots
        let mut ring = hashring::HashRing::<Slot>::new();
        ring.batch_add(slots);

        Self {
            version: 0,
            inner: Arc::new(Mutex::new(ring)),
        }
    }

    /// Get the version of the hashring
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Get the slot by key
    pub fn get_slot(&self, key: &str) -> Option<Slot> {
        // Lock the hashring
        let inner = self.inner.lock().unwrap();

        // Get the slot by key
        inner.get(&key).map(|node| node.clone())
    }

    
}