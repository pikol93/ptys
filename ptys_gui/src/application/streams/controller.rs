use std::sync::Arc;

use tokio::runtime::Runtime;
use tokio::sync::RwLock;

use crate::application::repaint_scheduler::RepaintScheduler;
use crate::application::streams::model::StreamsModel;
use crate::communication::streams::tcp_stream_container::TcpStreamContainer;

pub struct StreamsController {
    pub model: Arc<RwLock<StreamsModel>>,
    pub runtime: Arc<Runtime>,
    pub repaint_scheduler: Arc<RepaintScheduler>,
    pub stream_container: TcpStreamContainer,
}

impl StreamsController {
    pub fn button_clicked_connection_stop(&self, id: u32) {
        let stream_container = self.stream_container.clone();
        self.runtime.spawn(async move {
            let result = stream_container.cancel_stream(id).await;
            println!("Stop result: {:?}", result);
        });
    }
}
