use crate::communication::tcp_stream_container::TcpStreamContainer;

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

#[derive(Debug)]
pub struct StreamsModel {
    pub add_connection_model: AddStreamModel,
    stream_container: TcpStreamContainer,
}

impl StreamsModel {
    pub fn new(stream_container: TcpStreamContainer) -> Self {
        Self {
            add_connection_model: Default::default(),
            stream_container,
        }
    }

    pub fn get_connection_models(&self) -> Vec<StreamModel> {
        self.stream_container
            .streams
            .blocking_read()
            .iter()
            .map(|entry| StreamModel {
                id: entry.id,
                parent_id: entry.parent_id,
                port: 0,
            })
            .collect()
    }
}
