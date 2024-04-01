use std::sync::Arc;

use egui::{ComboBox, Context, Grid, RichText, Ui};
use strum::IntoEnumIterator;
use tokio::sync::RwLock;

use crate::application::connections::controller::ConnectionsController;
use crate::application::connections::model::{
    AddConnectionModel, AllConnectionsModel, ChannelType, ConnectionsModel,
};

pub struct ConnectionsView {
    pub model: Arc<RwLock<ConnectionsModel>>,
    pub controller: Arc<ConnectionsController>,
}

impl ConnectionsView {
    pub fn display(&self, _context: &Context, ui: &mut Ui) {
        let mut model = self.model.blocking_write();

        ui.label(RichText::new("PTYS connections").size(22.0));
        self.display_add_connection_section(ui, &mut model.add_connection_model);
        self.display_connection_list(ui, &mut model.all_connections_model);
        if ui.button("Back").clicked() {
            self.controller.button_clicked_back();
        }
    }

    fn display_add_connection_section(&self, ui: &mut Ui, model: &mut AddConnectionModel) {
        ui.heading("Add connection");
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
            ui.label("Type");
            ComboBox::from_label("")
                .selected_text(format!("{:?}", model.channel_type))
                .show_ui(ui, |ui| {
                    ChannelType::iter().for_each(|variant| {
                        ui.selectable_value(
                            &mut model.channel_type,
                            variant,
                            format!("{:?}", variant),
                        );
                    });
                });
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

    fn display_connection_list(&self, ui: &mut Ui, model: &mut AllConnectionsModel) {
        ui.heading("Available connections");
        Grid::new("available_connections").show(ui, |ui| {
            for model in &model.connections {
                ui.label(model.id.to_string());
                ui.label(format!("{:?}", model.channel_type));
                ui.label(model.hostname.to_string());
                ui.label(model.port.to_string());
                ui.label("TODO: STATE");
                if ui.button("Start").clicked() {
                    self.controller.button_clicked_connection_start(model.id);
                }
                if ui.button("Stop").clicked() {
                    self.controller.button_clicked_connection_stop(model.id);
                }
                if ui.button("Delete").clicked() {
                    self.controller.button_clicked_connection_remove(model.id);
                }
                ui.end_row();
            }
        });
    }
}
