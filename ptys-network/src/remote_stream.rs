use std::{net::SocketAddr, sync::Arc};

use super::sendable::Sendable;
use async_trait::async_trait;
use eyre::Result;
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::RwLock};

pub struct RemoteStream {
    pub address: SocketAddr,
    inner: Arc<Inner>,
}

struct Inner {
    stream: RwLock<TcpStream>,
}

#[async_trait]
impl Sendable for RemoteStream {
    async fn send(&self, bytes: &[u8]) -> Result<()> {
        let mut stream = self.inner.stream.write().await;
        stream.write_all(bytes).await?;

        Ok(())
    }
}
