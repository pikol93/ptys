use std::sync::Arc;

use anyhow::Error;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio::select;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub struct TcpClientChannel {
    pub id: u32,
    pub hostname: String,
    pub port: u16,
    token: Arc<RwLock<Option<CancellationToken>>>,
}

impl TcpClientChannel {
    pub fn new(id: u32, hostname: String, port: u16) -> Self {
        Self {
            id,
            hostname,
            port,
            token: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn start(&self, runtime: Arc<Runtime>) -> anyhow::Result<()> {
        let mut token = self.token.write().await;
        if token.is_some() {
            return Err(Error::msg("Client is already connected."));
        }

        let address = (self.hostname.clone(), self.port);
        let stream = TcpStream::connect(address).await?;
        let parent_token = CancellationToken::new();
        let child_token = parent_token.child_token();
        *token = Some(parent_token);

        let (mut read_half, write_half) = stream.into_split();

        let mut buf = [0u8; 1024];

        runtime.spawn(async move {
            select! {
                _ = child_token.cancelled() => {
                    println!("Child token cancelled.");
                }
                _ = async {
                    loop {
                        let read_result = read_half.read(&mut buf).await;
                        match read_result {
                            Ok(bytes_count) => {
                                println!("Read bytes: {:?}", bytes_count);
                                if bytes_count == 0 {
                                    break;
                                }
                            }
                            Err(error) => {
                                println!("Error: {:?}", error);
                            }
                        }
                    }
                } => {
                    println!("Finished reading.");
                }
            }
        });

        Ok(())
    }

    pub async fn stop(&self) -> anyhow::Result<()> {
        let mut token = self.token.write().await;
        if token.is_none() {
            return Err(Error::msg("Client is already disconnected."));
        }

        token.take().unwrap().cancel();

        Ok(())
    }
}
