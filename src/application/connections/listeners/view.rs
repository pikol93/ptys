use egui::{Context, RichText, Ui};

pub struct ListenersView {}

impl ListenersView {
    pub fn display(&self, _context: &Context, ui: &mut Ui) {
        ui.label(RichText::new("listeners").size(22.0));
    }
}
