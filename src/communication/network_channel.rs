use crate::communication::tcp_client_channel::TcpClientChannel;
use crate::communication::tcp_server_channel::TcpServerChannel;

#[derive(Debug)]
pub enum NetworkChannel {
    TcpServer(TcpServerChannel),
    TcpClient(TcpClientChannel),
}

impl NetworkChannel {
    pub fn id(&self) -> u32 {
        match self {
            NetworkChannel::TcpServer(channel) => channel.id,
            NetworkChannel::TcpClient(channel) => channel.id,
        }
    }
}
