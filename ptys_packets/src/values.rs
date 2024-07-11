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
