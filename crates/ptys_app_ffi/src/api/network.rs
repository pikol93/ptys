use std::sync::Arc;

use ptys_network::Network;
use tokio::runtime::Runtime;

pub struct Program {
    network: Network,
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

#[flutter_rust_bridge::frb(sync)]
pub fn initialize_program() -> Arc<Program> {
    let runtime = Arc::new(Runtime::new().unwrap());

    Arc::new(Program {
        network: Network::new(runtime),
    })
}
