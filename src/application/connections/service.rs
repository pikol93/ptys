use crate::application::connections::model::ChannelType;
use crate::communication::channel_container::ChannelContainer;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ConnectionsService {
    pub channel_container: Arc<RwLock<ChannelContainer>>,
}

impl ConnectionsService {
    pub async fn add_connection(
        &self,
        hostname: &str,
        port: u16,
        channel_type: ChannelType,
    ) -> anyhow::Result<()> {
        match channel_type {
            ChannelType::TcpServer => self.channel_container.write().await.add_server(port),
            ChannelType::TcpClient => self
                .channel_container
                .write()
                .await
                .add_client(hostname, port),
        }
    }
}
