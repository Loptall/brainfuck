
use std::io::{stdin, Read};

const BUFFER_SIZE: usize = 30000;

pub struct Buffer {
    array: [u8; BUFFER_SIZE],
    current: usize,
    input_buf: String
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            array: [0u8; BUFFER_SIZE],
            current: 0usize,
            input_buf: String::new()
        }
    }

    pub fn shr(&mut self) {
        self.current += 1;
    }

    pub fn shl(&mut self) {
        self.current -= 1;
    }

    pub fn get(&self) -> Option<&u8> {
        self.array.get(self.current)
    }

    pub fn get_mut(&mut self) -> Option<&mut u8> {
        self.array.get_mut(self.current)
    }

    pub fn inc(&mut self) {
        *self.get_mut().unwrap() += 1;
    }

    pub fn dec(&mut self) {
        *self.get_mut().unwrap() -= 1;
    }

    pub fn put(&self) {
        print!("{}", *self.get().unwrap() as char);
    }

    pub fn read(&mut self) {
        let mut buf = String::new();
        let mut h = stdin();
        h.lock();
        h.read_to_string(&mut buf).ok();
        self.input_buf.push_str(buf.as_str());
        *self.get_mut().unwrap() = self.input_buf.remove(0) as u8;
    }
}

