use std::sync::Arc;

use tokio::runtime::Runtime;
use tokio::sync::mpsc::Sender;

use crate::communication::network_channel::NetworkChannel;
use crate::communication::tcp_client_channel::TcpClientChannel;
use crate::communication::tcp_server_channel::TcpServerChannel;

#[derive(Debug)]
pub struct ChannelContainer {
    channels: Vec<NetworkChannel>,
    runtime: Arc<Runtime>,
    channel_added_tx: Sender<()>,
    channel_removed_tx: Sender<()>,
}

impl ChannelContainer {
    pub fn new(
        runtime: Arc<Runtime>,
        channel_added_tx: Sender<()>,
        channel_removed_tx: Sender<()>,
    ) -> Self {
        Self {
            channels: vec![],
            runtime,
            channel_added_tx,
            channel_removed_tx,
        }
    }

    pub fn add_server(&mut self, port: u16) -> anyhow::Result<()> {
        let id = self.get_next_id();

        self.channels
            .push(NetworkChannel::TcpServer(TcpServerChannel::new(id, port)));

        let tx = self.channel_added_tx.clone();
        self.runtime.spawn(async move {
            tx.send(()).await.unwrap();
        });

        Ok(())
    }

    pub fn add_client(&mut self, hostname: &str, port: u16) -> anyhow::Result<()> {
        let id = self.get_next_id();

        self.channels
            .push(NetworkChannel::TcpClient(TcpClientChannel::new(
                id,
                hostname.to_string(),
                port,
            )));

        let tx = self.channel_added_tx.clone();
        self.runtime.spawn(async move {
            tx.send(()).await.unwrap();
        });

        Ok(())
    }

    fn get_highest_channel_id(&self) -> Option<u32> {
        self.channels.iter().map(|channel| channel.id()).max()
    }

    fn get_next_id(&self) -> u32 {
        self.get_highest_channel_id()
            .map(|highest_id| highest_id + 1)
            .unwrap_or(1)
    }
}
