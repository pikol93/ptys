#[derive(Debug)]
pub struct ListenerModel {
    pub id: u32,
    pub port: u16,
}

#[derive(Debug, Default)]
pub struct ListenersModel {
    pub listeners_models: Vec<ListenerModel>,
}
