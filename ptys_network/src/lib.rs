use std::{ops::Deref, sync::Arc};

use channel::container::ChannelsContainer;
use listener::container::{EventClientAccepted, ListenersContainer};
use ptys_common::extension::sender::SenderExt;
use tokio::runtime::Runtime;

pub mod channel;
pub mod listener;

pub struct Network {
    pub channels: Arc<ChannelsContainer>,
    pub listeners: Arc<ListenersContainer>,
    runtime: Arc<Runtime>,
}

impl Network {
    pub fn initialize_arc(runtime: Arc<Runtime>) -> Self {
        let this = Self {
            channels: Arc::new(ChannelsContainer::new()),
            listeners: Arc::new(ListenersContainer::new(runtime.clone())),
            runtime,
        };

        this.register_events();
        this
    }

    fn register_events(&self) {
        let runtime = self.runtime.deref();
        self.listeners
            .event_client_accepted
            .event_subscribe(runtime, {
                let runtime = self.runtime.clone();
                let channels = self.channels.clone();
                move |EventClientAccepted { channel, .. }| {
                    channels.insert(runtime.deref(), channel);
                }
            });
    }
}
