use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, RwLock,
};

use tokio::{runtime::Runtime, sync::broadcast::Sender};

use ptys_common::extension::sender::SenderExt;

use super::{Channel, ChannelId};

#[derive(Clone)]
pub struct EventChannelInserted {
    pub id: ChannelId,
    pub channel: Arc<dyn Channel>,
}

#[derive(Clone)]
pub struct EventChannelRemoved {
    pub id: ChannelId,
    pub channel: Arc<dyn Channel>,
}

#[derive(Clone)]
pub struct EventBytesReceived {
    pub id: ChannelId,
    pub channel: Arc<dyn Channel>,
    pub bytes: Arc<[u8]>,
}

#[derive(Clone)]
pub struct EventBytesSent {
    pub id: ChannelId,
    pub channel: Arc<dyn Channel>,
    pub bytes: Arc<[u8]>,
}

pub struct ChannelsContainer {
    pub event_bytes_received: Sender<EventBytesReceived>,
    pub event_bytes_sent: Sender<EventBytesSent>,
    channels: RwLock<Vec<(ChannelId, Arc<dyn Channel>)>>,
    channel_id_counter: AtomicUsize,
}

impl ChannelsContainer {
    pub fn new() -> Self {
        Self {
            event_bytes_received: Sender::new(16),
            event_bytes_sent: Sender::new(16),
            channels: Default::default(),
            channel_id_counter: Default::default(),
        }
    }

    pub fn insert(&self, runtime: &Runtime, channel: Arc<dyn Channel>) {
        let id = self.channel_id_counter.fetch_add(1, Ordering::AcqRel);
        let id = ChannelId(id);
        self.channels.write().unwrap().push((id, channel.clone()));

        runtime.spawn({
            let channel = channel.clone();
            async move {
                channel.work_reader().await.unwrap();
            }
        });

        channel.event_bytes_received().event_subscribe(runtime, {
            let channel = channel.clone();
            let sender = self.event_bytes_received.clone();
            move |super::EventBytesReceived { bytes }| {
                let channel = channel.clone();
                let _ = sender.send(EventBytesReceived { id, channel, bytes });
            }
        });

        channel.event_bytes_sent().event_subscribe(runtime, {
            let channel = channel.clone();
            let sender = self.event_bytes_sent.clone();
            move |super::EventBytesSent { bytes }| {
                let channel = channel.clone();
                let _ = sender.send(EventBytesSent { id, channel, bytes });
            }
        });
    }
}
