use std::{net::SocketAddr, sync::Arc};

use async_trait::async_trait;
use eyre::Result;
use tokio::sync::broadcast::Sender;

pub mod container;
pub mod tcp_client;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ChannelId(pub usize);

#[derive(Clone)]
pub struct EventBytesReceived {
    pub bytes: Arc<[u8]>,
}

#[derive(Clone)]
pub struct EventBytesSent {
    pub bytes: Arc<[u8]>,
}

#[async_trait]
pub trait Channel: Send + Sync {
    fn event_bytes_received(&self) -> &Sender<EventBytesReceived>;
    fn event_bytes_sent(&self) -> &Sender<EventBytesSent>;
    fn bytes_received(&self) -> usize;
    fn bytes_sent(&self) -> usize;
    fn address(&self) -> SocketAddr;
    async fn work_reader(&self) -> Result<()>;
    async fn write(&self, bytes: &[u8]);
}
