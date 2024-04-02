use std::str::FromStr;
use std::sync::Arc;
use tokio::net::TcpStream;

use tokio::runtime::Runtime;
use tokio::sync::RwLock;

use crate::application::repaint_scheduler::RepaintScheduler;
use crate::application::streams::model::StreamsModel;
use crate::communication::tcp_stream_container::TcpStreamContainer;

pub struct StreamsController {
    pub model: Arc<RwLock<StreamsModel>>,
    pub runtime: Arc<Runtime>,
    pub repaint_scheduler: Arc<RepaintScheduler>,
    pub stream_container: TcpStreamContainer,
}

impl StreamsController {
    pub fn validate_add_connection_fields(&self) {
        // TODO: Add validation
    }

    pub fn button_clicked_add_connection(&self) {
        let model = self.model.clone();
        let stream_container = self.stream_container.clone();
        let repaint_scheduler = self.repaint_scheduler.clone();

        self.runtime.spawn(async move {
            let result = Self::add_stream(&model, &stream_container).await;
            let model = &mut model.write().await.add_connection_model;
            match result {
                Ok(_) => {
                    model.hostname.clear();
                    model.port.clear();
                    model.error = None;
                }
                Err(error) => {
                    model.error = Some(error.to_string());
                }
            }

            repaint_scheduler.schedule_now().await;
        });
    }

    pub fn button_clicked_connection_stop(&self, id: u32) {
        let stream_container = self.stream_container.clone();
        self.runtime.spawn(async move {
            let result = stream_container.cancel_stream(id).await;
            println!("Stop result: {:?}", result);
        });
    }

    async fn add_stream(
        model: &Arc<RwLock<StreamsModel>>,
        stream_container: &TcpStreamContainer,
    ) -> anyhow::Result<()> {
        let model = &model.read().await.add_connection_model;
        let hostname = model.hostname.as_str();
        let port = u16::from_str(&model.port)?;
        let stream = TcpStream::connect((hostname, port)).await?;

        stream_container.add_stream(None, stream).await;
        Ok(())
    }
}
