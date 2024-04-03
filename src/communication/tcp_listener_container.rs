use anyhow::Error;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::communication::tcp_stream_container::TcpStreamContainer;
use tokio::runtime::Runtime;
use tokio::select;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

pub struct ListenerEntry {
    pub id: u32,
    token: CancellationToken,
}

impl ListenerEntry {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            token: CancellationToken::new(),
        }
    }
}

impl ListenerEntry {
    pub fn cancel(&self) {
        self.token.cancel();
    }
}

#[derive(Clone)]
pub struct TcpListenersContainer {
    pub listeners: Arc<RwLock<Vec<ListenerEntry>>>,
    stream_container: TcpStreamContainer,
    runtime: Arc<Runtime>,
}

impl TcpListenersContainer {
    pub fn new(stream_container: TcpStreamContainer, runtime: Arc<Runtime>) -> Self {
        TcpListenersContainer {
            listeners: Arc::new(Default::default()),
            stream_container,
            runtime,
        }
    }

    pub async fn add_listener(&self, listener: TcpListener) {
        let mut streams = self.listeners.write().await;

        let id = streams
            .iter()
            .map(|entry| entry.id)
            .max()
            .map(|max| max + 1)
            .unwrap_or(1);

        let entry = ListenerEntry::new(id);
        let token = entry.token.clone();
        streams.push(entry);

        self.start_reading(id, token, listener)
    }

    pub async fn cancel_listener(&self, id: u32) -> anyhow::Result<()> {
        let streams = self.listeners.read().await;
        let Some(entry) = streams.iter().find(|entry| entry.id == id) else {
            return Err(Error::msg("Could not find an entry by the given ID."));
        };

        entry.cancel();
        Ok(())
    }

    fn start_reading(&self, id: u32, token: CancellationToken, listener: TcpListener) {
        let stream_container = self.stream_container.clone();

        self.runtime.spawn(async move {
            select! {
                _ = async {
                    loop {
                        let accept_result = listener.accept().await;
                        let Ok((stream, address)) = accept_result else {
                            break;
                        };
                        println!("Stream connected from address: {}", address);

                        stream_container.add_stream(Some(id), stream).await;
                    }
                } => {
                    token.cancel();
                    println!("Finished reading.");
                }
                _ = token.cancelled() => {
                    println!("Child token cancelled.");
                }
            }
        });
    }
}
