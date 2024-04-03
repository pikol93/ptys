use std::sync::Arc;

use crate::communication::streams::event_types::ReceivedDataEvent;
use anyhow::Error;
use tokio::io::AsyncReadExt;
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio::select;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

use crate::communication::streams::stream_entry::StreamEntry;

#[derive(Clone, Debug)]
pub struct TcpStreamContainer {
    pub streams: Arc<RwLock<Vec<Arc<StreamEntry>>>>,
    runtime: Arc<Runtime>,
    stream_added_tx: Sender<Arc<StreamEntry>>,
    stream_removed_tx: Sender<Arc<StreamEntry>>,
    stream_data_received_tx: Sender<ReceivedDataEvent>,
}

impl TcpStreamContainer {
    pub fn new(
        runtime: Arc<Runtime>,
        stream_added_tx: Sender<Arc<StreamEntry>>,
        stream_removed_tx: Sender<Arc<StreamEntry>>,
        stream_data_received_tx: Sender<ReceivedDataEvent>,
    ) -> Self {
        Self {
            streams: Arc::new(RwLock::new(vec![])),
            runtime,
            stream_added_tx,
            stream_removed_tx,
            stream_data_received_tx,
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

        let (read_half, write_half) = stream.into_split();

        let entry = Arc::new(StreamEntry::new(id, parent_id, write_half));

        Self::start_reading(
            entry.clone(),
            entry.clone_token(),
            read_half,
            self.runtime.as_ref(),
            self.stream_data_received_tx.clone(),
        );

        let token = entry.clone_token();
        self.stream_added_tx.send(entry.clone()).await.unwrap();
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
        let tx = self.stream_removed_tx.clone();

        self.runtime.spawn(async move {
            token.cancelled().await;
            let mut entries = streams.write().await;
            let position = entries.iter().position(|entry| entry.id == id).unwrap();
            let removed = entries.remove(position);

            tx.send(removed).await.unwrap();
        });
    }

    fn start_reading(
        entry: Arc<StreamEntry>,
        token: CancellationToken,
        mut read_half: OwnedReadHalf,
        runtime: &Runtime,
        stream_data_received_tx: Sender<ReceivedDataEvent>,
    ) {
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
                        stream_data_received_tx
                            .send(ReceivedDataEvent {
                                data: buffer[..read_count].to_vec(),
                                entry: entry.clone(),
                            })
                            .await
                            .unwrap()
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
