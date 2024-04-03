use eframe::Frame;
use egui::{Context, Ui};

pub trait WindowView {
    fn get_title(&self) -> &'static str;
    fn display(&self, context: &Context, frame: &mut Frame, ui: &mut Ui);
}
