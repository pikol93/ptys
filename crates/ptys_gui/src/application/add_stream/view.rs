use eframe::Frame;
use std::sync::Arc;

use egui::{Context, Ui};
use tokio::sync::RwLock;

use crate::application::add_stream::controller::AddStreamController;
use crate::application::add_stream::model::AddStreamModel;
use crate::application::window_view::WindowView;

pub struct AddStreamView {
    pub model: Arc<RwLock<AddStreamModel>>,
    pub controller: AddStreamController,
}

impl WindowView for AddStreamView {
    fn get_title(&self) -> &'static str {
        "Add stream"
    }

    fn display(&self, _context: &Context, _frame: &mut Frame, ui: &mut Ui) {
        let mut model = self.model.blocking_write();

        ui.heading("Connect");
        ui.horizontal(|ui| {
            ui.label("Hostname");
            if ui.text_edit_singleline(&mut model.hostname).changed() {
                self.controller.validate_add_connection_fields();
            }
        });
        ui.horizontal(|ui| {
            ui.label("Port");
            if ui.text_edit_singleline(&mut model.port).changed() {
                self.controller.validate_add_connection_fields();
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Add connection").clicked() {
                self.controller.button_clicked_add_connection();
            }
            if let Some(error) = &model.error {
                ui.label(error);
            }
        });
    }
}
