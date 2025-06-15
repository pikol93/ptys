use num_bigint::BigUint;

#[derive(Debug)]
pub enum Value {
    Children(Vec<Node>),
    Data(BigUint),
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub value: Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn foo() {
        let ab = Node {
            name: "ab".to_string(),
            value: Value::Children(vec![
                Node {
                    name: "ah".to_string(),
                    value: Value::Children(vec![
                        Node {
                            name: "ht".to_string(),
                            value: Value::Data(BigUint::from(0u32)),
                        },
                        Node {
                            name: "mt".to_string(),
                            value: Value::Data(BigUint::from(1u32)),
                        },
                    ]),
                },
                Node {
                    name: "xs".to_string(),
                    value: Value::Children(vec![]),
                },
            ]),
        };

        dbg!(ab);
    }
}
