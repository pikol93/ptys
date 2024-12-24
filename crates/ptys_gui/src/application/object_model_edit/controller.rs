use std::sync::Arc;

use tokio::runtime::Runtime;
use tokio::sync::RwLock;

use crate::application::object_model_edit::model::ObjectModelEditModel;

pub struct ObjectModelEditController {
    pub model: Arc<RwLock<ObjectModelEditModel>>,
    pub runtime: Arc<Runtime>,
}

impl ObjectModelEditController {}
