use crate::byte_reader::ByteReader;
use crate::class_file::ClassFile;
use crate::byte_reader::ReadError;
use crate::class_file_version::{ClassFileVersion, FileVersionError};
use crate::constant_pool::Constant;

type Result<T> = std::result::Result<T, ClassReaderError>;

#[derive(Debug, thiserror::Error)]
enum ClassReaderError {
    #[error("Invalid magic number {0}")]
    #[non_exhaustive]
    InvalidMagicNumber(u32),
    #[error("Error encountered during reading: {0}")]
    #[non_exhaustive]
    ReadError(#[from] ReadError),
    #[error("Error during parsing file version: {0}")]
    #[non_exhaustive]
    FileVersionError(#[from] FileVersionError)
}

#[derive(Debug)]
struct ClassFileReader<'a> {
    byte_reader: ByteReader<'a>,
    class_file: ClassFile,
}

impl<'a> ClassFileReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        ClassFileReader { byte_reader: ByteReader::new(data), class_file: ClassFile::default() }
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
                _ => panic!("at the disco"),
            }
        }
        Ok(())
    }

    fn read_string_constant(&mut self) -> Result<()> {
        let length = self.byte_reader.read_u16()?;
        let utf8 =  self.byte_reader.read_utf8(length as u32)?.into_owned();
        self.class_file.constant_pool.add(Constant::Utf8(utf8));
        Ok(())
    }

    fn read_int_constant(&mut self) -> Result<()> {
        let integer = self.byte_reader.read_i32()?;
        self.class_file.constant_pool.add(Constant::Integer(integer));
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
}