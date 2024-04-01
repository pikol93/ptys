use std::net::TcpStream;

#[derive(Debug)]
pub struct TcpClientChannel {
    hostname: String,
    port: u16,
    stream: Option<TcpStream>,
}

impl TcpClientChannel {
    pub fn new(hostname: String, port: u16) -> Self {
        Self {
            hostname,
            port,
            stream: None,
        }
    }
}
