use std::str::FromStr;
use std::sync::Arc;

use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio::sync::RwLock;

use crate::application::add_stream::model::AddStreamModel;
use crate::application::repaint_scheduler::RepaintScheduler;
use crate::communication::streams::tcp_stream_container::TcpStreamContainer;

pub struct AddStreamController {
    pub model: Arc<RwLock<AddStreamModel>>,
    pub runtime: Arc<Runtime>,
    pub repaint_scheduler: Arc<RepaintScheduler>,
    pub stream_container: TcpStreamContainer,
}

impl AddStreamController {
    pub fn validate_add_connection_fields(&self) {
        // TODO: Add validation
    }

    pub fn button_clicked_add_connection(&self) {
        let model = self.model.clone();
        let stream_container = self.stream_container.clone();
        let repaint_scheduler = self.repaint_scheduler.clone();

        self.runtime.spawn(async move {
            let result = Self::add_stream(&model, &stream_container).await;
            let model = &mut model.write().await;
            match result {
                Ok(_) => {
                    model.reset();
                }
                Err(error) => {
                    model.error = Some(error.to_string());
                }
            }

            repaint_scheduler.schedule_now().await;
        });
    }

    async fn add_stream(
        model: &Arc<RwLock<AddStreamModel>>,
        stream_container: &TcpStreamContainer,
    ) -> anyhow::Result<()> {
        let model = &model.read().await;
        let hostname = model.hostname.as_str();
        let port = u16::from_str(&model.port)?;
        let stream = TcpStream::connect((hostname, port)).await?;

        stream_container.add_stream(None, stream).await;
        Ok(())
    }
}
