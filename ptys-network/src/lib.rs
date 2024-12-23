use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use eyre::{OptionExt, Result};
use listener::Listener;
use tokio::{runtime::Runtime, sync::RwLock};

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
}

impl Network {
    pub fn new(runtime: Arc<Runtime>) -> Self {
        Self {
            inner: Inner {
                runtime,
                listener_id_counter: Default::default(),
                listeners: Default::default(),
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

        id
    }

    pub async fn start_listener(&self, id: usize) -> Result<()> {
        let listeners = self.inner.listeners.read().await;
        let listener = listeners
            .iter()
            .filter(|listener| listener.id == id)
            .next()
            .ok_or_eyre("No listener found")?;

        listener.start().await?;

        Ok(())
    }
}
