use crate::mem::AddressSpace;

pub struct Vram {
    pub buff: Vec<u8>,
}

impl Vram {
    pub fn new(size: usize) -> Vram {
        Vram {
            buff: vec![0; size],
        }
    }
}

impl AddressSpace for Vram {
    fn peek(&self, ptr: u16) -> u8 {
        return self.buff[ptr as usize];
    }
    fn poke(&mut self, ptr: u16, v: u8) {
        self.buff[ptr as usize] = v;
    }
}