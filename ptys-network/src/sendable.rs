use async_trait::async_trait;
use eyre::Result;

#[async_trait]
pub trait Sendable {
    async fn send(&self, bytes: &[u8]) -> Result<()>;
}
