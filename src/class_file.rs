use crate::{class_file_version::ClassFileVersion, constant_pool::ConstantPool};

#[derive(Debug, Default)]
pub struct ClassFile {
    pub version: ClassFileVersion,
    pub constant_pool: ConstantPool,
}
