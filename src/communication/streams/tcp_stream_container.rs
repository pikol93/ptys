use std::sync::Arc;

use anyhow::Error;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
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
}

impl TcpStreamContainer {
    pub fn new(
        runtime: Arc<Runtime>,
        stream_added_tx: Sender<Arc<StreamEntry>>,
        stream_removed_tx: Sender<Arc<StreamEntry>>,
    ) -> Self {
        Self {
            streams: Arc::new(RwLock::new(vec![])),
            runtime,
            stream_added_tx,
            stream_removed_tx,
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

        let entry = Arc::new(StreamEntry::new(
            id,
            parent_id,
            stream,
            self.runtime.as_ref(),
        ));
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
}
