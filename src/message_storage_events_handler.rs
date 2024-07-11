use std::sync::Arc;

use crate::application::received_messages::model::{MessageModel, ReceivedMessagesModel};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Receiver;
use tokio::sync::RwLock;

use crate::application::repaint_scheduler::RepaintScheduler;
use crate::communication::messages::storage::Message;

pub fn start_handler_message_added(
    runtime: Arc<Runtime>,
    mut rx: Receiver<Arc<Message>>,
    repaint_scheduler: Arc<RepaintScheduler>,
    model: Arc<RwLock<ReceivedMessagesModel>>,
) {
    runtime.spawn(async move {
        loop {
            let Some(message) = rx.recv().await else {
                break;
            };

            let messages = &mut model.write().await.messages;

            messages.push_front(MessageModel {
                id: message.id,
                source: message.source,
            });

            repaint_scheduler.schedule_now().await;
        }
    });
}

pub fn start_handler_message_removed(
    runtime: Arc<Runtime>,
    mut rx: Receiver<Arc<Message>>,
    repaint_scheduler: Arc<RepaintScheduler>,
    model: Arc<RwLock<ReceivedMessagesModel>>,
) {
    runtime.spawn(async move {
        loop {
            let Some(message) = rx.recv().await else {
                break;
            };

            let messages = &mut model.write().await.messages;
            let Some(index) = messages.iter().position(|message_model| message_model.id == message.id) else {
                println!("Could not find a message model by ID {}", message.id);
                continue;
            };

            messages.remove(index);

            repaint_scheduler.schedule_now().await;
        }
    });
}
