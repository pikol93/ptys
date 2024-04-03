use crate::application::listeners::controller::ListenersController;
use crate::application::listeners::model::{AddListenerModel, ListenerModel, ListenersModel};
use crate::application::streams::model::{AddStreamModel, StreamModel};
use crate::application::window_view::WindowView;
use egui::{Context, Grid, RichText, Ui};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ListenersView {
    pub model: Arc<RwLock<ListenersModel>>,
    pub controller: ListenersController,
}

impl WindowView for ListenersView {
    fn get_title(&self) -> &'static str {
        "Listeners"
    }

    fn display(&self, _context: &Context, ui: &mut Ui) {
        let mut model = self.model.blocking_write();

        self.display_add_listener_section(ui, &mut model.add_connection_model);
        self.display_listeners_list(ui, &model.stream_models);
    }
}

impl ListenersView {
    fn display_add_listener_section(&self, ui: &mut Ui, model: &mut AddListenerModel) {
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

    fn display_listeners_list(&self, ui: &mut Ui, models: &[ListenerModel]) {
        ui.heading("Listeners");
        Grid::new("listeners").show(ui, |ui| {
            for model in models {
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
