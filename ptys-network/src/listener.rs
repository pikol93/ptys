use std::sync::Arc;

use eyre::{eyre, Result};
use tokio::io::AsyncWriteExt;
use tokio::select;
use tokio::sync::RwLock;
use tokio::{net::TcpListener, runtime::Runtime};
use tokio_util::sync::CancellationToken;

#[derive(Clone)]
pub struct Listener {
    pub id: usize,
    pub port: u16,
    runtime: Arc<Runtime>,
    inner: Arc<Inner>,
}

#[derive(Default)]
struct Inner {
    cancellation_token: RwLock<Option<CancellationToken>>,
}

impl Listener {
    pub fn new(id: usize, port: u16, runtime: Arc<Runtime>) -> Listener {
        Listener {
            id,
            port,
            runtime,
            inner: Arc::default(),
        }
    }

    pub async fn start(&self) -> Result<()> {
        let inner = self.inner.clone();
        let mut cancellation_token = inner.cancellation_token.write().await;
        if cancellation_token.is_some() {
            return Err(eyre!("Listener already started."));
        }

        let new_cancellation_token = CancellationToken::new();
        let child_cancellation_token = new_cancellation_token.child_token();
        *cancellation_token = Some(new_cancellation_token);

        drop(cancellation_token);

        let listener = TcpListener::bind(("0.0.0.0", self.port)).await?;
        self.runtime.spawn(async move {
            run_listener(listener, child_cancellation_token).await;

            let mut cancellation_token = inner.cancellation_token.write().await;
            *cancellation_token = None;
        });

        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        let cancellation_token = self.inner.cancellation_token.read().await;
        let Some(cancellation_token) = cancellation_token.as_ref() else {
            return Err(eyre!("Listener already cancelled."));
        };

        cancellation_token.cancel();
        Ok(())
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
