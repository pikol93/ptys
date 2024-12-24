use app::App;
use eyre::Result;
use iced::Task;
use ptys_network::Network;
use std::sync::Arc;
use tokio::runtime::Runtime;

mod app;

fn main() -> Result<()> {
    let runtime = Arc::new(Runtime::new()?);
    let network = Arc::new(Network::new(runtime.clone()));

    iced::application("PTYS", App::update, App::view)
        .run_with(move || (App::new(network), Task::none()))?;

    Ok(())
}
