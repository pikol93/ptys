use std::process::exit;
use std::sync::Arc;

use tokio::runtime::Runtime;

use crate::application::model::DisplayedView;
use crate::application::service::ApplicationService;

pub struct MenuController {
    pub application_service: Arc<ApplicationService>,
    pub runtime: Arc<Runtime>,
}

impl MenuController {
    pub fn button_clicked_connections(&self) {
        self.application_service
            .change_displayed_view(DisplayedView::Connections);
    }

    pub fn button_clicked_exit(&self) {
        exit(0);
    }
}
