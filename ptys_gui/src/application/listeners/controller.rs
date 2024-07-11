use std::sync::Arc;

use tokio::runtime::Runtime;
use tokio::sync::RwLock;

use crate::application::listeners::model::ListenersModel;
use crate::application::repaint_scheduler::RepaintScheduler;
use crate::communication::listeners::tcp_listener_container::TcpListenersContainer;

pub struct ListenersController {
    pub model: Arc<RwLock<ListenersModel>>,
    pub listeners_container: TcpListenersContainer,
    pub runtime: Arc<Runtime>,
    pub repaint_scheduler: Arc<RepaintScheduler>,
}

impl ListenersController {
    pub fn button_clicked_listener_stop(&self, id: u32) {
        let listeners_container = self.listeners_container.clone();
        self.runtime.spawn(async move {
            let result = listeners_container.cancel_listener(id).await;
            println!("Stop result: {:?}", result);
        });
    }
}
