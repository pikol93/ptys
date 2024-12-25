use std::sync::Arc;

use flutter_rust_bridge::DartFnFuture;
use ptys_service::network_ext::NetworkExt;

use crate::service::{get_runtime, get_service};

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

pub async fn subscribe_listener_added(
    dart_callback: impl Fn(i64) -> DartFnFuture<()> + Send + Sync + 'static,
) {
    let arc_callback = Arc::new(dart_callback);
    let callback = move |value| {
        let dart_callback = arc_callback.clone();
        get_runtime().spawn(async move { dart_callback(value as i64).await });
    };

    get_service().subscribe_listener_added(callback)
}
