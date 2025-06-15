use std::collections::HashMap;

use num_bigint::BigInt;
use serde::{Deserialize, Serialize};

use crate::{field::Field, values::{Node, Value}};
use color_eyre::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelNode {
    pub name: String,
    pub node: FieldNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldNode {
    Data(Field),
    
}

#[derive(Default)]
struct Registry<'a> {
    data: HashMap<&'a str, &'a BigInt>,
    parent: Option<&'a Registry<'a>>,
}

impl ModelNode {
    pub fn decode(&self, buffer: &[u8]) -> Result<crate::values::Node> {
        let mut registry = Registry::default();
        self.do_decode(buffer, &mut registry)
    }

    fn do_decode<'a, 'b: 'a>(&'a self, buffer: &[u8], registry: &'b mut Registry<'a>) -> Result<Node> {
        match &self.node {
            FieldNode::Data(field) => {
                let value = field.decode(buffer)?;
                let node = crate::values::Node { name: self.name.clone(), value: Value::Data(value) };
                Ok(node)
            }
        }
    }
}

impl<'a> Registry<'a> {
    pub fn get(&self, key: &str) {

    }

    pub fn insert(&'a mut self, key: &'a str, value: &'a BigInt) {
        self.data.insert(key, value);
    }
}
