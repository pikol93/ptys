use std::str::FromStr;
use std::sync::Arc;

use crate::application::connections::model::ConnectionsModel;
use crate::application::connections::service::ConnectionsService;
use tokio::runtime::Runtime;
use tokio::sync::RwLock;

use crate::application::model::DisplayedView;
use crate::application::repaint_scheduler::RepaintScheduler;
use crate::application::service::ApplicationService;

pub struct ConnectionsController {
    pub model: Arc<RwLock<ConnectionsModel>>,
    pub service: Arc<ConnectionsService>,
    pub application_service: Arc<ApplicationService>,
    pub runtime: Arc<Runtime>,
    pub repaint_scheduler: Arc<RepaintScheduler>,
}

impl ConnectionsController {
    pub fn validate_add_connection_fields(&self) {
        // TODO: Add validation
    }

    pub fn button_clicked_connection_added(&self) {
        let model = self.model.clone();
        let service = self.service.clone();
        let repaint_scheduler = self.repaint_scheduler.clone();

        self.runtime.spawn(async move {
            let result = Self::add_connection(&model, &service).await;
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

    pub fn button_clicked_back(&self) {
        self.application_service
            .change_displayed_view(DisplayedView::Menu);
    }

    async fn add_connection(
        model: &Arc<RwLock<ConnectionsModel>>,
        service: &Arc<ConnectionsService>,
    ) -> anyhow::Result<()> {
        let model = &model.read().await.add_connection_model;
        let hostname = model.hostname.as_str();
        let port = u16::from_str(&model.port)?;
        let channel_type = model.channel_type;

        service.add_connection(hostname, port, channel_type).await
    }
}
