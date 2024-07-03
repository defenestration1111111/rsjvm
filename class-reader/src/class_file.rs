use std::fmt::Display;

use crate::access_flag::ClassFileAccessFlags;
use crate::attribute::Attribute;
use crate::class_file_version::ClassFileVersion;
use crate::constant_pool::ConstantPool;
use crate::field::Field;
use crate::method::Method;

#[derive(Debug, Default, Clone)]
pub struct ClassFile {
    pub version: ClassFileVersion,
    pub constant_pool: ConstantPool,
    pub flags: ClassFileAccessFlags,
    pub this_class: String,
    pub super_class: Option<String>,
    pub interfaces: Vec<String>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
}

impl Display for ClassFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Class file version: {}", self.version)
    }
}
