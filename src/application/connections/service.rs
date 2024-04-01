use crate::application::connections::model::ChannelType;

pub struct ConnectionsService {}

impl ConnectionsService {
    pub fn add_connection(
        &self,
        hostname: &str,
        port: u16,
        channel_type: ChannelType,
    ) -> anyhow::Result<()> {
        dbg!(hostname, port, channel_type);
        todo!()
    }
}
