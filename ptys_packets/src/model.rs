use std::collections::HashMap;

use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

use crate::big_uint::big_uint_to_usize;
use crate::values::{Node, Value};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Endianness {
    Little,
    Big,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Length {
    Constant(usize),
    Variable(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicData {
    pub subtype_reference_name: String,
    pub subtypes: HashMap<usize, Box<ObjectModel>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Data {
    Value(ValueData),
    Parent(Vec<ObjectModel>),
    Dynamic(DynamicData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueData {
    pub length: Length,
    pub endianness: Endianness,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectModel {
    pub name: String,
    pub data: Data,
}

impl ObjectModel {
    pub fn decode<'a>(
        &self,
        buffer: &'a [u8],
        registry: &mut HashMap<String, BigUint>,
    ) -> (Node, &'a [u8]) {
        println!("Decoding field: {}", self.name);
        let (value, right_buffer) = match &self.data {
            Data::Value(data_type) => {
                let (big_int, buffer) = data_type.decode(buffer, registry);

                // TODO: These should not be cloned
                registry.insert(self.name.clone(), big_int.clone());

                (Value::Data(big_int), buffer)
            }
            Data::Parent(children) => {
                let mut buffer = buffer;
                let mut key_values = Vec::with_capacity(children.len());
                for information in children {
                    let (key_value, new_buffer) = information.decode(buffer, registry);
                    buffer = new_buffer;
                    key_values.push(key_value);
                }

                (Value::Children(key_values), buffer)
            }
            Data::Dynamic(dynamic_data) => {
                let subtype_value_big_uint =
                    registry.get(&dynamic_data.subtype_reference_name).unwrap();
                let subtype_value = big_uint_to_usize(subtype_value_big_uint).unwrap();
                let field_information = dynamic_data.subtypes.get(&subtype_value).unwrap().as_ref();
                let (key_value, buffer) = field_information.decode(buffer, registry);

                (key_value.value, buffer)
            }
        };

        (
            Node {
                name: self.name.clone(),
                value,
            },
            right_buffer,
        )
    }
}

impl ValueData {
    pub fn decode<'a>(
        &self,
        buffer: &'a [u8],
        registry: &HashMap<String, BigUint>,
    ) -> (BigUint, &'a [u8]) {
        let length = match &self.length {
            Length::Constant(size) => *size,
            Length::Variable(name) => big_uint_to_usize(registry.get(name).unwrap()).unwrap(),
        };

        let (left, right) = buffer.split_at(length);
        let bit_uint = match self.endianness {
            Endianness::Little => BigUint::from_bytes_le(left),
            Endianness::Big => BigUint::from_bytes_be(left),
        };

        (bit_uint, right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_test_simple_structure() {
        let length_information = ObjectModel {
            name: "length".to_string(),
            data: Data::Value(ValueData {
                length: Length::Constant(1),
                endianness: Endianness::Little,
            }),
        };

        let bytes_information = ObjectModel {
            name: "bytes".to_string(),
            data: Data::Value(ValueData {
                length: Length::Variable("length".to_string()),
                endianness: Endianness::Little,
            }),
        };

        let message_information = ObjectModel {
            name: "message_a".to_string(),
            data: Data::Parent(vec![
                length_information,
                bytes_information.clone(),
                bytes_information,
            ]),
        };

        let buffer = [2u8, 0x12, 0x34, 0x56, 0x78];

        let mut registry = HashMap::new();
        let (result, right_buffer) = message_information.decode(&buffer, &mut registry);
        println!("Buffer: {:02X?}, result:\n{:#?}", right_buffer, result);
        println!("Registry: {:?}", registry);
    }

    #[test]
    fn decode_test_simple_dynamic_structure_variant_a() {
        let message_information = get_message_field_information();
        let buffer = [0x12, 0x01, 0x01];
        let mut registry = HashMap::new();
        let (key_value, right) = message_information.decode(&buffer, &mut registry);

        dbg!(key_value, right);
    }

    #[test]
    fn decode_test_simple_dynamic_structure_variant_b() {
        let message_information = get_message_field_information();
        let buffer = [0x34, 0x04, 0x01, 0x00, 0x00, 0x01];
        let mut registry = HashMap::new();
        let (key_value, right) = message_information.decode(&buffer, &mut registry);

        dbg!(key_value, right);
    }

    #[test]
    fn serialize() {
        let object_model = get_message_field_information();
        let json = serde_json::to_string(&object_model).unwrap();
        println!("{}", json);
    }

    fn get_message_field_information() -> ObjectModel {
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

        ObjectModel {
            name: "message".to_string(),
            data: Data::Parent(vec![subtype, sub_message]),
        }
    }
}
