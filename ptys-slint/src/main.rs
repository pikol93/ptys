use std::sync::Arc;

use eyre::Result;
use ptys_network::Network;
use ptys_slint_components::{AddListenerState, Ui};
use result_ext::CoreResultExt;
use slint::ComponentHandle;
use tokio::runtime::Runtime;

mod result_ext;

fn main() -> Result<()> {
    let runtime = Arc::new(Runtime::new()?);
    let network = Arc::new(Network::new(runtime.clone()));
    let ui = Ui::new()?;
    init_add_listener_handler(&ui, runtime.clone(), network.clone());

    ui.run()?;

    Ok(())
}

fn init_add_listener_handler(ui: &Ui, runtime: Arc<Runtime>, network: Arc<Network>) {
    let ui_weak = ui.as_weak();
    ui.global::<AddListenerState>()
        .on_add_listener_pressed(move |value| {
            let ui_weak = ui_weak.clone();
            let network = network.clone();
            runtime.spawn(async move {
                let result = add_listener(network, &value)
                    .await
                    .map(|id| format!("Added listener with ID {}", id))
                    .unwrap_or_else(|error| format!("{}", error));

                ui_weak
                    .upgrade_in_event_loop(|ui| {
                        let state = ui.global::<AddListenerState>();
                        state.set_add_listener_result(result.into());
                    })
                    .or_log_debug();
            });
        });
}

async fn add_listener(network: Arc<Network>, port_string: &str) -> Result<usize> {
    let port = port_string.parse::<u16>()?;
    let id = network.add_listener(port).await;

    Ok(id)
}
