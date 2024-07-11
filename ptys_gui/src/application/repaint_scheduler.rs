use egui::Context;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct RepaintScheduler {
    context: RwLock<Option<Context>>,
}

impl RepaintScheduler {
    pub async fn schedule_now(&self) {
        let context_lock = self.context.read().await;

        if let Some(context) = context_lock.as_ref() {
            context.request_repaint();
        } else {
            println!("Could not request repaint.");
        };
    }

    pub fn set_context(&self, context: &Context) {
        {
            let context_lock = self.context.blocking_read();
            if context_lock.is_some() {
                return;
            }
        }

        let mut context_lock = self.context.blocking_write();
        *context_lock = Some(context.clone());
    }
}
