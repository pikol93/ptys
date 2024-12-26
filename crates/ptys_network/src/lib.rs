use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use listener::Listener;
use tokio::{
    runtime::Runtime,
    sync::{
        broadcast::{channel, Receiver, Sender},
        RwLock,
    },
};

pub mod listener;
pub mod remote_stream;
pub mod sendable;

pub struct Network {
    inner: Inner,
}

pub struct Inner {
    runtime: Arc<Runtime>,
    listener_id_counter: AtomicUsize,
    listeners: RwLock<Vec<Listener>>,
    listener_added_sender: Sender<usize>,
}

impl Network {
    pub fn new(runtime: Arc<Runtime>) -> Self {
        Self {
            inner: Inner {
                runtime,
                listener_id_counter: Default::default(),
                listeners: Default::default(),
                listener_added_sender: channel(1).0,
            },
        }
    }

    pub async fn add_listener(&self, port: u16) -> usize {
        let id = self
            .inner
            .listener_id_counter
            .fetch_add(1, Ordering::SeqCst);
        let runtime = self.inner.runtime.clone();
        let mut listeners = self.inner.listeners.write().await;
        listeners.push(Listener::new(id, port, runtime));

        let _ = self.inner.listener_added_sender.send(id);

        id
    }

    pub async fn iter_listeners<T>(&self, func: impl FnOnce(&[Listener]) -> T) -> T {
        let listeners = self.inner.listeners.read().await;
        func(&listeners)
    }

    pub fn get_listener_added_receiver(&self) -> Receiver<usize> {
        self.inner.listener_added_sender.subscribe()
    }
}
