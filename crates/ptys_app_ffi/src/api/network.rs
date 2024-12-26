use std::sync::Arc;

use eyre::Result;
use flutter_rust_bridge::DartFnFuture;
use ptys_service::network_ext::NetworkExt;

use crate::service::{get_runtime, get_service};

pub enum ListenerState {
    Disabled,
    Listening,
}

impl From<ptys_network::listener::ListenerState> for ListenerState {
    fn from(value: ptys_network::listener::ListenerState) -> Self {
        match value {
            ptys_network::listener::ListenerState::Disabled => Self::Disabled,
            ptys_network::listener::ListenerState::Listening => Self::Listening,
        }
    }
}

pub struct Listener {
    pub id: i64,
    pub port: u16,
    pub state: ListenerState,
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

pub async fn add_listener(port: u16) -> i64 {
    get_service().network.add_listener(port).await as i64
}

pub async fn remove_listener(id: i64) -> Result<(), String> {
    get_service()
        .network
        .remove_listener(id as usize)
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

pub async fn subscribe_listener_removed(
    dart_callback: impl Fn(i64) -> DartFnFuture<()> + Send + Sync + 'static,
) {
    let arc_callback = Arc::new(dart_callback);
    let callback = move |value| {
        let dart_callback = arc_callback.clone();
        get_runtime().spawn(async move { dart_callback(value as i64).await });
    };

    get_service().subscribe_listener_removed(callback)
}

pub async fn get_listeners() -> Vec<Listener> {
    get_service()
        .network
        .iter_listeners(|listeners| {
            listeners
                .into_iter()
                .map(|listener| Listener {
                    id: listener.id as i64,
                    port: listener.port,
                    state: listener.get_state().into(),
                })
                .collect()
        })
        .await
}

pub async fn start_listener(id: i64) -> Result<(), String> {
    let id = id as usize;
    get_service()
        .network
        .iter_listeners(|listeners| {
            listeners
                .iter()
                .filter(|listener| listener.id == id)
                .next()
                .map(Clone::clone)
        })
        .await
        .ok_or("Could not find listener.")?
        .start()
        .await
        .map_err(|err| format!("{}", err))
}

pub async fn stop_listener(id: i64) -> Result<(), String> {
    let id = id as usize;
    get_service()
        .network
        .iter_listeners(|listeners| {
            listeners
                .iter()
                .filter(|listener| listener.id == id)
                .next()
                .map(Clone::clone)
        })
        .await
        .ok_or("Could not find listener.")?
        .stop()
        .await
        .map_err(|err| format!("{}", err))
}
