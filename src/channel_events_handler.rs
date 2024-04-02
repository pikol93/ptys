use std::sync::Arc;

use tokio::runtime::Runtime;
use tokio::sync::mpsc::Receiver;

use crate::application::repaint_scheduler::RepaintScheduler;

pub fn start_handler_stream_added(
    runtime: Arc<Runtime>,
    mut rx: Receiver<()>,
    repaint_scheduler: Arc<RepaintScheduler>,
) {
    runtime.spawn(async move {
        loop {
            let Some(_) = rx.recv().await else {
                // No further events can be received at this point.
                return;
            };

            repaint_scheduler.schedule_now().await;
        }
    });
}

pub fn start_handler_stream_removed(
    runtime: Arc<Runtime>,
    mut rx: Receiver<()>,
    repaint_scheduler: Arc<RepaintScheduler>,
) {
    runtime.spawn(async move {
        loop {
            let Some(_) = rx.recv().await else {
                // No further events can be received at this point.
                return;
            };

            repaint_scheduler.schedule_now().await;
        }
    });
}
