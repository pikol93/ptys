use async_trait::async_trait;
use eyre::Result;

use crate::service::Service;

#[async_trait]
pub trait NetworkExt {
    async fn add_listener(&self, port: u16) -> usize;

    async fn start_listener(&self, id: usize) -> Result<()>;

    async fn subscribe_listener_added<T>(&self, callback: T)
    where
        T: Fn(usize) -> usize + Send + 'static;
}

#[async_trait]
impl NetworkExt for Service {
    async fn add_listener(&self, port: u16) -> usize {
        self.network.add_listener(port).await
    }

    async fn start_listener(&self, id: usize) -> Result<()> {
        self.network.start_listener(id).await
    }

    async fn subscribe_listener_added<T>(&self, callback: T)
    where
        T: Fn(usize) -> usize + Send + 'static,
    {
        let network = self.network.clone();
        self.runtime.spawn(async move {
            let mut rx = network.get_listener_added_receiver();
            loop {
                let Ok(id) = rx.recv().await else {
                    println!("\"Listener added\" receiver stopped working.");
                    break;
                };

                callback(id);
            }
        });
    }
}
