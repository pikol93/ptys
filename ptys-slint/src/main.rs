use std::sync::Arc;

use eyre::Result;
use ptys_network::Network;
use ptys_slint_components::{AddListenerState, ListenerEntry, ListenerListState, Ui};
use result_ext::CoreResultExt;
use slint::{ComponentHandle, Model, ModelExt, ModelRc, VecModel, Weak};
use tokio::runtime::Runtime;

mod result_ext;
mod slint_ext;

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
    let runtime_clone = runtime.clone();
    let network_clone = network.clone();
    ui.global::<AddListenerState>()
        .on_add_listener_pressed(move |value| {
            let ui_weak = ui_weak.clone();
            let network = network_clone.clone();
            runtime_clone.spawn(async move {
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

    ui.global::<ListenerListState>()
        .set_entries(ModelRc::new(VecModel::from(vec![])));
    let ui_weak = ui.as_weak();
    runtime.spawn(async move {
        let mut rx = network.get_listener_added_receiver();
        loop {
            let Ok(id) = rx.recv().await else {
                println!("Could not receive data.");
                break;
            };

            ui_weak
                .upgrade_in_event_loop(|ui| {
                    let entries = ui.global::<ListenerListState>().get_entries();
                    entries
                        .as_any()
                        .downcast_ref::<VecModel<ListenerEntry>>()
                        .unwrap()
                        .push(ListenerEntry {
                            id: 1,
                            state: "Disabled".into(),
                        });
                })
                .or_log_debug();
        }
    });
}

async fn add_and_update_listener_state(ui: Weak<Ui>, network: &Network, id: usize) {
    let network_state = network.get_listener_state(id);
    ui.upgrade_in_event_loop(|ui| {
        let entries = ui.global::<ListenerListState>().get_entries();
        let a = entries
        .as_any()
        .downcast_ref::<VecModel<ListenerEntry>>()
        .unwrap();
    }).or_log_debug();
}

async fn add_listener(network: Arc<Network>, port_string: &str) -> Result<usize> {
    let port = port_string.parse::<u16>()?;
    let id = network.add_listener(port).await;

    Ok(id)
}
