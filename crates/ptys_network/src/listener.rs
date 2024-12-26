use std::sync::atomic::Ordering;
use std::sync::Arc;

use atomic_enum::atomic_enum;
use eyre::{eyre, Result};
use tokio::io::AsyncWriteExt;
use tokio::select;
use tokio::sync::RwLock;
use tokio::{net::TcpListener, runtime::Runtime};
use tokio_util::sync::{CancellationToken, DropGuard};

#[atomic_enum]
pub enum ListenerState {
    Disabled,
    Listening,
}

#[derive(Clone)]
pub struct Listener {
    pub id: usize,
    pub port: u16,
    runtime: Arc<Runtime>,
    inner: Arc<Inner>,
}

struct Inner {
    state: AtomicListenerState,
    cancellation_token: RwLock<Option<DropGuard>>,
}

impl Listener {
    pub fn new(id: usize, port: u16, runtime: Arc<Runtime>) -> Listener {
        Listener {
            id,
            port,
            runtime,
            inner: Arc::new(Inner {
                state: AtomicListenerState::new(ListenerState::Disabled),
                cancellation_token: RwLock::default(),
            }),
        }
    }

    pub async fn start(&self) -> Result<()> {
        let inner = Arc::<Inner>::downgrade(&self.inner);
        let mut cancellation_token = self.inner.cancellation_token.write().await;
        if cancellation_token.is_some() {
            return Err(eyre!("Listener already started."));
        }

        let new_cancellation_token = CancellationToken::new();
        let child_cancellation_token = new_cancellation_token.child_token();
        *cancellation_token = Some(new_cancellation_token.drop_guard());

        drop(cancellation_token);

        let listener = TcpListener::bind(("0.0.0.0", self.port)).await?;

        self.inner
            .state
            .store(ListenerState::Listening, Ordering::Relaxed);

        self.runtime.spawn(async move {
            run_listener(listener, child_cancellation_token).await;

            let Some(inner) = inner.upgrade() else {
                println!("Inner dropped.");
                return;
            };
            let mut cancellation_token = inner.cancellation_token.write().await;
            *cancellation_token = None;

            inner
                .state
                .store(ListenerState::Disabled, Ordering::Relaxed);
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
        self.inner.state.load(Ordering::Relaxed)
    }
}

async fn run_listener(listener: TcpListener, cancellation_token: CancellationToken) {
    loop {
        select! {
            result = listener.accept() => {
                let Ok((mut stream, address)) = result else {
                    continue;
                };

                println!("Client accepted: {}", address);

                let message = format!("hello world {}", address).into_bytes();
                let temp = stream.write_all(&message).await;
                let _ = dbg!(temp);
            },
            _ = cancellation_token.cancelled() => {
                println!("Cancellation token cancelled.");
                break;
            }
        }
    }

    println!("Listener stopped.");
}
