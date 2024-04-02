use egui::{Context, Ui};

pub trait WindowView {
    fn get_title(&self) -> &'static str;
    fn display(&self, context: &Context, ui: &mut Ui);
}
