use std::sync::Arc;

use eframe::{run_native, NativeOptions};
use egui::ViewportBuilder;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::channel;
use tokio::sync::RwLock;

use crate::app::App;
use crate::application::channel_events_handler::{
    start_handler_stream_added, start_handler_stream_removed,
};
use crate::application::connections::controller::ConnectionsController;
use crate::application::connections::listeners::view::ListenersView;
use crate::application::connections::streams::controller::StreamsController;
use crate::application::connections::streams::model::StreamsModel;
use crate::application::connections::streams::service::StreamsService;
use crate::application::connections::streams::view::StreamsView;
use crate::application::connections::view::ConnectionsView;
use crate::application::menu::controller::MenuController;
use crate::application::menu::view::MenuView;
use crate::application::model::ApplicationModel;
use crate::application::repaint_scheduler::RepaintScheduler;
use crate::application::service::ApplicationService;
use crate::application::view::ApplicationView;
use crate::communication::tcp_stream_container::TcpStreamContainer;

mod app;
pub mod application;
pub mod communication;

fn main() {
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1600.0, 900.0]),
        ..Default::default()
    };

    let runtime = Arc::new(Runtime::new().unwrap());
    let stream_container = TcpStreamContainer::new(runtime.clone());
    let (channel_added_tx, channel_added_rx) = channel(16);
    let (channel_removed_tx, channel_removed_rx) = channel(16);

    let repaint_scheduler = Arc::new(RepaintScheduler::default());

    let streams_model = Arc::new(RwLock::new(StreamsModel::new(stream_container.clone())));
    let application_model = Arc::new(RwLock::new(ApplicationModel::default()));
    let connections_model = Arc::new(RwLock::new(StreamsModel::new(stream_container.clone())));

    let application_service = Arc::new(ApplicationService {
        model: application_model.clone(),
        runtime: runtime.clone(),
    });
    let connections_service = Arc::new(StreamsService { stream_container });

    let streams_controller = Arc::new(StreamsController {
        model: connections_model.clone(),
        service: connections_service,
        application_service: application_service.clone(),
        runtime: runtime.clone(),
        repaint_scheduler: repaint_scheduler.clone(),
    });
    let connections_controller = Arc::new(ConnectionsController {
        application_service: application_service.clone(),
    });
    let menu_controller = Arc::new(MenuController {
        application_service,
        runtime: runtime.clone(),
    });

    let menu_view = Arc::new(MenuView {
        controller: menu_controller,
    });
    let streams_view = Arc::new(StreamsView {
        model: connections_model.clone(),
        controller: streams_controller.clone(),
    });
    let listeners_view = Arc::new(ListenersView {});
    let connections_view = Arc::new(ConnectionsView {
        controller: connections_controller,
        listeners_view,
        streams_view,
    });
    let application_view = Arc::new(ApplicationView {
        model: application_model.clone(),
        menu_view,
        connections_view,
    });

    let app = App {
        model: application_model,
        view: application_view,
        repaint_scheduler: repaint_scheduler.clone(),
    };

    start_handler_stream_added(runtime.clone(), channel_added_rx, repaint_scheduler.clone());
    start_handler_stream_removed(runtime.clone(), channel_removed_rx, repaint_scheduler);
    run_native("PTYS", options, Box::new(|_context| Box::new(app))).unwrap();
}
