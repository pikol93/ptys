use std::sync::{Arc, OnceLock};

use ptys_service::service::Service;
use tokio::runtime::Runtime;

static SERVICE: OnceLock<Service> = OnceLock::new();

pub fn get_service() -> &'static Service {
    SERVICE.get_or_init(|| {
        let runtime = Arc::new(Runtime::new().unwrap());
        Service::new(runtime)
    })
}
