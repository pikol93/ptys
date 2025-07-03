use std::{
    ops::Deref,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
};

use eyre::{OptionExt, Result};
use ptys_common::extension::sender::SenderExt;
use tokio::{runtime::Runtime, sync::broadcast::Sender};

use crate::channel::tcp_client::TcpClientChannel;

use super::{
    item::{Listener, ListenerState},
    ListenerId,
};

#[derive(Clone, Copy)]
pub struct EventListenerAdded {
    pub id: ListenerId,
}

#[derive(Clone, Copy)]
pub struct EventListenerRemoved {
    pub id: ListenerId,
}

#[derive(Clone, Copy)]
pub struct EventListenerStateChanged {
    pub id: ListenerId,
    pub state: ListenerState,
}

#[derive(Clone)]
pub struct EventClientAccepted {
    pub id: ListenerId,
    pub channel: Arc<TcpClientChannel>,
}

pub struct ListenersContainer {
    pub event_listener_added: Sender<EventListenerAdded>,
    pub event_listener_removed: Sender<EventListenerRemoved>,
    pub event_listener_state_changed: Sender<EventListenerStateChanged>,
    pub event_client_accepted: Sender<EventClientAccepted>,
    listeners: RwLock<Vec<(ListenerId, Arc<Listener>)>>,
    listener_id_counter: AtomicUsize,
    runtime: Arc<Runtime>,
}

impl ListenersContainer {
    pub fn new(runtime: Arc<Runtime>) -> Self {
        Self {
            event_listener_added: Sender::new(16),
            event_listener_removed: Sender::new(16),
            event_listener_state_changed: Sender::new(16),
            event_client_accepted: Sender::new(16),
            listeners: Default::default(),
            listener_id_counter: Default::default(),
            runtime,
        }
    }

    pub fn add_listener(&self, port: u16) -> Result<ListenerId> {
        let id = self.listener_id_counter.fetch_add(1, Ordering::SeqCst);
        let id = ListenerId(id);
        let runtime = self.runtime.clone();
        let mut listeners = self.listeners.write().unwrap();
        let listener = Listener::new(port, runtime);
        self.register_listener_events(&listener, id);
        listeners.push((id, Arc::new(listener)));

        let _ = self.event_listener_added.send(EventListenerAdded { id });

        Ok(id)
    }

    pub fn remove_listener(&self, target_id: ListenerId) -> Result<()> {
        let mut listeners = self.listeners.write().unwrap();
        let index = listeners
            .iter()
            .enumerate()
            .filter(|(_, (id, _))| *id == target_id)
            .next()
            .map(|(index, _)| index)
            .ok_or_eyre("Listener not found.")?;
        listeners.remove(index);

        let _ = self
            .event_listener_removed
            .send(EventListenerRemoved { id: target_id });

        Ok(())
    }

    pub fn get_listener(&self, target_id: ListenerId) -> Option<Arc<Listener>> {
        self.listeners
            .read()
            .unwrap()
            .iter()
            .filter(|(id, _)| *id == target_id)
            .next()
            .map(|(_, listener)| listener.clone())
    }

    pub fn iter_listeners<T>(&self, func: impl FnOnce(&[(ListenerId, Listener)]) -> T) -> T {
        let listeners = self.listeners.read().unwrap();
        func(&listeners)
    }

    fn register_listener_events(&self, listener: &Listener, id: ListenerId) {
        listener
            .event_state_changed
            .event_subscribe(self.runtime.deref(), {
                let event_listener_state_changed = self.event_listener_state_changed.clone();
                move |state| {
                    let _ =
                        event_listener_state_changed.send(EventListenerStateChanged { id, state });
                }
            });

        listener
            .event_client_accepted
            .event_subscribe(self.runtime.deref(), {
                let event_client_accepted = self.event_client_accepted.clone();
                move |super::item::EventClientAccepted { channel }| {
                    let _ = event_client_accepted.send(EventClientAccepted { id, channel });
                }
            });
    }
}
