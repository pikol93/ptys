use std::net::TcpListener;

#[derive(Debug)]
pub struct TcpServerChannel {
    pub id: u32,
    pub port: u16,
    listener: Option<TcpListener>,
}

impl TcpServerChannel {
    pub fn new(id: u32, port: u16) -> Self {
        Self {
            id,
            port,
            listener: None,
        }
    }
}
