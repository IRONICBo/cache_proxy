/// Cache proxy config
/// 
/// This struct is used to manage the cache proxy configuration.
#[derive(Debug)]
pub struct Config {
    /// HashRing slot size
    slot_size: usize,
    /// Meta data endpoint
    meta_endpoint: String,
}

impl Config {
    /// Create a new config
    pub fn new(slot_size: usize, meta_endpoint: String) -> Self {
        Self {
            slot_size,
            meta_endpoint,
        }
    }

    /// Get the slot size
    pub fn slot_size(&self) -> usize {
        self.slot_size
    }

    /// Get the meta endpoint
    pub fn meta_endpoint(&self) -> &str {
        &self.meta_endpoint
    }
}