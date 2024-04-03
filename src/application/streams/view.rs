use eframe::Frame;
use std::sync::Arc;

use egui::{Context, Grid, Ui};
use tokio::sync::RwLock;

use crate::application::streams::controller::StreamsController;
use crate::application::streams::model::StreamsModel;
use crate::application::window_view::WindowView;

pub struct StreamsView {
    pub model: Arc<RwLock<StreamsModel>>,
    pub controller: Arc<StreamsController>,
}

impl WindowView for StreamsView {
    fn get_title(&self) -> &'static str {
        "Streams"
    }

    fn display(&self, _context: &Context, _frame: &mut Frame, ui: &mut Ui) {
        let model = self.model.blocking_read();

        Grid::new("streams").show(ui, |ui| {
            for model in &model.stream_models {
                ui.label(model.id.to_string());
                ui.label(
                    model
                        .parent_id
                        .map(|id| id.to_string())
                        .unwrap_or("-".to_owned()),
                );
                ui.label(model.port.to_string());
                if ui.button("Stop").clicked() {
                    self.controller.button_clicked_connection_stop(model.id);
                }
                ui.end_row();
            }
        });
    }
}
