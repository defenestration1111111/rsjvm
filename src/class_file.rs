use crate::{
    access_flag::ClassFileAccessFlags, class_file_version::ClassFileVersion,
    constant_pool::ConstantPool, field::Field,
};

#[derive(Debug, Default)]
pub struct ClassFile {
    pub version: ClassFileVersion,
    pub constant_pool: ConstantPool,
    pub flags: ClassFileAccessFlags,
    pub this_class: String,
    pub super_class: Option<String>,
    pub interfaces: Vec<String>,
    pub fields: Vec<Field>,
}
