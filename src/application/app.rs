use std::sync::Arc;

use eframe::Frame;
use egui::{CentralPanel, Context, Window};

use crate::application::repaint_scheduler::RepaintScheduler;
use crate::application::window_view::WindowView;

pub struct App {
    pub views: Vec<Box<dyn WindowView>>,
    pub visibility: Vec<bool>,
    pub repaint_scheduler: Arc<RepaintScheduler>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.repaint_scheduler.set_context(ctx);

        CentralPanel::default().show(ctx, |ui| {
            self.views.iter().enumerate().for_each(|(index, view)| {
                Window::new(view.get_title())
                    .open(self.visibility.get_mut(index).unwrap())
                    .show(ctx, |ui| view.display(ctx, ui));
            });
        });
    }
}

impl App {
    pub fn new(views: Vec<Box<dyn WindowView>>, repaint_scheduler: Arc<RepaintScheduler>) -> Self {
        let visibility = vec![true; views.len()];
        Self {
            views,
            visibility,
            repaint_scheduler,
        }
    }
}
