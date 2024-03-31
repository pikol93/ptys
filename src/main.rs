use eframe::{NativeOptions, run_native};
use egui::ViewportBuilder;

use crate::app::App;
use crate::application::model::ApplicationModel;

mod app;
pub mod application;

fn main() {
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1600.0, 900.0]),
        ..Default::default()
    };

    let application_model = ApplicationModel::default();
    let app = App {
        model: application_model,
    };

    run_native("PTYS", options, Box::new(|_context| Box::new(app))).unwrap();
}
