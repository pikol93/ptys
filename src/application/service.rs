use std::sync::Arc;

use tokio::runtime::Runtime;
use tokio::sync::RwLock;

use crate::application::model::{ApplicationModel, DisplayedView};

pub struct ApplicationService {
    pub model: Arc<RwLock<ApplicationModel>>,
    pub runtime: Arc<Runtime>,
}

impl ApplicationService {
    pub fn change_displayed_view(&self, displayed_view: DisplayedView) {
        let model = self.model.clone();

        self.runtime.spawn(async move {
            model.write().await.displayed_view = displayed_view;
        });
    }
}
