use std::{fmt::Debug, usize};

use anyhow::Ok;
use tokio::{select, time};

use crate::{client::{self, MetaClient}, config::Config, node::NodeList, ring::HashRing, rpc::server::RPCServer, slot::SlotMapping};

use tracing::warn;

/// Cache proxy manager
///
/// This manager is used to manage the cache proxy topology.
#[derive(Debug)]
#[allow(dead_code)]
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
    /// RPC Server
    rpc_server: RPCServer,
}

impl <C> CacheProxyManager<C>
where
    C: MetaClient,
{
    /// Create a new cache proxy manager
    pub fn new(config: Config) -> Self {
        let inner = ProxyTopology::new(config.clone());
        let rpc_server = RPCServer::new(config.clone().rpc_ip, config.clone().rpc_port);
        let client = client::new_meta_client(config.clone().meta_endpoints);

        Self {
            inner,
            config,
            client,
            rpc_server,
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

    /// Get free slot
    #[allow(dead_code)]
    pub fn allocate_free_slot(&self) -> anyhow::Result<()> {
        // Get available slot mapping and node list
        let slot_mapping = self.inner.slot_mapping();

        // Get available slot
        let _available_slot = slot_mapping.available_slot();

        // Update slot mapping
        // slot_mapping.update_slot(available_slot);
        // self.inner.update_slot_mapping();

        Ok(())
    }

    /// Start
    pub async fn start(&self) -> anyhow::Result<()> {
        // self.rpc_server.start().await?;

        // Fetch metadata from meta client
        // match self.client.read("/", true) {
        //     Ok(data) => {
        //         // Update metadata client
        //         info!("Update metadata from meta client success");
                
        //         // Mark current node as online
        //         self.register_node();
                
        //         // TODO: convert data to slot mapping and node list

        //         // init topology
        //         self.inner.init(data.slot_mapping, data.node_list);

        //         // Get available slot mapping and node list
        //         self.inner.allocate_free_slot();

        //         // Update to slotmapping to meta client
        //         self.client.update("/", data);
        //     }
        //     Err(e) => {
        //         warn!("Update metadata from meta client failed: {:?}", e);
        //     }
        // }

        // Start timer worker to fetch metadata
        let mut metadata_interval = time::interval(time::Duration::from_secs(self.inner.time_period as u64));
        loop {
            select! {
                _ = metadata_interval.tick() => {
                    // Update metadata from meta client
                    self.update_metadata().await?;
                },
                _ = self.normal_worker() => {

                },
            }
        }
    }

    /// Rebalancing
    #[allow(dead_code)]
    pub async fn rebalancing(&self) -> anyhow::Result<()> {
        // Request balancing lock
        // let lock = self.client.lock("/");

        // Get available slot mapping and node list
        let slot_mapping = self.inner.slot_mapping();
        let _node_list = self.inner.nodes();

        // Get available slot
        let _available_slot = slot_mapping.available_slot();

        // Update slot mapping
        // slot_mapping.update_slot(available_slot);
        // self.inner.update_slot_mapping();

        // TODO: rebalancing slot mapping

        // Update to slotmapping to meta client
        // self.client.update("/", data);

        Ok(())
    }

    /// Current node online
    #[allow(dead_code)]
    fn register_node(&self) -> anyhow::Result<()> {
        // update current node info to meta client
        let _data = b"127.0.0.1"; // Mock data

        // Register node to meta client
        // match self.client.create("/", data) {
        //     Ok(_) => {
        //         // Update metadata client
        //         info!("Update metadata from meta client success");
                
        //         // self.inner.init(slot_mapping, node_list);
        //     }
        //     Err(e) => {
        //         warn!("Update metadata from meta client failed: {:?}", e);
        //     }
        // }

        Ok(())
    }

    /// Current node offline
    #[allow(dead_code)]
    fn unregister_node(&self) -> anyhow::Result<()> {
        // update current node info to meta client
        // match self.client.delete("/") {
        //     Ok(_) => {
        //         // Update metadata client
        //         info!("Update metadata from meta client success");
                
        //         // self.inner.init(slot_mapping, node_list);
        //     }
        //     Err(e) => {
        //         warn!("Update metadata from meta client failed: {:?}", e);
        //     }
        // }

        Err(anyhow::anyhow!("Unregister node failed"))
    }

    async fn update_metadata(&self) -> anyhow::Result<()> {
        // Fetch metadata from meta client
        // match self.client.read("/", true) {
        //     Ok(_data) => {
        //         // Update metadata client
        //         info!("Update metadata from meta client success");
                
        //         // TODO: convert data to slot mapping and node list
        //         // self.inner.update_slot_mapping();
        //         // self.inner.update_node_list();
        //     }
        //     Err(e) => {
        //         warn!("Update metadata from meta client failed: {:?}", e);
        //     }
        // }

        warn!("Update metadata from meta client failed");
        Ok(())
    }

    async fn normal_worker(&self) -> anyhow::Result<()> {
        // serve rpc request

        Ok(())
    }
}

/// Proxy topology
/// 
/// This struct is used to manage the inner topology in memory cache.
#[allow(dead_code)]
pub struct ProxyTopology {
    /// Proxy topology for hashring
    /// TODO: update to node list?
    hash_ring: HashRing,
    /// Mapping from slot to physical node
    slot_mapping: SlotMapping,
    /// Node list
    node_list: NodeList,
    /// Slot size
    slot_size: usize,
    /// time period
    time_period: usize,
}

impl ProxyTopology {
    /// Create a new proxy topology
    pub fn new(config: Config) -> Self {
        let slot_mapping = SlotMapping::default();
        let hash_ring = HashRing::new(slot_mapping.inner());
        let node_list = NodeList::new();
        let slot_size = config.slot_size();
        let time_period = config.time_period();

        Self {
            hash_ring,
            slot_mapping,
            node_list,
            slot_size,
            time_period,
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
    pub fn nodes(&self) -> &NodeList {
        &self.node_list
    }

    /// Get the slot size
    pub fn slot_size(&self) -> usize {
        self.slot_size
    }

    /// Update slotmapping
    pub fn update_slot_mapping(&mut self, slot_mapping: SlotMapping) {
        self.slot_mapping = slot_mapping;
        self.hash_ring = HashRing::new(self.slot_mapping.inner());
    }

    /// Update online node list
    pub fn update_node_list(&mut self, node_list: NodeList) {
        self.node_list = node_list;
    }

    /// Start the manager
    pub fn init(&mut self, slot_mapping: SlotMapping, node_list: NodeList) -> anyhow::Result<()> {
        // Update slot mapping and node list
        self.update_slot_mapping(slot_mapping);
        self.update_node_list(node_list);

        Ok(())
    }
    
}

impl Debug for ProxyTopology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProxyTopology")
            .field("slot_mapping", &self.slot_mapping)
            .field("nodes", &self.node_list)
            .field("slot_size", &self.slot_size)
            .finish()
    }
}