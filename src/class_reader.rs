type Result<T> = std::result::Result<T, ReadError>;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ReadError {
    #[error("End of file encountered unexpectedly")]
    #[non_exhaustive]
    UnexpectedEOF,
}

struct ClassReader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> ClassReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        ClassReader { buf: data, pos: 0}
    }

    fn read_bytes(&mut self, size: usize) -> Result<&'a [u8]> {
        if self.pos + size > self.buf.len() {
            Err(ReadError::UnexpectedEOF)
        } else {
            let bytes = &self.buf[self.pos..self.pos + size];
            Ok(bytes)
        }
    }

    fn read_u8(&mut self) -> Result<u8> {
        let slice = self.read_bytes(std::mem::size_of::<u8>())?;
        Ok(u8::from_be_bytes([slice[0]]))
    }

    fn read_u16(&mut self) -> Result<u16> {
        let slice = self.read_bytes(std::mem::size_of::<u16>())?;
        Ok(u16::from_be_bytes([slice[0], slice[1]]))
    }

    fn read_u32(&mut self) -> Result<u32> {
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
    use crate::class_reader::ReadError;

    use super::ClassReader;

    #[test]
    fn test_magic_success() {
        let data = [0xCA, 0xFE, 0xBA, 0xBE];
        let mut reader = ClassReader::new(&data);
        let magic = reader.read_u32().unwrap();
        
        assert_eq!(magic, 0xCAFEBABE);
    }

    #[test]
    fn test_error_eof() {
        let data = [0xCA];
        let mut reader = ClassReader::new(&data);
        
        assert_eq!(reader.read_u32(), Err(ReadError::UnexpectedEOF));
    }
}
