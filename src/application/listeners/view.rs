use crate::application::window_view::WindowView;
use egui::{Context, RichText, Ui};

pub struct ListenersView {}

impl WindowView for ListenersView {
    fn get_title(&self) -> &'static str {
        "Listeners"
    }

    fn display(&self, context: &Context, ui: &mut Ui) {
        ui.label(RichText::new("listeners").size(22.0));
    }
}
