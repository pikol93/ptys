use std::sync::Arc;

use eframe::{run_native, NativeOptions};
use egui::ViewportBuilder;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::channel;
use tokio::sync::RwLock;

use crate::application::listeners::view::ListenersView;
use crate::application::repaint_scheduler::RepaintScheduler;
use crate::application::streams::controller::StreamsController;
use crate::application::streams::view::StreamsView;
use crate::channel_events_handler::{start_handler_stream_added, start_handler_stream_removed};
use crate::communication::tcp_stream_container::TcpStreamContainer;
use application::app::App;

pub mod application;
pub mod channel_events_handler;
pub mod communication;

fn main() {
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1600.0, 900.0]),
        ..Default::default()
    };

    let runtime = Arc::new(Runtime::new().unwrap());
    let (stream_added_tx, stream_added_rx) = channel(16);
    let (stream_removed_tx, stream_removed_rx) = channel(16);
    let stream_container =
        TcpStreamContainer::new(runtime.clone(), stream_added_tx, stream_removed_tx);

    let repaint_scheduler = Arc::new(RepaintScheduler::default());

    let streams_model = Arc::new(RwLock::default());

    let streams_controller = Arc::new(StreamsController {
        model: streams_model.clone(),
        runtime: runtime.clone(),
        repaint_scheduler: repaint_scheduler.clone(),
        stream_container,
    });

    let streams_view = Box::new(StreamsView {
        model: streams_model.clone(),
        controller: streams_controller.clone(),
    });
    let listeners_view = Box::new(ListenersView {});

    let app = App::new(
        vec![streams_view, listeners_view],
        repaint_scheduler.clone(),
    );

    start_handler_stream_added(
        runtime.clone(),
        stream_added_rx,
        repaint_scheduler.clone(),
        streams_model.clone(),
    );
    start_handler_stream_removed(
        runtime.clone(),
        stream_removed_rx,
        repaint_scheduler,
        streams_model.clone(),
    );
    run_native("PTYS", options, Box::new(|_context| Box::new(app))).unwrap();
}
