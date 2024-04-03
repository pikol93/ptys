#[derive(Debug)]
pub struct StreamModel {
    pub id: u32,
    pub parent_id: Option<u32>,
    pub port: u16,
}

#[derive(Debug, Default)]
pub struct StreamsModel {
    pub stream_models: Vec<StreamModel>,
}
