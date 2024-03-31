use egui::Ui;

use crate::application::model::ApplicationModel;

pub fn display_app(ui: &mut Ui, model: &mut ApplicationModel) {
    ui.heading("PTYS");
    ui.label(format!("{}", model.temporary));
}
