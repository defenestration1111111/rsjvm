use crate::access_flag::ClassFileAccessFlags;
use crate::byte_reader::ByteReader;
use crate::byte_reader::ReadError;
use crate::class_file::ClassFile;
use crate::class_file_version::{ClassFileVersion, FileVersionError};
use crate::constant_pool::Constant;
use crate::constant_pool::ConstantPoolError;

type Result<T> = std::result::Result<T, ClassReaderError>;

#[derive(Debug, thiserror::Error)]
enum ClassReaderError {
    #[error("Invalid magic number {0}")]
    #[non_exhaustive]
    InvalidMagicNumber(u32),
    #[error("Tag not supported: {0}")]
    #[non_exhaustive]
    TagNotSupported(u8),
    #[error("Unexpected constant")]
    #[non_exhaustive]
    UnexpectedConstant,
    #[error("Error encountered during reading: {0}")]
    #[non_exhaustive]
    ReadError(#[from] ReadError),
    #[error("Error during parsing file version: {0}")]
    #[non_exhaustive]
    FileVersionError(#[from] FileVersionError),
    #[error("Error during parsing constant pool {0}")]
    #[non_exhaustive]
    ConstantPoolError(#[from] ConstantPoolError),
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
        for mut index in 1..constant_pool_count {
            let tag = self.byte_reader.read_u8()?;
            let constant = match tag {
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
            };

            self.class_file.constant_pool.add(constant.clone());

            if matches!(constant, Constant::Long(_) | Constant::Double(_)) {
                index += 1;
            }
        }
        Ok(())
    }

    fn read_string_constant(&mut self) -> Result<Constant> {
        let length = self.byte_reader.read_u16()?;
        Ok(Constant::Utf8(self.byte_reader.read_utf8(length as u32)?.into_owned()))
    }

    fn read_int_constant(&mut self) -> Result<Constant> {
        let integer = self.byte_reader.read_i32()?;
        Ok(Constant::Integer(integer))
    }

    fn read_float_constant(&mut self) -> Result<Constant> {
        let float = self.byte_reader.read_f32()?;
        Ok(Constant::Float(float))
    }

    fn read_long_constant(&mut self) -> Result<Constant> {
        let long = self.byte_reader.read_i64()?;
        Ok(Constant::Long(long))
    }

    fn read_double_constant(&mut self) -> Result<Constant> {
        let double = self.byte_reader.read_f64()?;
        Ok(Constant::Double(double))
    }

    fn read_class_index(&mut self) -> Result<Constant> {
        let class_index = self.byte_reader.read_u16()?;
        Ok(Constant::ClassIndex(class_index))
    }

    fn read_string_info(&mut self) -> Result<Constant> {
        let string_info = self.byte_reader.read_u16()?;
        Ok(Constant::StringIndex(string_info))
    }

    fn read_field_ref(&mut self) -> Result<Constant> {
        let pair = self.byte_reader.read_pair_u16()?;
        Ok(Constant::FieldRef(pair.0, pair.1))
    }

    fn read_method_ref(&mut self) -> Result<Constant> {
        let pair = self.byte_reader.read_pair_u16()?;
        Ok(Constant::MethodRef(pair.0, pair.1))
    }

    fn read_interface_method_ref(&mut self) -> Result<Constant> {
        let pair = self.byte_reader.read_pair_u16()?;
        Ok(Constant::InterfaceMethodRef(pair.0, pair.1))
    }

    fn read_name_and_type(&mut self) -> Result<Constant> {
        let pair = self.byte_reader.read_pair_u16()?;
        Ok(Constant::NameAndType(pair.0, pair.1))
    }

    fn read_method_handle(&mut self) -> Result<Constant> {
        let reference_kind = self.byte_reader.read_u8()?;
        let reference_index = self.byte_reader.read_u16()?;
        Ok(Constant::MethodHandle(reference_kind, reference_index))
    }

    fn read_method_type(&mut self) -> Result<Constant> {
        let descriptor_index = self.byte_reader.read_u16()?;
        Ok(Constant::MethodType(descriptor_index))
    }

    fn read_dynamic(&mut self) -> Result<Constant> {
        let pair = self.byte_reader.read_pair_u16()?;
        Ok(Constant::Dynamic(pair.0, pair.1))
    }

    fn read_invoke_dynamic(&mut self) -> Result<Constant> {
        let pair = self.byte_reader.read_pair_u16()?;
        Ok(Constant::InvokeDynamic(pair.0, pair.1))
    }

    fn read_module(&mut self) -> Result<Constant> {
        let name_index = self.byte_reader.read_u16()?;
        Ok(Constant::Module(name_index))
    }

    fn read_package(&mut self) -> Result<Constant> {
        let name_index = self.byte_reader.read_u16()?;
        Ok(Constant::Package(name_index))
    }

    fn read_access_flags(&mut self) -> Result<()> {
        let mask = self.byte_reader.read_u16()?;
        let flags = ClassFileAccessFlags::new(mask);
        self.class_file.flags = flags;
        Ok(())
    }

    fn get_utf8(&mut self, name_index: u16) -> Result<String> {
        match self.class_file.constant_pool.get(name_index as usize)? {
            Constant::Utf8(utf8_content) => Ok(*utf8_content),
            _ => Err(ClassReaderError::UnexpectedConstant),
        }
    }

    fn get_class_name(&mut self, name_index: u16) -> Result<String> {
        let constant = self.class_file.constant_pool.get(name_index as usize)?;
        return match constant {
            Constant::ClassIndex(class_index) => self.get_utf8(*class_index),
            _ => Err(ClassReaderError::UnexpectedConstant)
        }
    }

    fn read_this_class(&mut self) -> Result<()> {
        let name_index = self.byte_reader.read_u16()?;
        self.class_file.this_class = self.get_class_name(name_index)?;
        Ok(())
    }

    fn read_super_class(&mut self) -> Result<()> {
        let name_index = self.byte_reader.read_u16()?;
        if name_index == 0 {
            return Ok(())
        }
        self.class_file.super_class = Some(self.get_class_name(name_index)?);
        Ok(())
    }

    fn read_interfaces(&mut self) -> Result<()> {
        let interfaces_count = self.byte_reader.read_u16()?;
        let mut intefaces = Vec::new();
        for _ in 0..interfaces_count {
            let name_index = self.byte_reader.read_u16()?;
            intefaces.push(self.get_class_name(name_index)?);
        }
        self.class_file.interfaces = intefaces;
        Ok(())
    }

}
