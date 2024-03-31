use std::sync::Arc;

use egui::Context;
use tokio::sync::RwLock;

#[derive(Clone, Default)]
pub struct RepaintScheduler {
    context: Arc<RwLock<Option<Context>>>,
}

impl RepaintScheduler {
    pub async fn schedule_now(&self) {
        let context_lock = self.context.read().await;

        if let Some(context) = &*context_lock {
            context.request_repaint();
        };
    }

    pub async fn set_context(&self, context: &Context) {
        {
            let context_lock = self.context.read().await;
            if context_lock.is_none() {
                return;
            }
        }

        let mut context_lock = self.context.write().await;
        *context_lock = Some(context.clone());
    }
}
