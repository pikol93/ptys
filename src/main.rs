use std::sync::Arc;

use eframe::{run_native, NativeOptions};
use egui::ViewportBuilder;
use tokio::runtime::Runtime;
use tokio::sync::RwLock;

use crate::app::App;
use crate::application::connections::controller::ConnectionsController;
use crate::application::connections::model::ConnectionsModel;
use crate::application::connections::view::ConnectionsView;
use crate::application::menu::controller::MenuController;
use crate::application::menu::view::MenuView;
use crate::application::model::ApplicationModel;
use crate::application::service::ApplicationService;
use crate::application::view::ApplicationView;

mod app;
pub mod application;

fn main() {
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1600.0, 900.0]),
        ..Default::default()
    };

    let runtime = Arc::new(Runtime::new().unwrap());

    let application_model = Arc::new(RwLock::new(ApplicationModel::default()));
    let connections_model = Arc::new(RwLock::new(ConnectionsModel::default()));

    let application_service = Arc::new(ApplicationService {
        model: application_model.clone(),
        runtime: runtime.clone(),
    });

    let connections_controller = Arc::new(ConnectionsController {
        application_service: application_service.clone(),
        runtime: runtime.clone(),
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
    };

    run_native("PTYS", options, Box::new(|_context| Box::new(app))).unwrap();
}
