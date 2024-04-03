use crate::application::listeners::model::{ListenerModel, ListenersModel};
use crate::application::repaint_scheduler::RepaintScheduler;
use crate::communication::tcp_listener_container::ListenerEntry;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Receiver;
use tokio::sync::RwLock;

pub fn start_handler_listener_added(
    runtime: Arc<Runtime>,
    mut rx: Receiver<Arc<ListenerEntry>>,
    repaint_scheduler: Arc<RepaintScheduler>,
    listeners_model: Arc<RwLock<ListenersModel>>,
) {
    runtime.spawn(async move {
        loop {
            let Some(entry) = rx.recv().await else {
                // No further events can be received at this point
                return;
            };

            let listener_model = ListenerModel {
                id: entry.id,
                // Insert valid port
                port: 4321,
            };

            let mut model = listeners_model.write().await;
            model.listeners_models.push(listener_model);

            repaint_scheduler.schedule_now().await;
        }
    });
}

pub fn start_handler_listener_removed(
    runtime: Arc<Runtime>,
    mut rx: Receiver<Arc<ListenerEntry>>,
    repaint_scheduler: Arc<RepaintScheduler>,
    listeners_model: Arc<RwLock<ListenersModel>>,
) {
    runtime.spawn(async move {
        loop {
            let Some(listener) = rx.recv().await else {
                // No further events can be received at this point.
                return;
            };

            let mut model = listeners_model.write().await;
            let Some(index) = model
                .listeners_models
                .iter()
                .position(|item| item.id == listener.id)
            else {
                break;
            };

            model.listeners_models.remove(index);

            repaint_scheduler.schedule_now().await;
        }
    });
}
