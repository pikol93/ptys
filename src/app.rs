use std::sync::Arc;

use eframe::Frame;
use egui::{CentralPanel, Context};
use tokio::sync::RwLock;

use crate::application::model::ApplicationModel;
use crate::application::view::ApplicationView;

pub struct App {
    pub model: Arc<RwLock<ApplicationModel>>,
    pub view: Arc<ApplicationView>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.view.display(ui);
        });
    }
}
