use std::sync::Arc;

use eframe::{run_native, NativeOptions};
use egui::ViewportBuilder;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::channel;
use tokio::sync::RwLock;

use crate::application::add_listener::controller::AddListenerController;
use crate::application::add_listener::view::AddListenerView;
use crate::application::add_stream::controller::AddStreamController;
use crate::application::add_stream::view::AddStreamView;
use crate::application::application_information::view::ApplicationInformationView;
use crate::application::listeners::controller::ListenersController;
use crate::application::listeners::view::ListenersView;
use crate::application::repaint_scheduler::RepaintScheduler;
use crate::application::streams::controller::StreamsController;
use crate::application::streams::view::StreamsView;
use crate::communication::tcp_listener_container::TcpListenersContainer;
use crate::communication::tcp_stream_container::TcpStreamContainer;
use crate::listeners_events_handler::{
    start_handler_listener_added, start_handler_listener_removed,
};
use crate::streams_events_handler::{start_handler_stream_added, start_handler_stream_removed};
use application::app::App;

pub mod application;
pub mod communication;
mod listeners_events_handler;
pub mod streams_events_handler;

fn main() {
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1600.0, 900.0]),
        ..Default::default()
    };

    let runtime = Arc::new(Runtime::new().unwrap());
    let (listener_added_tx, listener_added_rx) = channel(16);
    let (listener_removed_tx, listener_removed_rx) = channel(16);
    let (stream_added_tx, stream_added_rx) = channel(16);
    let (stream_removed_tx, stream_removed_rx) = channel(16);
    let stream_container =
        TcpStreamContainer::new(runtime.clone(), stream_added_tx, stream_removed_tx);
    let listeners_container = TcpListenersContainer::new(
        stream_container.clone(),
        runtime.clone(),
        listener_added_tx,
        listener_removed_tx,
    );

    let repaint_scheduler = Arc::new(RepaintScheduler::default());

    let streams_model = Arc::new(RwLock::default());
    let listeners_model = Arc::new(RwLock::default());
    let add_listener_model = Arc::new(RwLock::default());
    let add_stream_model = Arc::new(RwLock::default());

    let streams_controller = Arc::new(StreamsController {
        model: streams_model.clone(),
        runtime: runtime.clone(),
        repaint_scheduler: repaint_scheduler.clone(),
        stream_container: stream_container.clone(),
    });
    let listeners_controller = ListenersController {
        model: listeners_model.clone(),
        listeners_container: listeners_container.clone(),
        runtime: runtime.clone(),
        repaint_scheduler: repaint_scheduler.clone(),
    };
    let add_listeners_controller = AddListenerController {
        model: add_listener_model.clone(),
        listeners_container: listeners_container.clone(),
        runtime: runtime.clone(),
        repaint_scheduler: repaint_scheduler.clone(),
    };
    let add_stream_controller = AddStreamController {
        model: add_stream_model.clone(),
        runtime: runtime.clone(),
        repaint_scheduler: repaint_scheduler.clone(),
        stream_container: stream_container.clone(),
    };

    let streams_view = Box::new(StreamsView {
        model: streams_model.clone(),
        controller: streams_controller.clone(),
    });
    let listeners_view = Box::new(ListenersView {
        model: listeners_model.clone(),
        controller: listeners_controller,
    });
    let add_listeners_view = Box::new(AddListenerView {
        model: add_listener_model,
        controller: add_listeners_controller,
    });
    let add_stream_view = Box::new(AddStreamView {
        model: add_stream_model,
        controller: add_stream_controller,
    });
    let application_information_view = Box::new(ApplicationInformationView {});

    let app = App::new(
        vec![
            streams_view,
            listeners_view,
            add_listeners_view,
            add_stream_view,
            application_information_view,
        ],
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
        repaint_scheduler.clone(),
        streams_model.clone(),
    );
    start_handler_listener_added(
        runtime.clone(),
        listener_added_rx,
        repaint_scheduler.clone(),
        listeners_model.clone(),
    );
    start_handler_listener_removed(
        runtime.clone(),
        listener_removed_rx,
        repaint_scheduler.clone(),
        listeners_model.clone(),
    );
    run_native("PTYS", options, Box::new(|_context| Box::new(app))).unwrap();
}
