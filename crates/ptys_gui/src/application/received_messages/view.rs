use crate::application::received_messages::controller::ReceivedMessagesController;
use crate::application::received_messages::model::ReceivedMessagesModel;
use crate::application::window_view::WindowView;
use eframe::Frame;
use egui::{Context, Ui};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ReceivedMessagesView {
    pub controller: ReceivedMessagesController,
    pub model: Arc<RwLock<ReceivedMessagesModel>>,
}

impl WindowView for ReceivedMessagesView {
    fn get_title(&self) -> &'static str {
        "Received messages"
    }

    fn display(&self, _context: &Context, _frame: &mut Frame, _ui: &mut Ui) {
        let model = self.model.blocking_read();

        model
            .messages
            .iter()
            .for_each(|message| todo!("{:?}", message));
    }
}
