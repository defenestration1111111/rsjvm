use crate::predefined_attributes::{Code, ConstantValue, NestMembers, SourceFile, StackMapTable};
use derive_more::From;

#[derive(Debug, Clone, From)]
pub enum Attribute {
    ConstantValue(ConstantValue),
    Code(Code),
    StackMapTable(StackMapTable),
    NestMembers(NestMembers),
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