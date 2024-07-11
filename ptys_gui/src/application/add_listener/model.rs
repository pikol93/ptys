#[derive(Debug, Default)]
pub struct AddListenerModel {
    pub port: String,
    pub error: Option<String>,
}

impl AddListenerModel {
    pub fn reset(&mut self) {
        *self = AddListenerModel::default();
    }
}
