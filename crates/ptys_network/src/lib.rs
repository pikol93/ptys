use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, RwLock,
};

use eyre::{OptionExt, Result};
use listener::{Listener, ListenerState};
use ptys_common::extension::sender::SenderExt;
use tokio::{runtime::Runtime, sync::broadcast::Sender};

pub mod listener;
pub mod remote_stream;
pub mod sendable;

#[derive(Clone, Copy)]
pub struct EventListenerAdded {
    pub listener_id: usize,
}

#[derive(Clone, Copy)]
pub struct EventListenerRemoved {
    pub listener_id: usize,
}

#[derive(Clone, Copy)]
pub struct EventListenerStateChanged {
    pub listener_id: usize,
    pub state: ListenerState,
}

pub struct Network {
    pub event_listener_added: Sender<EventListenerAdded>,
    pub event_listener_removed: Sender<EventListenerRemoved>,
    pub event_listener_state_changed: Sender<EventListenerStateChanged>,
    runtime: Arc<Runtime>,
    listener_id_counter: AtomicUsize,
    listeners: RwLock<Vec<Listener>>,
}

impl Network {
    pub fn new(runtime: Arc<Runtime>) -> Self {
        Self {
            runtime,
            listener_id_counter: Default::default(),
            listeners: Default::default(),
            event_listener_added: Sender::new(16),
            event_listener_removed: Sender::new(16),
            event_listener_state_changed: Sender::new(16),
        }
    }

    pub fn add_listener(&self, port: u16) -> usize {
        let listener_id = self.listener_id_counter.fetch_add(1, Ordering::SeqCst);
        let runtime = self.runtime.clone();
        let mut listeners = self.listeners.write().unwrap();
        let listener = Listener::new(listener_id, port, runtime);
        self.register_listener_change_state(&listener);
        listeners.push(listener);

        let _ = self.event_listener_added.send(EventListenerAdded { listener_id });

        listener_id
    }

    pub fn remove_listener(&self, listener_id: usize) -> Result<()> {
        let mut listeners = self.listeners.write().unwrap();
        let index = listeners
            .iter()
            .enumerate()
            .filter(|(_, listener)| listener.id == listener_id)
            .next()
            .map(|(index, _)| index)
            .ok_or_eyre("Listener not found.")?;
        listeners.remove(index);

        let _ = self.event_listener_removed.send(EventListenerRemoved { listener_id });

        Ok(())
    }

    pub fn iter_listeners<T>(&self, func: impl FnOnce(&[Listener]) -> T) -> T {
        let listeners = self.listeners.read().unwrap();
        func(&listeners)
    }

    fn register_listener_change_state(&self, listener: &Listener) {
        let listener_id = listener.id;
        let listener_state_changed_sender = self.event_listener_state_changed.clone();
        listener
            .event_state_changed
            .event_subscribe(&self.runtime, move |state| {
                let _ = listener_state_changed_sender.send(EventListenerStateChanged { listener_id, state });
            });
    }
}
