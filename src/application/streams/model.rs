#[derive(Debug)]
pub struct AddStreamModel {
    pub hostname: String,
    pub port: String,
    pub error: Option<String>,
}

impl Default for AddStreamModel {
    fn default() -> Self {
        Self {
            hostname: "".to_string(),
            port: "".to_string(),
            error: None,
        }
    }
}

#[derive(Debug)]
pub struct StreamModel {
    pub id: u32,
    pub parent_id: Option<u32>,
    pub port: u16,
}

#[derive(Debug, Default)]
pub struct StreamsModel {
    pub add_connection_model: AddStreamModel,
    pub stream_models: Vec<StreamModel>,
}
