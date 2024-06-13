use crate::predefined_attributes::{Code, ConstantValue, SourceFile, StackMapTable};

#[derive(Debug, Clone)]
pub enum Attribute {
    ConstantValue(ConstantValue),
    Code(Code),
    StackMapTable(StackMapTable),
    UserDefined(UserDefinedAttribute),
    SourceFile(SourceFile),
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