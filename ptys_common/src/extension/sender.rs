use std::{any::type_name, borrow::Borrow};

use log::{debug, error};
use tokio::{
    runtime::Runtime,
    sync::broadcast::{error::RecvError, Sender},
};

pub trait SenderExt<T>
where
    T: Clone + Send + 'static,
{
    fn event_subscribe<R, L>(&self, runtime: R, on_event: L)
    where
        R: Borrow<Runtime>,
        L: Fn(T) + Send + 'static;
}

impl<T> SenderExt<T> for Sender<T>
where
    T: Clone + Send + 'static,
{
    fn event_subscribe<R, L>(&self, runtime: R, on_event: L)
    where
        R: Borrow<Runtime>,
        L: Fn(T) + Send + 'static,
    {
        let mut rx = self.subscribe();
        runtime.borrow().spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(item) => on_event(item),
                    Err(RecvError::Lagged(skipped_amount)) => {
                        error!(
                            "A receiver lagged. Skipped {} events. Event type name = {:?}.",
                            skipped_amount,
                            type_name::<T>()
                        )
                    }
                    Err(RecvError::Closed) => {
                        debug!("Sender closed. Finishing up subscribed event.");
                        break;
                    }
                }
            }
        });
    }
}
