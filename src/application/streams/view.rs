use std::sync::Arc;

use egui::{Context, Grid, RichText, Ui};
use tokio::sync::RwLock;

use crate::application::streams::controller::ConnectionsController;
use crate::application::streams::model::{
    AddStreamModel, StreamModel, StreamsModel,
};

pub struct StreamsView {
    pub model: Arc<RwLock<StreamsModel>>,
    pub controller: Arc<ConnectionsController>,
}

impl StreamsView {
    pub fn display(&self, _context: &Context, ui: &mut Ui) {
        let mut model = self.model.blocking_write();

        ui.label(RichText::new("PTYS streams").size(22.0));
        self.display_add_connection_section(ui, &mut model.add_connection_model);
        self.display_connection_list(ui, &model.get_connection_models());
        if ui.button("Back").clicked() {
            self.controller.button_clicked_back();
        }
    }

    fn display_add_connection_section(&self, ui: &mut Ui, model: &mut AddStreamModel) {
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

    fn display_listener_list(&self, ui: &mut Ui) {
        ui.label("TODO");
    }

    fn display_connection_list(&self, ui: &mut Ui, models: &[StreamModel]) {
        ui.heading("Connections");
        Grid::new("streams").show(ui, |ui| {
            for model in models {
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
