use std::sync::Arc;

use eyre::{eyre, Result};
use tokio::select;
use tokio::sync::broadcast::Sender;
use tokio::sync::RwLock;
use tokio::{net::TcpListener, runtime::Runtime};
use tokio_util::sync::{CancellationToken, DropGuard};

use crate::channel::tcp_client::TcpClientChannel;

#[derive(Clone)]
pub struct EventClientAccepted {
    pub channel: Arc<TcpClientChannel>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ListenerState {
    Disabled,
    Listening,
}

#[derive(Clone)]
pub struct Listener {
    pub port: u16,
    pub event_state_changed: Sender<ListenerState>,
    pub event_client_accepted: Sender<EventClientAccepted>,
    runtime: Arc<Runtime>,
    inner: Arc<Inner>,
}

struct Inner {
    state: std::sync::RwLock<ListenerState>,
    cancellation_token: RwLock<Option<DropGuard>>,
}

impl Listener {
    pub fn new(port: u16, runtime: Arc<Runtime>) -> Listener {
        Listener {
            port,
            event_state_changed: Sender::new(16),
            event_client_accepted: Sender::new(16),
            runtime,
            inner: Arc::new(Inner {
                state: std::sync::RwLock::new(ListenerState::Disabled),
                cancellation_token: RwLock::default(),
            }),
        }
    }

    pub async fn start(&self) -> Result<()> {
        let mut cancellation_token = self.inner.cancellation_token.write().await;
        if cancellation_token.is_some() {
            return Err(eyre!("Listener already started."));
        }

        let listener = TcpListener::bind(("0.0.0.0", self.port)).await?;

        let new_cancellation_token = CancellationToken::new();
        let child_cancellation_token = new_cancellation_token.child_token();
        *cancellation_token = Some(new_cancellation_token.drop_guard());

        *self.inner.state.write().unwrap() = ListenerState::Listening;
        let inner = Arc::<Inner>::downgrade(&self.inner);
        let event_state_changed = self.event_state_changed.clone();
        let event_client_accepted = self.event_client_accepted.clone();
        let _ = event_state_changed.send(ListenerState::Listening);

        self.runtime.spawn(async move {
            run_listener(listener, child_cancellation_token, event_client_accepted).await;

            let Some(inner) = inner.upgrade() else {
                println!("Inner dropped.");
                return;
            };

            *inner.cancellation_token.write().await = None;
            *inner.state.write().unwrap() = ListenerState::Disabled;
            let _ = event_state_changed.send(ListenerState::Disabled);
        });

        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        let mut cancellation_token = self.inner.cancellation_token.write().await;
        if cancellation_token.is_none() {
            return Err(eyre!("Listener already cancelled."));
        }

        *cancellation_token = None;
        Ok(())
    }

    pub fn get_state(&self) -> ListenerState {
        *self.inner.state.read().unwrap()
    }
}

async fn run_listener(
    listener: TcpListener,
    cancellation_token: CancellationToken,
    event_client_accepted: Sender<EventClientAccepted>,
) {
    loop {
        select! {
            result = listener.accept() => {
                let Ok((stream, address)) = result else {
                    continue;
                };

                println!("Client accepted: {}", address);

                let channel = Arc::new(TcpClientChannel::new(address, stream));
                let _ = event_client_accepted.send(EventClientAccepted { channel });
            },
            _ = cancellation_token.cancelled() => {
                println!("Cancellation token cancelled.");
                break;
            }
        }
    }

    println!("Listener stopped.");
}
