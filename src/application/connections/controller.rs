use std::sync::Arc;

use tokio::runtime::Runtime;

use crate::application::model::DisplayedView;
use crate::application::service::ApplicationService;

pub struct ConnectionsController {
    pub application_service: Arc<ApplicationService>,
    pub runtime: Arc<Runtime>,
}

impl ConnectionsController {
    pub fn button_clicked_back(&self) {
        self.application_service
            .change_displayed_view(DisplayedView::Menu);
    }
}
