use tokio::net::TcpStream;

use crate::communication::tcp_stream_container::TcpStreamContainer;

pub struct StreamsService {
    pub stream_container: TcpStreamContainer,
}

impl StreamsService {
    pub async fn add_stream(&self, hostname: &str, port: u16) -> anyhow::Result<()> {
        let stream = TcpStream::connect((hostname, port)).await?;
        self.stream_container.add_stream(None, stream).await;

        Ok(())
    }

    pub async fn stop_stream(&self, id: u32) -> anyhow::Result<()> {
        self.stream_container.cancel_stream(id).await
    }
}
