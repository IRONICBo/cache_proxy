use anyhow::Error;

/// Meta data client trait.
/// 
/// This trait is used to interact with meta data service.
/// Probably it's a etcd server
pub trait MetaClient {
    /// Create a new meta data
    fn create(&self, path: &str, data: &[u8]) -> Result<(), Error>;

    /// Update the meta data
    fn update(&self, path: &str, data: &[u8]) -> Result<(), Error>;

    /// Delete the meta data
    fn delete(&self, path: &str) -> Result<(), Error>;

    /// Read the meta data
    fn read(&self, path: &str, must: bool) -> Result<Vec<u8>, Error>;

    /// List the meta data
    fn list(&self, path: &str, must: bool) -> Result<Vec<String>, Error>;

    /// Close the meta data client
    fn close(&self) -> Result<(), Error>;

    // Watch the meta data?
    // fn watch(&self, path: &str) -> Result<W, Error>;
}

/// Metadata watcher trait
/// 
/// This trait is used to watch the meta data change.(TODO)
pub trait Watcher {
    // TODO: Support watch event?
}

/// Create a new meta data client
pub fn new_meta_client<C: MetaClient>(endpoints: Vec<String>) -> C {
    let _ = endpoints;
    unimplemented!()
}