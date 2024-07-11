use std::sync::Arc;

use tokio::runtime::Runtime;
use tokio::sync::mpsc::Receiver;
use tokio::sync::RwLock;

use crate::application::repaint_scheduler::RepaintScheduler;
use crate::application::streams::model::{StreamModel, StreamsModel};
use crate::communication::streams::event_types::ReceivedDataEvent;
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

            let stream_model = StreamModel::new(stream.id, stream.parent_id, 1234);

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
            let Some(index) = model.find_index_by_id(stream.id) else {
                break;
            };

            model.stream_models.remove(index);

            repaint_scheduler.schedule_now().await;
        }
    });
}

pub fn start_handler_stream_data_received(
    runtime: Arc<Runtime>,
    mut rx: Receiver<ReceivedDataEvent>,
    repaint_scheduler: Arc<RepaintScheduler>,
    streams_model: Arc<RwLock<StreamsModel>>,
) {
    runtime.spawn(async move {
        loop {
            let Some(event) = rx.recv().await else {
                return;
            };

            let mut model = streams_model.write().await;
            let Some(stream) = model.find_mut_by_id(event.entry.id) else {
                break;
            };

            stream.received_bytes_amount += event.data.len() as u64;
            stream.most_recent_bytes = event.data;

            repaint_scheduler.schedule_now().await;
        }
    });
}
