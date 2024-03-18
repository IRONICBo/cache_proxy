use std::{fmt::Debug, sync::Arc};

use crate::{client::MetaClient, config::Config, node::Node, ring::HashRing, slot::SlotMapping};

/// Cache proxy manager
///
/// This manager is used to manage the cache proxy topology.
#[derive(Debug)]
pub struct CacheProxyManager<C>
where
    C: MetaClient,
{
    /// The cache proxy topology
    inner: ProxyTopology,
    /// config
    config: Config,
    /// Meta data client
    client: C,
}

impl <C> CacheProxyManager<C>
where
    C: MetaClient,
{
    /// Create a new cache proxy manager
    pub fn new(config: Config, client: C) -> Self {
        let slot_size = config.slot_size();
        let inner = ProxyTopology::new(slot_size);
        Self {
            inner,
            config,
            client,
        }
    }

    /// Get the cache proxy topology
    pub fn inner(&self) -> &ProxyTopology {
        &self.inner
    }

    /// Get the config
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get the meta data client
    pub fn client(&self) -> &C {
        &self.client
    }
}

/// Proxy topology
/// 
/// This struct is used to manage the inner topology in memory cache.
pub struct ProxyTopology {
    /// Proxy topology for hashring
    /// TODO: update to node list?
    hash_ring: HashRing,
    /// Mapping from slot to physical node
    slot_mapping: SlotMapping,
    /// Node list
    nodes: Arc<Vec<Node>>,
    /// Slot size
    slot_size: usize,
}

impl ProxyTopology {
    /// Create a new proxy topology
    pub fn new(slot_size: usize) -> Self {
        Self {
            hash_ring: HashRing::new(),
            slot_mapping: SlotMapping::new(),
            nodes: Arc::new(Vec::new()),
            slot_size,
        }
    }

    /// Get the hash ring
    pub fn hash_ring(&self) -> &HashRing {
        &self.hash_ring
    }

    /// Get the slot mapping
    pub fn slot_mapping(&self) -> &SlotMapping {
        &self.slot_mapping
    }

    /// Get the nodes
    pub fn nodes(&self) -> Arc<Vec<Node>> {
        self.nodes.clone()
    }

    /// Get the slot size
    pub fn slot_size(&self) -> usize {
        self.slot_size
    }
}

impl Debug for ProxyTopology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProxyTopology")
            .field("slot_mapping", &self.slot_mapping)
            .field("nodes", &self.nodes)
            .field("slot_size", &self.slot_size)
            .finish()
    }
}