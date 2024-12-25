use ptys_service::network_ext::NetworkExt;

use crate::service::get_service;

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

pub async fn add_listener(port: u16) -> i64 {
    get_service().add_listener(port).await as i64
}

pub async fn start_listener(id: i64) -> Result<(), String> {
    get_service()
        .start_listener(id as usize)
        .await
        .map_err(|err| format!("{}", err))
}
