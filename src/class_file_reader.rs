use crate::access_flag::ClassFileAccessFlags;
use crate::byte_reader::ByteReader;
use crate::byte_reader::ReadError;
use crate::class_file::ClassFile;
use crate::class_file_version::{ClassFileVersion, FileVersionError};
use crate::constant_pool::Constant;

type Result<T> = std::result::Result<T, ClassReaderError>;

#[derive(Debug, thiserror::Error)]
enum ClassReaderError {
    #[error("Invalid magic number {0}")]
    #[non_exhaustive]
    InvalidMagicNumber(u32),
    #[error("Tag not supported: {0}")]
    #[non_exhaustive]
    TagNotSupported(u8),
    #[error("Error encountered during reading: {0}")]
    #[non_exhaustive]
    ReadError(#[from] ReadError),
    #[error("Error during parsing file version: {0}")]
    #[non_exhaustive]
    FileVersionError(#[from] FileVersionError),
}

#[derive(Debug)]
struct ClassFileReader<'a> {
    byte_reader: ByteReader<'a>,
    class_file: ClassFile,
}

impl<'a> ClassFileReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        ClassFileReader {
            byte_reader: ByteReader::new(data),
            class_file: ClassFile::default(),
        }
    }

    pub fn read_magic_number(&mut self) -> Result<()> {
        match self.byte_reader.read_u32() {
            Ok(0xCAFEBABE) => Ok(()),
            Ok(magic_number) => Err(ClassReaderError::InvalidMagicNumber(magic_number)),
            Err(err) => Err(err.into()),
        }
    }

    pub fn read_version(&mut self) -> Result<()> {
        let minor_version = self.byte_reader.read_u16()?;
        let major_version = self.byte_reader.read_u16()?;
        self.class_file.version = ClassFileVersion::from(major_version, minor_version)?;
        Ok(())
    }

    pub fn read_constant_pool(&mut self) -> Result<()> {
        let constant_pool_count = self.byte_reader.read_u16()?;
        for _ in 1..constant_pool_count {
            let tag = self.byte_reader.read_u8()?;
            match tag {
                1 => self.read_string_constant()?,
                3 => self.read_int_constant()?,
                4 => self.read_float_constant()?,
                5 => self.read_long_constant()?,
                6 => self.read_double_constant()?,
                7 => self.read_class_index()?,
                8 => self.read_string_info()?,
                9 => self.read_field_ref()?,
                10 => self.read_method_ref()?,
                11 => self.read_interface_method_ref()?,
                12 => self.read_name_and_type()?,
                15 => self.read_method_handle()?,
                16 => self.read_method_type()?,
                17 => self.read_dynamic()?,
                18 => self.read_invoke_dynamic()?,
                19 => self.read_module()?,
                20 => self.read_package()?,
                _ => return Err(ClassReaderError::TagNotSupported(tag)),
            }
        }
        Ok(())
    }

    fn read_string_constant(&mut self) -> Result<()> {
        let length = self.byte_reader.read_u16()?;
        let utf8 = self.byte_reader.read_utf8(length as u32)?.into_owned();
        self.class_file.constant_pool.add(Constant::Utf8(utf8));
        Ok(())
    }

    fn read_int_constant(&mut self) -> Result<()> {
        let integer = self.byte_reader.read_i32()?;
        self.class_file
            .constant_pool
            .add(Constant::Integer(integer));
        Ok(())
    }

    fn read_float_constant(&mut self) -> Result<()> {
        let float = self.byte_reader.read_f32()?;
        self.class_file.constant_pool.add(Constant::Float(float));
        Ok(())
    }

    fn read_long_constant(&mut self) -> Result<()> {
        let long = self.byte_reader.read_i64()?;
        self.class_file.constant_pool.add(Constant::Long(long));
        Ok(())
    }

    fn read_double_constant(&mut self) -> Result<()> {
        let double = self.byte_reader.read_f64()?;
        self.class_file.constant_pool.add(Constant::Double(double));
        Ok(())
    }

    fn read_class_index(&mut self) -> Result<()> {
        let class_index = self.byte_reader.read_u16()?;
        self.class_file
            .constant_pool
            .add(Constant::ClassIndex(class_index));
        Ok(())
    }

    fn read_string_info(&mut self) -> Result<()> {
        let string_info = self.byte_reader.read_u16()?;
        self.class_file
            .constant_pool
            .add(Constant::StringIndex(string_info));
        Ok(())
    }

    fn read_field_ref(&mut self) -> Result<()> {
        let class_index = self.byte_reader.read_u16()?;
        let name_and_type_index = self.byte_reader.read_u16()?;
        self.class_file
            .constant_pool
            .add(Constant::FieldRef(class_index, name_and_type_index));
        Ok(())
    }

    fn read_method_ref(&mut self) -> Result<()> {
        let class_index = self.byte_reader.read_u16()?;
        let name_and_type_index = self.byte_reader.read_u16()?;
        self.class_file
            .constant_pool
            .add(Constant::MethodRef(class_index, name_and_type_index));
        Ok(())
    }

    fn read_interface_method_ref(&mut self) -> Result<()> {
        let class_index = self.byte_reader.read_u16()?;
        let name_and_type_index = self.byte_reader.read_u16()?;
        self.class_file
            .constant_pool
            .add(Constant::InterfaceMethodRef(
                class_index,
                name_and_type_index,
            ));
        Ok(())
    }

    fn read_name_and_type(&mut self) -> Result<()> {
        let name_index = self.byte_reader.read_u16()?;
        let descriptor_index = self.byte_reader.read_u16()?;
        self.class_file
            .constant_pool
            .add(Constant::NameAndType(name_index, descriptor_index));
        Ok(())
    }

    fn read_method_handle(&mut self) -> Result<()> {
        let reference_kind = self.byte_reader.read_u8()?;
        let reference_index = self.byte_reader.read_u16()?;
        self.class_file
            .constant_pool
            .add(Constant::MethodHandle(reference_kind, reference_index));
        Ok(())
    }

    fn read_method_type(&mut self) -> Result<()> {
        let descriptor_index = self.byte_reader.read_u16()?;
        self.class_file
            .constant_pool
            .add(Constant::MethodType(descriptor_index));
        Ok(())
    }

    fn read_dynamic(&mut self) -> Result<()> {
        let bootstrap_method_attr_index = self.byte_reader.read_u16()?;
        let name_and_type_index = self.byte_reader.read_u16()?;
        self.class_file.constant_pool.add(Constant::Dynamic(
            bootstrap_method_attr_index,
            name_and_type_index,
        ));
        Ok(())
    }

    fn read_invoke_dynamic(&mut self) -> Result<()> {
        let bootstrap_method_attr_index = self.byte_reader.read_u16()?;
        let name_and_type_index = self.byte_reader.read_u16()?;
        self.class_file.constant_pool.add(Constant::InvokeDynamic(
            bootstrap_method_attr_index,
            name_and_type_index,
        ));
        Ok(())
    }

    fn read_module(&mut self) -> Result<()> {
        let name_index = self.byte_reader.read_u16()?;
        self.class_file
            .constant_pool
            .add(Constant::Module(name_index));
        Ok(())
    }

    fn read_package(&mut self) -> Result<()> {
        let name_index = self.byte_reader.read_u16()?;
        self.class_file
            .constant_pool
            .add(Constant::Module(name_index));
        Ok(())
    }

    fn read_access_flags(&mut self) -> Result<()> {
        let mask = self.byte_reader.read_u16()?;
        let flags = ClassFileAccessFlags::new(mask);
        self.class_file.flags = flags;
        Ok(())
    }
}
