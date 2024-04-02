use std::sync::Arc;

use anyhow::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio::select;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub struct StreamEntry {
    pub id: u32,
    pub parent_id: Option<u32>,
    token: CancellationToken,
    write_half: OwnedWriteHalf,
}

impl StreamEntry {
    pub fn new(id: u32, parent_id: Option<u32>, stream: TcpStream, runtime: &Runtime) -> Self {
        let parent_token = CancellationToken::new();
        let token_clone = parent_token.clone();
        let (read_half, write_half) = stream.into_split();

        Self::start_reading(token_clone, read_half, runtime);

        Self {
            id,
            parent_id,
            token: parent_token,
            write_half,
        }
    }

    pub fn cancel(&self) {
        self.token.cancel();
    }

    pub async fn write(&mut self, data: &[u8]) -> anyhow::Result<()> {
        self.write_half.write_all(data).await?;
        Ok(())
    }

    fn start_reading(token: CancellationToken, mut read_half: OwnedReadHalf, runtime: &Runtime) {
        let mut buffer = Box::new([0u8; 1024]);
        runtime.spawn(async move {
            select! {
                _ = async {
                    loop {
                        let Ok(read_count) = read_half.read(buffer.as_mut()).await else {
                            println!("Read returned an Err.");
                            return;
                        };

                        if read_count == 0 {
                            println!("Read count is equal to 0. Breaking.");
                            break;
                        }

                        println!("Received {} bytes", read_count);
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

#[derive(Clone, Debug)]
pub struct TcpStreamContainer {
    pub streams: Arc<RwLock<Vec<StreamEntry>>>,
    runtime: Arc<Runtime>,
}

impl TcpStreamContainer {
    pub fn new(runtime: Arc<Runtime>) -> Self {
        Self {
            streams: Arc::new(RwLock::new(vec![])),
            runtime,
        }
    }

    pub async fn add_stream(&self, parent_id: Option<u32>, stream: TcpStream) {
        let mut streams = self.streams.write().await;

        let id = streams
            .iter()
            .map(|entry| entry.id)
            .max()
            .map(|max| max + 1)
            .unwrap_or(1);

        let entry = StreamEntry::new(id, parent_id, stream, self.runtime.as_ref());
        let token = entry.token.clone();
        streams.push(entry);

        self.hook_token_cancellation(id, token);
    }

    pub async fn cancel_stream(&self, id: u32) -> anyhow::Result<()> {
        let streams = self.streams.read().await;
        let Some(entry) = streams.iter().find(|entry| entry.id == id) else {
            return Err(Error::msg("Could not find an entry by the given ID."));
        };

        entry.cancel();
        Ok(())
    }

    fn hook_token_cancellation(&self, id: u32, token: CancellationToken) {
        let streams = self.streams.clone();
        self.runtime.spawn(async move {
            token.cancelled().await;
            let mut entries = streams.write().await;
            let position = entries.iter().position(|entry| entry.id == id).unwrap();
            entries.remove(position);
        });
    }
}
