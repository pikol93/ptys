use std::sync::Arc;

use egui::{Context, RichText, Ui, Window};

use crate::application::connections::controller::ConnectionsController;
use crate::application::connections::listeners::view::ListenersView;
use crate::application::connections::streams::view::StreamsView;

pub struct ConnectionsView {
    pub controller: Arc<ConnectionsController>,
    pub listeners_view: Arc<ListenersView>,
    pub streams_view: Arc<StreamsView>,
}

impl ConnectionsView {
    pub fn display(&self, _context: &Context, ui: &mut Ui) {
        let mut open = true;
        Window::new("connections")
            .open(&mut open)
            .default_height(500.0)
            .show(_context, |ui| {
                ui.label(RichText::new("PTYS connections").size(22.0));
                if ui.button("Back").clicked() {
                    self.controller.button_clicked_back();
                }
            });
    }
}
