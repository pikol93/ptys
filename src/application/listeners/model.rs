#[derive(Debug)]
pub struct AddListenerModel {
    pub port: String,
    pub error: Option<String>,
}

impl Default for AddListenerModel {
    fn default() -> Self {
        Self {
            port: "".to_string(),
            error: None,
        }
    }
}

#[derive(Debug)]
pub struct ListenerModel {
    pub id: u32,
    pub port: u16,
}

#[derive(Debug, Default)]
pub struct ListenersModel {
    pub add_connection_model: AddListenerModel,
    pub stream_models: Vec<ListenerModel>,
}
