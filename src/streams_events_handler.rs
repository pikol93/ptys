use std::sync::Arc;

use tokio::runtime::Runtime;
use tokio::sync::mpsc::Receiver;
use tokio::sync::RwLock;

use crate::application::repaint_scheduler::RepaintScheduler;
use crate::application::streams::model::{StreamModel, StreamsModel};
use crate::communication::streams::stream_entry::StreamEntry;

pub fn start_handler_stream_added(
    runtime: Arc<Runtime>,
    mut rx: Receiver<Arc<StreamEntry>>,
    repaint_scheduler: Arc<RepaintScheduler>,
    streams_model: Arc<RwLock<StreamsModel>>,
) {
    runtime.spawn(async move {
        loop {
            let Some(stream) = rx.recv().await else {
                // No further events can be received at this point.
                return;
            };

            let stream_model = StreamModel {
                id: stream.id,
                parent_id: stream.parent_id,
                port: 1234,
            };

            let mut model = streams_model.write().await;
            model.stream_models.push(stream_model);

            repaint_scheduler.schedule_now().await;
        }
    });
}

pub fn start_handler_stream_removed(
    runtime: Arc<Runtime>,
    mut rx: Receiver<Arc<StreamEntry>>,
    repaint_scheduler: Arc<RepaintScheduler>,
    streams_model: Arc<RwLock<StreamsModel>>,
) {
    runtime.spawn(async move {
        loop {
            let Some(stream) = rx.recv().await else {
                // No further events can be received at this point.
                return;
            };

            let mut model = streams_model.write().await;
            let Some(index) = model
                .stream_models
                .iter()
                .position(|item| item.id == stream.id)
            else {
                break;
            };

            model.stream_models.remove(index);

            repaint_scheduler.schedule_now().await;
        }
    });
}
