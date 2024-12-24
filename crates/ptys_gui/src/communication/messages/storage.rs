use bounded_vec_deque::BoundedVecDeque;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;

const MAX_CAPACITY: usize = 256;

pub struct Message {
    pub id: u32,
    pub source: u32,
    pub bytes: Vec<u8>,
}

pub struct MessageStorage {
    pub messages: RwLock<BoundedVecDeque<Arc<Message>>>,
    message_added: Sender<Arc<Message>>,
    message_removed: Sender<Arc<Message>>,
}

impl MessageStorage {
    pub fn new(message_added: Sender<Arc<Message>>, message_removed: Sender<Arc<Message>>) -> Self {
        Self {
            messages: RwLock::new(BoundedVecDeque::with_capacity(MAX_CAPACITY, MAX_CAPACITY)),
            message_added,
            message_removed,
        }
    }

    pub async fn insert_new(&self, message: Message) {
        let message = Arc::new(message);
        let popped_message = {
            let mut messages = self.messages.write().await;

            let popped_message = if messages.is_full() {
                messages.pop_back()
            } else {
                None
            };

            messages.push_front(message.clone());

            popped_message
        };

        if let Some(popped_message) = popped_message {
            self.message_removed.send(popped_message).await.unwrap();
        }

        self.message_added.send(message).await.unwrap();
    }
}
