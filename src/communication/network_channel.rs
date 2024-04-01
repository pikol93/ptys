use crate::communication::tcp_client_channel::TcpClientChannel;
use crate::communication::tcp_server_channel::TcpServerChannel;

#[derive(Debug)]
pub enum NetworkChannel {
    TcpServer(TcpServerChannel),
    TcpClient(TcpClientChannel),
}
