use std::sync::Arc;

use tokio::runtime::Runtime;
use tokio::sync::mpsc::Receiver;

use crate::application::repaint_scheduler::RepaintScheduler;
use crate::communication::tcp_stream_container::StreamEntry;

pub fn start_handler_stream_added(
    runtime: Arc<Runtime>,
    mut rx: Receiver<Arc<StreamEntry>>,
    repaint_scheduler: Arc<RepaintScheduler>,
) {
    runtime.spawn(async move {
        loop {
            let Some(stream) = rx.recv().await else {
                // No further events can be received at this point.
                return;
            };

            println!("added {}, {:?}", stream.id, stream.parent_id);

            repaint_scheduler.schedule_now().await;
        }
    });
}

pub fn start_handler_stream_removed(
    runtime: Arc<Runtime>,
    mut rx: Receiver<Arc<StreamEntry>>,
    repaint_scheduler: Arc<RepaintScheduler>,
) {
    runtime.spawn(async move {
        loop {
            let Some(stream) = rx.recv().await else {
                // No further events can be received at this point.
                return;
            };

            println!("removed {}, {:?}", stream.id, stream.parent_id);

            repaint_scheduler.schedule_now().await;
        }
    });
}
