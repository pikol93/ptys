#[derive(Default, Debug)]
pub struct AddStreamModel {
    pub hostname: String,
    pub port: String,
    pub error: Option<String>,
}

impl AddStreamModel {
    pub fn reset(&mut self) {
        *self = AddStreamModel::default();
    }
}
