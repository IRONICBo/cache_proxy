/// Cache proxy config
/// 
/// This struct is used to manage the cache proxy configuration.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Config {
    /// HashRing slot size
    pub slot_size: usize,
    /// Meta type
    pub meta_type: MetaType,
    /// Meta data endpoint
    pub meta_endpoints: Vec<String>,
    /// Time period to fetch meta data
    pub time_period: usize,

    /// RPC server ip
    pub rpc_ip: String,
    /// RPC server port
    pub rpc_port: u16,
}

/// Meta type
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum MetaType {
    /// ETCD
    ETCD,
    /// Redis
    Redis,
}

impl MetaType {
    /// Get the meta type
    pub fn get_meta_type(&self) -> String {
        match self {
            MetaType::ETCD => "etcd".to_string(),
            MetaType::Redis => "redis".to_string(),
        }
    }

    /// Get the meta type from string
    pub fn from_string(meta_type: &str) -> Self {
        match meta_type {
            "etcd" => MetaType::ETCD,
            "redis" => MetaType::Redis,
            _ => panic!("Invalid meta type"),
        }
    }
}

impl Config {
    /// Create a new config
    pub fn new(slot_size: usize, meta_type_string: &str, meta_endpoints: Vec<String>, time_period: usize,
            rpc_ip: String, rpc_port: u16) -> Self {
        let meta_type = MetaType::from_string(meta_type_string);

        Self {
            slot_size,
            meta_type,
            meta_endpoints,
            time_period,
            rpc_ip,
            rpc_port,
        }
    }

    /// Get the slot size
    pub fn slot_size(&self) -> usize {
        self.slot_size
    }

    /// Get the meta endpoint
    pub fn meta_endpoints(&self) -> &Vec<String> {
        &self.meta_endpoints
    }

    /// Get the time period
    pub fn time_period(&self) -> usize {
        self.time_period
    }
}