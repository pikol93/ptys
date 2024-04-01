use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum ChannelType {
    TcpServer,
    TcpClient,
}

#[derive(Debug)]
pub struct AddConnectionModel {
    pub hostname: String,
    pub port: String,
    pub channel_type: ChannelType,
    pub error: Option<String>,
}

impl Default for AddConnectionModel {
    fn default() -> Self {
        Self {
            hostname: "".to_string(),
            port: "".to_string(),
            channel_type: ChannelType::TcpServer,
            error: None,
        }
    }
}

#[derive(Debug)]
pub struct SingleConnectionModel {
    pub id: u64,
    pub hostname: String,
    pub port: u16,
}

#[derive(Default, Debug)]
pub struct AllConnectionsModel {
    pub connections: Vec<SingleConnectionModel>,
}

#[derive(Default, Debug)]
pub struct ConnectionsModel {
    pub add_connection_model: AddConnectionModel,
    pub all_connections_model: AllConnectionsModel,
}
