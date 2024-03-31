use std::sync::Arc;

use eframe::Frame;
use egui::{CentralPanel, Context};
use tokio::sync::RwLock;

use crate::application::model::ApplicationModel;
use crate::application::repaint_scheduler::RepaintScheduler;
use crate::application::view::ApplicationView;

pub struct App {
    pub model: Arc<RwLock<ApplicationModel>>,
    pub view: Arc<ApplicationView>,
    pub repaint_scheduler: Arc<RepaintScheduler>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.repaint_scheduler.set_context(ctx);

        CentralPanel::default().show(ctx, |ui| {
            self.view.display(ui);
        });
    }
}
