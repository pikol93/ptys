use std::str::FromStr;
use std::sync::Arc;

use tokio::runtime::Runtime;
use tokio::sync::RwLock;

use crate::application::connections::streams::model::StreamsModel;
use crate::application::connections::streams::service::StreamsService;
use crate::application::model::DisplayedView;
use crate::application::repaint_scheduler::RepaintScheduler;
use crate::application::service::ApplicationService;

pub struct StreamsController {
    pub model: Arc<RwLock<StreamsModel>>,
    pub service: Arc<StreamsService>,
    pub application_service: Arc<ApplicationService>,
    pub runtime: Arc<Runtime>,
    pub repaint_scheduler: Arc<RepaintScheduler>,
}

impl StreamsController {
    pub fn validate_add_connection_fields(&self) {
        // TODO: Add validation
    }

    pub fn button_clicked_add_connection(&self) {
        let model = self.model.clone();
        let service = self.service.clone();
        let repaint_scheduler = self.repaint_scheduler.clone();

        self.runtime.spawn(async move {
            let result = Self::add_stream(&model, &service).await;
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
        let service = self.service.clone();

        self.runtime.spawn(async move {
            let result = service.stop_stream(id).await;
            println!("Stop result: {:?}", result);
        });
    }

    pub fn button_clicked_back(&self) {
        self.application_service
            .change_displayed_view(DisplayedView::Menu);
    }

    async fn add_stream(
        model: &Arc<RwLock<StreamsModel>>,
        service: &Arc<StreamsService>,
    ) -> anyhow::Result<()> {
        let model = &model.read().await.add_connection_model;
        let hostname = model.hostname.as_str();
        let port = u16::from_str(&model.port)?;

        service.add_stream(hostname, port).await
    }
}
