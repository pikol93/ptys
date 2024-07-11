use std::sync::Arc;

use eframe::Frame;
use egui::{CollapsingHeader, Context, Ui};
use ptys_packets::model::{Data, DynamicData, ObjectModel, ValueData};
use tokio::sync::RwLock;

use crate::application::object_model_edit::controller::ObjectModelEditController;
use crate::application::object_model_edit::model::ObjectModelEditModel;
use crate::application::window_view::WindowView;

pub struct ObjectModelEditView {
    pub model: Arc<RwLock<ObjectModelEditModel>>,
    pub controller: ObjectModelEditController,
}

impl WindowView for ObjectModelEditView {
    fn get_title(&self) -> &'static str {
        "Edit object model"
    }

    fn display(&self, _context: &Context, _frame: &mut Frame, ui: &mut Ui) {
        let Some(model) = &mut self.model.blocking_write().edited_model else {
            return;
        };

        Self::display_object_model(ui, model);
    }
}

impl ObjectModelEditView {
    fn display_object_model(ui: &mut Ui, object_model: &mut ObjectModel) {
        CollapsingHeader::new(&object_model.name)
            .show_background(true)
            .show(ui, |ui| {
                ui.label(format!("Name: {}", object_model.name));
                ui.vertical(|ui| match &mut object_model.data {
                    Data::Value(value) => {
                        Self::display_value(ui, value);
                    }
                    Data::Parent(children) => {
                        Self::display_children(ui, children);
                    }
                    Data::Dynamic(dynamic_data) => {
                        Self::display_dynamic(ui, dynamic_data);
                    }
                });
            });
    }

    fn display_value(ui: &mut Ui, data: &mut ValueData) {
        ui.label(format!("endianness: {:?}", data.endianness));
        ui.label(format!("length: {:?}", data.length));
    }

    fn display_children(ui: &mut Ui, children: &mut [ObjectModel]) {
        children
            .iter_mut()
            .for_each(|child| Self::display_object_model(ui, child));
    }

    fn display_dynamic(ui: &mut Ui, dynamic_data: &mut DynamicData) {
        ui.label(format!(
            "subtype value ref name: {}",
            dynamic_data.subtype_reference_name
        ));
        dynamic_data
            .subtypes
            .iter_mut()
            .for_each(|(subtype_value, object_model)| {
                ui.label(format!("Subtype value: {}", subtype_value));
                Self::display_object_model(ui, object_model);
            });
    }
}
