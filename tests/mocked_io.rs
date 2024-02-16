use std::io::{self, Read, Write};

pub struct Mock {
    pub response: String,
    pub input: String,
    position: usize,
}
impl Mock {
    pub fn new() -> Self {
        Self {
            response: String::new(),
            input: String::new(),
            position: 0,
        }
    }
    pub fn response(mut self, response: String) -> Self {
        self.response = response;
        self
    }
}

impl Read for Mock {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.position >= self.response.len() {
            return Ok(0);
        }
        let response_bytes = self.response.as_bytes();
        let len = std::cmp::min(buf.len(), response_bytes.len() - self.position);
        buf[..len].copy_from_slice(&response_bytes[self.position..self.position + len]);
        self.position += len;
        Ok(len)
    }
}
impl Write for Mock {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let string = String::from_utf8(buf.to_vec()).unwrap();
        self.input.push_str(&string);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
