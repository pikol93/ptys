#[derive(Debug)]
pub struct StreamModel {
    pub id: u32,
    pub parent_id: Option<u32>,
    pub port: u16,
    pub received_bytes_amount: u64,
}

impl StreamModel {
    pub fn new(id: u32, parent_id: Option<u32>, port: u16) -> Self {
        Self {
            id,
            parent_id,
            port,
            received_bytes_amount: 0,
        }
    }
}

#[derive(Debug, Default)]
pub struct StreamsModel {
    pub stream_models: Vec<StreamModel>,
}

impl StreamsModel {
    pub fn find_index_by_id(&self, id: u32) -> Option<usize> {
        self.stream_models.iter().position(|entry| entry.id == id)
    }

    pub fn find_mut_by_id(&mut self, id: u32) -> Option<&mut StreamModel> {
        self.find_index_by_id(id)
            .map(|index| self.stream_models.get_mut(index).unwrap())
    }
}
