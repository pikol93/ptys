use std::sync::Arc;

use ptys_network::Network;
use tokio::runtime::Runtime;

#[derive(Clone)]
pub struct Service {
    pub(crate) runtime: Arc<Runtime>,
    pub network: Arc<Network>,
}

impl Service {
    pub fn new(runtime: Arc<Runtime>) -> Self {
        Self {
            runtime: runtime.clone(),
            network: Arc::new(Network::new(runtime)),
        }
    }
}
