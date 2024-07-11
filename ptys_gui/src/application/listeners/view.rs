use eframe::Frame;
use std::sync::Arc;

use egui::{Context, Grid, Ui};
use tokio::sync::RwLock;

use crate::application::listeners::controller::ListenersController;
use crate::application::listeners::model::ListenersModel;
use crate::application::window_view::WindowView;

pub struct ListenersView {
    pub model: Arc<RwLock<ListenersModel>>,
    pub controller: ListenersController,
}

impl WindowView for ListenersView {
    fn get_title(&self) -> &'static str {
        "Listeners"
    }

    fn display(&self, _context: &Context, _frame: &mut Frame, ui: &mut Ui) {
        let model = self.model.blocking_read();

        ui.heading("Listeners");
        Grid::new("listeners").show(ui, |ui| {
            for model in &model.listeners_models {
                ui.label(model.id.to_string());
                ui.label(model.port.to_string());
                if ui.button("Stop").clicked() {
                    self.controller.button_clicked_listener_stop(model.id);
                }
                ui.end_row();
            }
        });
    }
}
