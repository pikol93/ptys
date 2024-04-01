use crate::application::connections::model::{ConnectionsModel, SingleConnectionModel};
use crate::application::repaint_scheduler::RepaintScheduler;
use crate::communication::network_channel::NetworkChannel;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Receiver;
use tokio::sync::RwLock;

pub fn start_handler_channel_added(
    runtime: Arc<Runtime>,
    mut rx: Receiver<Arc<NetworkChannel>>,
    connections_model: Arc<RwLock<ConnectionsModel>>,
    repaint_scheduler: Arc<RepaintScheduler>,
) {
    runtime.spawn(async move {
        loop {
            let Some(network_channel) = rx.recv().await else {
                // No further events can be received at this point.
                return;
            };

            let model = &mut connections_model
                .write()
                .await
                .all_connections_model
                .connections;

            model.push(SingleConnectionModel {
                id: network_channel.id(),
                channel_type: network_channel.channel_type(),
                hostname: network_channel.hostname().to_string(),
                port: network_channel.port(),
            });

            repaint_scheduler.schedule_now().await;
        }
    });
}
