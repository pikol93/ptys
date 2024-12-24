use std::sync::Arc;

use iced::{
    widget::{button, column, text, text_input, Column},
    Task,
};
use ptys_network::Network;

pub struct App {
    network: Arc<Network>,
    port_input: String,
    add_result: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    PortInput(String),
    ListenerAddPressed,
    FinishAdding,
    ListenerAddError(String),
}

impl App {
    pub fn new(network: Arc<Network>) -> Self {
        Self {
            network,
            port_input: "".to_string(),
            add_result: "".to_string(),
        }
    }

    pub fn view(&self) -> Column<Message> {
        column![
            text_input("Insert port", &self.port_input).on_input(Message::PortInput),
            button("+").on_press(Message::ListenerAddPressed),
            text!("{}", &self.add_result),
        ]
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ListenerAddPressed => {
                let port = match self.port_input.parse::<u16>() {
                    Ok(port) => port,
                    Err(error) => {
                        return Task::done(Message::ListenerAddError(format!("{}", error)));
                    }
                };

                let network = self.network.clone();
                Task::perform(async move { network.add_listener(port).await }, |_| {
                    Message::FinishAdding
                })
            }
            Message::FinishAdding => {
                self.add_result = "OK".to_string();
                Task::none()
            }
            Message::PortInput(value) => {
                self.port_input = value;
                Task::none()
            }
            Message::ListenerAddError(error) => {
                self.add_result = error;
                Task::none()
            }
        }
    }
}
