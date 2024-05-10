struct ClassReader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> ClassReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        ClassReader { buf: data, pos: 0}
    }

    fn read_bytes(&mut self, size: usize) -> Result<&'a [u8], &'static str> {
        if self.pos + size > self.buf.len() {
            Err("Error reading bytes")
        } else {
            let bytes = &self.buf[self.pos..self.pos + size];
            Ok(bytes)
        }
    }

    fn read_u32(&mut self) -> Result<u32, &'static str> {
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
    use super::ClassReader;

    #[test]
    fn test_magic() {
        let data = [0xCA, 0xFE, 0xBA, 0xBE];
        let mut reader = ClassReader::new(&data);
        let magic = reader.read_u32().unwrap();
        
        assert_eq!(magic, 0xCAFEBABE);
    }
}
