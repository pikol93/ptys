use std::sync::Arc;

use egui::{Context, Ui};
use tokio::sync::RwLock;

use crate::application::add_listener::controller::AddListenerController;
use crate::application::add_listener::model::AddListenerModel;
use crate::application::window_view::WindowView;

pub struct AddListenerView {
    pub model: Arc<RwLock<AddListenerModel>>,
    pub controller: AddListenerController,
}

impl WindowView for AddListenerView {
    fn get_title(&self) -> &'static str {
        "Add listener"
    }

    fn display(&self, _context: &Context, ui: &mut Ui) {
        let mut model = self.model.blocking_write();

        ui.horizontal(|ui| {
            ui.label("Port");
            if ui.text_edit_singleline(&mut model.port).changed() {
                self.controller.validate_add_listener_fields();
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Add listener").clicked() {
                self.controller.button_clicked_add_listener();
            }
            if let Some(error) = &model.error {
                ui.label(error);
            }
        });
    }
}
