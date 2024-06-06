use crate::predefined_attributes::{ConstantValue, StackMapTable};

#[derive(Debug, Clone)]
pub enum Attribute {
    ConstantValue(ConstantValue),
    StackMapTable(StackMapTable),
    UserDefined(UserDefinedAttribute),
}

#[derive(Debug, Clone)]
pub struct UserDefinedAttribute {
    name: String,
    info: Vec<u8>,
}

impl UserDefinedAttribute {
    pub fn new(name: String, info: &[u8]) -> Self {
        UserDefinedAttribute { name, info: info.to_vec() }
    }
}