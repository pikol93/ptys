use crate::application::window_view::WindowView;
use eframe::Frame;
use egui::{Context, Ui};

pub struct ApplicationInformationView {}

impl WindowView for ApplicationInformationView {
    fn get_title(&self) -> &'static str {
        "Application information"
    }

    fn display(&self, _context: &Context, _frame: &mut Frame, ui: &mut Ui) {
        ui.label(format!("Frame number: {}", _context.frame_nr()));
        ui.label(format!(
            "Time to draw last frame: {} ms",
            _frame.info().cpu_usage.unwrap_or(0.0) * 1000.0
        ));
    }
}
