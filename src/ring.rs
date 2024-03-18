use std::sync::Arc;
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
    inner: Arc<hashring::HashRing<Slot, DefaultHashBuilder>>,
}

impl Debug for HashRing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HashRing {{ version: {} }}", self.version)
    }
}

impl HashRing {
    /// Create a new hashring
    pub fn new() -> Self {
        Self {
            version: 0,
            inner: Arc::new(hashring::HashRing::<Slot, DefaultHashBuilder>::new()),
        }
    }

    /// Get the version of the hashring
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Get the inner hashring
    pub fn inner(&self) -> Arc<hashring::HashRing<Slot, DefaultHashBuilder>> {
        self.inner.clone()
    }
}