type Result<T> = std::result::Result<T, ReadError>;

#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error("End of file encountered unexpectedly")]
    #[non_exhaustive]
    UnexpectedEOF,
}

#[derive(Debug)]
pub struct ByteReader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> ByteReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        ByteReader { buf: data, pos: 0}
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
        Ok(u8::from_be_bytes([slice[0]]))
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let slice = self.read_bytes(std::mem::size_of::<u16>())?;
        Ok(u16::from_be_bytes([slice[0], slice[1]]))
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let slice = self.read_bytes(std::mem::size_of::<u32>())?;
        Ok(u32::from_be_bytes([
            slice[0],
            slice[1],
            slice[2],
            slice[3],
        ]))    
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
}
