use std::sync::Arc;

use egui::Ui;
use tokio::sync::RwLock;

use crate::application::connections::controller::ConnectionsController;
use crate::application::connections::model::ConnectionsModel;

pub struct ConnectionsView {
    pub model: Arc<RwLock<ConnectionsModel>>,
    pub controller: Arc<ConnectionsController>,
}

impl ConnectionsView {
    pub fn display(&self, ui: &mut Ui) {
        ui.heading("PTYS connections");
        if ui.button("Back").clicked() {
            self.controller.button_clicked_back();
        }
    }
}
