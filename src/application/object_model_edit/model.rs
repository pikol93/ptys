use std::collections::HashMap;

use crate::communication::messages::model::{
    Data, DynamicData, Endianness, Length, ObjectModel, ValueData,
};

pub struct ObjectModelEditModel {
    pub edited_model: Option<ObjectModel>,
}

impl Default for ObjectModelEditModel {
    fn default() -> Self {
        let subtype = ObjectModel {
            name: "subtype".to_string(),
            data: Data::Value(ValueData {
                length: Length::Constant(1),
                endianness: Endianness::Little,
            }),
        };

        let subtype_a = ObjectModel {
            name: "subtype_a".to_string(),
            data: Data::Value(ValueData {
                length: Length::Constant(2),
                endianness: Endianness::Little,
            }),
        };

        let subtype_b_length = ObjectModel {
            name: "subtype_b_length".to_string(),
            data: Data::Value(ValueData {
                length: Length::Constant(1),
                endianness: Endianness::Little,
            }),
        };

        let subtype_b_data = ObjectModel {
            name: "subtype_b_data".to_string(),
            data: Data::Value(ValueData {
                length: Length::Variable("subtype_b_length".to_owned()),
                endianness: Endianness::Little,
            }),
        };

        let subtype_b = ObjectModel {
            name: "subtype_b".to_string(),
            data: Data::Parent(vec![subtype_b_length, subtype_b_data]),
        };

        let sub_message = ObjectModel {
            name: "sub_message".to_string(),
            data: Data::Dynamic(DynamicData {
                subtype_reference_name: "subtype".to_string(),
                subtypes: HashMap::from([(0x12, Box::new(subtype_a)), (0x34, Box::new(subtype_b))]),
            }),
        };

        let obj = ObjectModel {
            name: "message".to_string(),
            data: Data::Parent(vec![subtype, sub_message]),
        };

        Self {
            edited_model: Some(obj),
        }
    }
}
