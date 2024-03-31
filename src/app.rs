use eframe::Frame;
use egui::{CentralPanel, Context};

use crate::application::model::ApplicationModel;
use crate::application::view::display_app;

pub struct App {
    pub model: ApplicationModel,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            display_app(ui, &mut self.model);
        });
    }
}
