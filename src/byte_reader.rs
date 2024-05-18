use std::borrow::Cow;

use cesu8::Cesu8DecodingError;

type Result<T> = std::result::Result<T, ReadError>;

#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error("End of file encountered unexpectedly")]
    #[non_exhaustive]
    UnexpectedEOF,
    #[error("Error reading utf-8 bytes: {0}")]
    #[non_exhaustive]
    Cesu8DecodingError(#[from] Cesu8DecodingError),
}

#[derive(Debug)]
pub struct ByteReader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> ByteReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        ByteReader { buf: data, pos: 0 }
    }

    fn read_bytes(&mut self, size: usize) -> Result<&'a [u8]> {
        if self.pos + size > self.buf.len() {
            Err(ReadError::UnexpectedEOF)
        } else {
            let bytes = &self.buf[self.pos..self.pos + size];
            Ok(bytes)
        }
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let slice = self.read_bytes(std::mem::size_of::<u8>())?;
        Ok(u8::from_be_bytes(slice.try_into().unwrap()))
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let slice = self.read_bytes(std::mem::size_of::<u16>())?;
        Ok(u16::from_be_bytes(slice.try_into().unwrap()))
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let slice = self.read_bytes(std::mem::size_of::<u32>())?;
        Ok(u32::from_be_bytes(slice.try_into().unwrap()))
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        let slice = self.read_bytes(std::mem::size_of::<i32>())?;
        Ok(i32::from_be_bytes(slice.try_into().unwrap()))
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        let slice = self.read_bytes(std::mem::size_of::<i64>())?;
        Ok(i64::from_be_bytes(slice.try_into().unwrap()))
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        let slice = self.read_bytes(std::mem::size_of::<f32>())?;
        Ok(f32::from_be_bytes(slice.try_into().unwrap()))
    }

    pub fn read_f64(&mut self) -> Result<f64> {
        let slice = self.read_bytes(std::mem::size_of::<f64>())?;
        Ok(f64::from_be_bytes(slice.try_into().unwrap()))
    }

    pub fn read_utf8(&mut self, len: u32) -> Result<Cow<str>> {
        let modified_utf_bytes = self.read_bytes(len as usize)?;
        cesu8::from_java_cesu8(&modified_utf_bytes).map_err(|e| ReadError::Cesu8DecodingError(e))
    }
}

#[cfg(test)]
mod tests {
    use crate::byte_reader::ReadError;

    use super::ByteReader;

    #[test]
    fn test_magic_success() {
        let data = [0xCA, 0xFE, 0xBA, 0xBE];
        let mut reader = ByteReader::new(&data);
        let magic = reader.read_u32().unwrap();

        assert_eq!(magic, 0xCAFEBABE);
    }

    #[test]
    fn test_error_eof() {
        let data = [0xCA];
        let mut reader = ByteReader::new(&data);

        assert!(reader.read_u32().is_err());
    }

    #[test]
    fn test_integer() {
        let data = [0x00, 0x14, 0x67, 0x8C];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_i32().unwrap(), 1337228);
    }

    #[test]
    fn test_float_inf() {
        let data = [0x7F, 0x80, 0x00, 0x00];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_f32().unwrap(), f32::INFINITY);
    }

    #[test]
    fn test_float_neg_inf() {
        let data = [0xFF, 0x80, 0x00, 0x00];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_f32().unwrap(), f32::NEG_INFINITY);
    }
}
