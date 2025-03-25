use derive_more::From;

use crate::predefined_attributes::{
    BootstrapMethods, Code, ConstantValue, LineNumberTable, LocalVariableTable,
    LocalVariableTypeTable, NestHost, NestMembers, PetrmittedSubclasses, SourceFile, StackMapTable,
};

#[derive(Debug, Clone, From, PartialEq)]
pub enum Attribute {
    ConstantValue(ConstantValue),
    Code(Code),
    StackMapTable(StackMapTable),
    LineNumberTable(LineNumberTable),
    LocalVariableTable(LocalVariableTable),
    LocalVariableTypeTable(LocalVariableTypeTable),
    NestHost(NestHost),
    NestMembers(NestMembers),
    PermittedSubclasses(PetrmittedSubclasses),
    UserDefined(UserDefinedAttribute),
    SourceFile(SourceFile),
    BootstrapMethods(BootstrapMethods),
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserDefinedAttribute {
    name: String,
    info: Vec<u8>,
}

impl UserDefinedAttribute {
    pub fn new(name: String, info: &[u8]) -> Self {
        UserDefinedAttribute { name, info: info.to_vec() }
    }
}
