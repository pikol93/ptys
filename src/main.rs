use std::sync::Arc;

use eframe::{run_native, NativeOptions};
use egui::ViewportBuilder;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::channel;
use tokio::sync::RwLock;

use crate::app::App;
use crate::application::connections::controller::ConnectionsController;
use crate::application::connections::model::ConnectionsModel;
use crate::application::connections::service::ConnectionsService;
use crate::application::connections::view::ConnectionsView;
use crate::application::menu::controller::MenuController;
use crate::application::menu::view::MenuView;
use crate::application::model::ApplicationModel;
use crate::application::repaint_scheduler::RepaintScheduler;
use crate::application::service::ApplicationService;
use crate::application::view::ApplicationView;
use crate::communication::channel_container::ChannelContainer;

mod app;
pub mod application;
mod communication;

fn main() {
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1600.0, 900.0]),
        ..Default::default()
    };

    let runtime = Arc::new(Runtime::new().unwrap());
    let (channel_added_tx, channel_added_rx) = channel(16);
    let (channel_removed_tx, channel_removed_rx) = channel(16);
    let channel_container = Arc::new(RwLock::new(ChannelContainer::new(
        runtime.clone(),
        channel_added_tx,
        channel_removed_tx,
    )));

    let repaint_scheduler = Arc::new(RepaintScheduler::default());

    let application_model = Arc::new(RwLock::new(ApplicationModel::default()));
    let connections_model = Arc::new(RwLock::new(ConnectionsModel::default()));

    let application_service = Arc::new(ApplicationService {
        model: application_model.clone(),
        runtime: runtime.clone(),
    });
    let connections_service = Arc::new(ConnectionsService { channel_container });

    let connections_controller = Arc::new(ConnectionsController {
        model: connections_model.clone(),
        service: connections_service,
        application_service: application_service.clone(),
        runtime: runtime.clone(),
        repaint_scheduler: repaint_scheduler.clone(),
    });
    let menu_controller = Arc::new(MenuController {
        application_service,
        runtime,
    });

    let menu_view = Arc::new(MenuView {
        controller: menu_controller,
    });
    let connections_view = Arc::new(ConnectionsView {
        model: connections_model,
        controller: connections_controller,
    });
    let application_view = Arc::new(ApplicationView {
        model: application_model.clone(),
        menu_view,
        connections_view,
    });

    let app = App {
        model: application_model,
        view: application_view,
        repaint_scheduler,
    };

    run_native("PTYS", options, Box::new(|_context| Box::new(app))).unwrap();
}
