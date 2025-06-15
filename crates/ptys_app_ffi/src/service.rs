use std::sync::{Arc, OnceLock};

use ptys_service::service::Service;
use tokio::runtime::Runtime;

static RUNTIME: OnceLock<Arc<Runtime>> = OnceLock::new();
static SERVICE: OnceLock<Service> = OnceLock::new();

pub fn get_runtime() -> &'static Arc<Runtime> {
    RUNTIME.get_or_init(|| Arc::new(Runtime::new().unwrap()))
}

pub fn get_service() -> &'static Service {
    SERVICE.get_or_init(|| Service::new(get_runtime().clone()))
}
