use std::str::FromStr;
use std::sync::Arc;
use tokio::net::TcpListener;

use tokio::runtime::Runtime;
use tokio::sync::RwLock;

use crate::application::listeners::model::ListenersModel;
use crate::application::repaint_scheduler::RepaintScheduler;
use crate::communication::tcp_listener_container::TcpListenersContainer;

pub struct ListenersController {
    pub model: Arc<RwLock<ListenersModel>>,
    pub listeners_container: TcpListenersContainer,
    pub runtime: Arc<Runtime>,
    pub repaint_scheduler: Arc<RepaintScheduler>,
}

impl ListenersController {
    pub fn validate_add_listener_fields(&self) {
        // TODO: Add validation
    }

    pub fn button_clicked_add_listener(&self) {
        let model = self.model.clone();
        let listener_container = self.listeners_container.clone();
        let repaint_scheduler = self.repaint_scheduler.clone();

        self.runtime.spawn(async move {
            let result = Self::add_listener(&model, &listener_container).await;
            let model = &mut model.write().await.add_connection_model;
            match result {
                Ok(_) => {
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

    pub fn button_clicked_listener_stop(&self, id: u32) {
        let listeners_container = self.listeners_container.clone();
        self.runtime.spawn(async move {
            let result = listeners_container.cancel_listener(id).await;
            println!("Stop result: {:?}", result);
        });
    }

    async fn add_listener(
        model: &Arc<RwLock<ListenersModel>>,
        listeners_container: &TcpListenersContainer,
    ) -> anyhow::Result<()> {
        let model = &model.read().await.add_connection_model;
        let port = u16::from_str(&model.port)?;
        let listener = TcpListener::bind(("0.0.0.0", port)).await?;

        listeners_container.add_listener(listener).await;
        Ok(())
    }
}
