use crate::byte_reader::ByteReader;
use crate::class_file::ClassFile;
use crate::byte_reader::ReadError;
use crate::class_file_version::{ClassFileVersion, FileVersionError};

type Result<T> = std::result::Result<T, ClassReaderError>;

#[derive(Debug, thiserror::Error, PartialEq)]
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
}