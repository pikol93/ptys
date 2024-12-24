use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub struct StreamEntry {
    pub id: u32,
    pub parent_id: Option<u32>,
    token: CancellationToken,
    write_half: RwLock<OwnedWriteHalf>,
}

impl StreamEntry {
    pub fn new(id: u32, parent_id: Option<u32>, write_half: OwnedWriteHalf) -> Self {
        Self {
            id,
            parent_id,
            token: CancellationToken::new(),
            write_half: RwLock::new(write_half),
        }
    }

    pub async fn send(&self, buffer: &[u8]) -> anyhow::Result<()> {
        self.write_half.write().await.write_all(buffer).await?;

        Ok(())
    }

    pub fn clone_token(&self) -> CancellationToken {
        self.token.clone()
    }

    pub fn cancel(&self) {
        self.token.cancel();
    }
}
