use std::{
    net::SocketAddr,
    sync::atomic::{AtomicUsize, Ordering},
};

use async_trait::async_trait;
use eyre::{eyre, Result};
use log::debug;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::{broadcast::Sender, Mutex},
};

use super::{Channel, EventBytesReceived, EventBytesSent};

pub struct TcpClientChannel {
    event_bytes_received: Sender<EventBytesReceived>,
    event_bytes_sent: Sender<EventBytesSent>,
    bytes_received: AtomicUsize,
    bytes_sent: AtomicUsize,
    address: SocketAddr,
    read_half: Mutex<OwnedReadHalf>,
    write_half: Mutex<OwnedWriteHalf>,
}

impl TcpClientChannel {
    pub fn new(address: SocketAddr, stream: TcpStream) -> Self {
        let (rx, tx) = stream.into_split();
        Self {
            event_bytes_received: Sender::new(16),
            event_bytes_sent: Sender::new(16),
            bytes_received: AtomicUsize::new(0),
            bytes_sent: AtomicUsize::new(0),
            address,
            read_half: tokio::sync::Mutex::new(rx),
            write_half: Mutex::new(tx),
        }
    }
}

#[async_trait]
impl Channel for TcpClientChannel {
    fn event_bytes_received(&self) -> &Sender<EventBytesReceived> {
        &self.event_bytes_received
    }

    fn event_bytes_sent(&self) -> &Sender<EventBytesSent> {
        &self.event_bytes_sent
    }

    fn bytes_received(&self) -> usize {
        self.bytes_received.load(Ordering::Acquire)
    }

    fn bytes_sent(&self) -> usize {
        self.bytes_sent.load(Ordering::Acquire)
    }

    fn address(&self) -> SocketAddr {
        self.address
    }

    async fn work_reader(&self) -> Result<()> {
        let mut guard = match self.read_half.try_lock() {
            Ok(ok) => ok,
            Err(error) => {
                return Err(eyre!("{}", error));
            }
        };
        let mut buffer = vec![0u8; 4096];

        loop {
            match guard.read(&mut buffer).await {
                Ok(bytes_received) => {
                    self.bytes_received
                        .fetch_add(bytes_received, Ordering::AcqRel);
                }
                Err(err) => {
                    debug!("Error while reading: {}", err);
                    break;
                }
            }
        }

        Ok(())
    }

    async fn write(&self, bytes: &[u8]) {
        let result = {
            let mut a = self.write_half.lock().await;
            a.write(bytes).await
        };

        match result {
            Ok(bytes_sent) => {
                self.bytes_sent.fetch_add(bytes_sent, Ordering::AcqRel);
            }
            Err(err) => {
                debug!("Error while sending: {}", err);
            }
        }
    }
}
