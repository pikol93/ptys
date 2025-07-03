use std::{ops::Deref, sync::Arc};

use eyre::Result;
use flutter_rust_bridge::DartFnFuture;
use ptys_common::extension::sender::SenderExt;
use ptys_network::{
    listener::ListenerId, EventListenerAdded, EventListenerRemoved, EventListenerStateChanged,
};

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
    get_service().network.add_listener(port) as i64
}

pub async fn remove_listener(id: i64) -> Result<(), String> {
    get_service()
        .network
        .remove_listener(id as usize)
        .map_err(|err| format!("{}", err))
}

pub fn subscribe_listener_added(
    dart_callback: impl Fn(i64) -> DartFnFuture<()> + Send + Sync + 'static,
) {
    let arc_callback = Arc::new(dart_callback);
    let callback = move |event: EventListenerAdded| {
        let dart_callback = arc_callback.clone();
        get_runtime().spawn(async move { dart_callback(event.listener_id as i64).await });
    };

    get_service()
        .network
        .event_listener_added
        .event_subscribe(get_runtime().deref(), callback);
}

pub fn subscribe_listener_removed(
    dart_callback: impl Fn(i64) -> DartFnFuture<()> + Send + Sync + 'static,
) {
    let arc_callback = Arc::new(dart_callback);
    let callback = move |event: EventListenerRemoved| {
        let dart_callback = arc_callback.clone();
        get_runtime().spawn(async move { dart_callback(event.listener_id as i64).await });
    };

    get_service()
        .network
        .event_listener_removed
        .event_subscribe(get_runtime().deref(), callback);
}

pub fn subscribe_listener_changed_state(
    dart_callback: impl Fn(i64, ListenerState) -> DartFnFuture<()> + Send + Sync + 'static,
) {
    let callback = {
        let dart_callback = Arc::new(dart_callback);
        move |event: EventListenerStateChanged| {
            let dart_callback = dart_callback.clone();
            get_runtime().spawn(async move {
                dart_callback(event.listener_id as i64, event.state.into()).await
            });
        }
    };

    get_service()
        .network
        .event_listener_state_changed
        .event_subscribe(get_runtime().deref(), callback);
}

pub async fn get_listeners() -> Vec<Listener> {
    get_service().network.iter_listeners(|listeners| {
        listeners
            .into_iter()
            .map(|listener| Listener {
                id: listener.id as i64,
                port: listener.port,
                state: listener.get_state().into(),
            })
            .collect()
    })
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
        .ok_or("Could not find listener.")?
        .start()
        .await
        .map_err(|err| format!("{}", err))
}

pub async fn stop_listener(id: i64) -> Result<(), String> {
    let target_id = ListenerId(id as usize);
    get_service()
        .network
        .listeners
        .iter_listeners(|listeners| {
            listeners
                .iter()
                .filter(|(id, _)| *id == target_id)
                .next()
                .map(|(_, listener)| listener)
                .map(Clone::clone)
        })
        .ok_or("Could not find listener.")?
        .stop()
        .await
        .map_err(|err| format!("{}", err))
}
