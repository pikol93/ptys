use async_trait::async_trait;

use crate::service::Service;

#[async_trait]
pub trait NetworkExt {
    fn subscribe_listener_added<T>(&self, callback: T)
    where
        T: Fn(usize) + Send + Sync + 'static;

    fn subscribe_listener_removed<T>(&self, callback: T)
    where
        T: Fn(usize) + Send + Sync + 'static;
}

#[async_trait]
impl NetworkExt for Service {
    fn subscribe_listener_added<T>(&self, callback: T)
    where
        T: Fn(usize) + Send + Sync + 'static,
    {
        let network = self.network.clone();
        self.runtime.spawn(async move {
            let mut rx = network.subscribe_listener_added();
            loop {
                let Ok(id) = rx.recv().await else {
                    println!("\"Listener added\" receiver stopped working.");
                    break;
                };

                callback(id);
            }
        });
    }

    fn subscribe_listener_removed<T>(&self, callback: T)
    where
        T: Fn(usize) + Send + Sync + 'static,
    {
        let network = self.network.clone();
        self.runtime.spawn(async move {
            let mut rx = network.subscribe_listener_removed();
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
