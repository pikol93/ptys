use crate::communication::streams::stream_entry::StreamEntry;
use std::sync::Arc;

pub struct ReceivedDataEvent {
    pub entry: Arc<StreamEntry>,
    pub data: Vec<u8>,
}
