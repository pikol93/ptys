use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio::select;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub struct StreamEntry {
    pub id: u32,
    pub parent_id: Option<u32>,
    token: CancellationToken,
    write_half: OwnedWriteHalf,
}

impl StreamEntry {
    pub fn new(id: u32, parent_id: Option<u32>, stream: TcpStream, runtime: &Runtime) -> Self {
        let parent_token = CancellationToken::new();
        let token_clone = parent_token.clone();
        let (read_half, write_half) = stream.into_split();

        Self::start_reading(token_clone, read_half, runtime);

        Self {
            id,
            parent_id,
            token: parent_token,
            write_half,
        }
    }

    pub fn clone_token(&self) -> CancellationToken {
        self.token.clone()
    }

    pub fn cancel(&self) {
        self.token.cancel();
    }

    pub async fn write(&mut self, data: &[u8]) -> anyhow::Result<()> {
        self.write_half.write_all(data).await?;
        Ok(())
    }

    fn start_reading(token: CancellationToken, mut read_half: OwnedReadHalf, runtime: &Runtime) {
        let mut buffer = Box::new([0u8; 1024]);
        runtime.spawn(async move {
            select! {
                _ = async {
                    loop {
                        let Ok(read_count) = read_half.read(buffer.as_mut()).await else {
                            println!("Read returned an Err.");
                            return;
                        };

                        if read_count == 0 {
                            println!("Read count is equal to 0. Breaking.");
                            break;
                        }

                        println!("Received {} bytes", read_count);
                    }
                } => {
                    token.cancel();
                    println!("Finished reading.");
                }
                _ = token.cancelled() => {
                    println!("Child token cancelled.");
                }
            }
        });
    }
}
