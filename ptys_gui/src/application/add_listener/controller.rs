use std::str::FromStr;
use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio::sync::RwLock;

use crate::application::add_listener::model::AddListenerModel;
use crate::application::repaint_scheduler::RepaintScheduler;
use crate::communication::listeners::tcp_listener_container::TcpListenersContainer;

pub struct AddListenerController {
    pub model: Arc<RwLock<AddListenerModel>>,
    pub listeners_container: TcpListenersContainer,
    pub runtime: Arc<Runtime>,
    pub repaint_scheduler: Arc<RepaintScheduler>,
}

impl AddListenerController {
    pub fn validate_add_listener_fields(&self) {
        // TODO: Add validation
    }

    pub fn button_clicked_add_listener(&self) {
        let model = self.model.clone();
        let listener_container = self.listeners_container.clone();
        let repaint_scheduler = self.repaint_scheduler.clone();

        self.runtime.spawn(async move {
            let result = Self::add_listener(&model, &listener_container).await;
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

    async fn add_listener(
        model: &Arc<RwLock<AddListenerModel>>,
        listeners_container: &TcpListenersContainer,
    ) -> anyhow::Result<()> {
        let model = &model.read().await;
        let port = u16::from_str(&model.port)?;
        let listener = TcpListener::bind(("0.0.0.0", port)).await?;

        listeners_container.add_listener(listener).await;
        Ok(())
    }
}
