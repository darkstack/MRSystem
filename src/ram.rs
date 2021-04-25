use crate::mem::*;


pub struct Ram {
    pub buff: Vec<u8>,
}

impl Ram {
    pub fn new(size: usize) -> Ram {
        Ram {
            buff: vec![0; size],
        }
    }
}

impl AddressSpace for Ram {
    fn peek(&self, ptr: u16) -> u8 {
        return self.buff[ptr as usize];
    }
    fn poke(&mut self, ptr: u16, v: u8) {
        self.buff[ptr as usize] = v;
    }
}