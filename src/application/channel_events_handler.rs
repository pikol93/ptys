use crate::application::connections::model::{ConnectionsModel, SingleConnectionModel};
use crate::application::repaint_scheduler::RepaintScheduler;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Receiver;
use tokio::sync::RwLock;

pub fn start_handler_channel_added(
    runtime: Arc<Runtime>,
    mut rx: Receiver<()>,
    connections_model: Arc<RwLock<ConnectionsModel>>,
    repaint_scheduler: Arc<RepaintScheduler>,
) {
    runtime.spawn(async move {
        loop {
            rx.recv().await;
            let model = &mut connections_model
                .write()
                .await
                .all_connections_model
                .connections;

            model.push(SingleConnectionModel {
                id: 0,
                hostname: "".to_string(),
                port: 0,
            });

            repaint_scheduler.schedule_now().await;
        }
    });
}
