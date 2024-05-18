use crate::{
    access_flag::ClassFileAccessFlags, class_file_version::ClassFileVersion,
    constant_pool::ConstantPool,
};

#[derive(Debug, Default)]
pub struct ClassFile {
    pub version: ClassFileVersion,
    pub constant_pool: ConstantPool,
    pub flags: ClassFileAccessFlags,
    pub this_class: u16,
    pub super_class: u16,
}
