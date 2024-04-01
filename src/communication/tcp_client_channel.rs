use std::net::TcpStream;

#[derive(Debug)]
pub struct TcpClientChannel {
    pub id: u32,
    pub hostname: String,
    pub port: u16,
    stream: Option<TcpStream>,
}

impl TcpClientChannel {
    pub fn new(id: u32, hostname: String, port: u16) -> Self {
        Self {
            id,
            hostname,
            port,
            stream: None,
        }
    }
}
