use std::sync::Arc;

use egui::Ui;

use crate::application::menu::controller::MenuController;

pub struct MenuView {
    pub controller: Arc<MenuController>,
}

impl MenuView {
    pub fn display(&self, ui: &mut Ui) {
        ui.heading("PTYS menu");

        if ui.button("Connections").clicked() {
            self.controller.button_clicked_connections();
        }

        if ui.button("Exit").clicked() {
            self.controller.button_clicked_exit();
        }
    }
}
