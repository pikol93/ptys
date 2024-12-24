use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct MessageModel {
    pub id: u32,
    pub source: u32,
}

#[derive(Default)]
pub struct ReceivedMessagesModel {
    pub messages: VecDeque<MessageModel>,
}
