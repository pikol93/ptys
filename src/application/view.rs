use std::sync::Arc;

use egui::{Context, Ui};
use tokio::sync::RwLock;

use crate::application::connections::view::ConnectionsView;
use crate::application::menu::view::MenuView;
use crate::application::model::{ApplicationModel, DisplayedView};

pub struct ApplicationView {
    pub model: Arc<RwLock<ApplicationModel>>,
    pub menu_view: Arc<MenuView>,
    pub connections_view: Arc<ConnectionsView>,
}

impl ApplicationView {
    pub fn display(&self, context: &Context, ui: &mut Ui) {
        let displayed_view = self.model.blocking_write().displayed_view;
        match displayed_view {
            DisplayedView::Menu => self.menu_view.display(ui),
            DisplayedView::Connections => self.connections_view.display(context, ui),
        }
    }
}
