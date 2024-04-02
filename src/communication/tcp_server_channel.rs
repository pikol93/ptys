use anyhow::Error;
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

    pub fn start(&mut self) -> anyhow::Result<()> {
        if self.listener.is_some() {
            return Err(Error::msg("Server is already started."));
        }

        self.listener = Some(TcpListener::bind(("0.0.0.0", self.port))?);

        Ok(())
    }
}
