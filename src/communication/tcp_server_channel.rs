use std::net::TcpListener;

#[derive(Debug)]
pub struct TcpServerChannel {
    port: u16,
    listener: Option<TcpListener>,
}

impl TcpServerChannel {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            listener: None,
        }
    }
}
