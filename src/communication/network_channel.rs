use crate::application::connections::model::ChannelType;
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

    pub fn channel_type(&self) -> ChannelType {
        match self {
            NetworkChannel::TcpServer(_) => ChannelType::TcpServer,
            NetworkChannel::TcpClient(_) => ChannelType::TcpClient,
        }
    }

    pub fn hostname<'a: 'b, 'b>(&'a self) -> &'b str {
        match self {
            NetworkChannel::TcpServer(_) => "-",
            NetworkChannel::TcpClient(channel) => &channel.hostname,
        }
    }

    pub fn port(&self) -> u16 {
        match self {
            NetworkChannel::TcpServer(channel) => channel.port,
            NetworkChannel::TcpClient(channel) => channel.port,
        }
    }
}
