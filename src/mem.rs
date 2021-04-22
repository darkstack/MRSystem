use std::io::Error;
use std::path::Path;
use std::fs::File;
use std::path::PathBuf;
use std::io::{self, Read, Write};

pub trait AddressSpace {
    // Minimal definition
    fn peek(&self, ptr: u16) -> u8;
    fn poke(&mut self, ptr: u16, v: u8);
}


pub trait Mem {
    fn load(&self, addr: u16) -> u8;
    fn store(&mut self, addr: u16, v: u8);
}

pub struct Rom {
    buff: Vec<u8>,
}
pub struct Ram {
    buff: Vec<u8>,
}

impl Ram {
    pub fn new(size: usize) -> Ram {
        Ram { buff: vec![0; size] }
    }
}

impl Rom {
    pub fn new(size: usize) -> Rom {
        Rom { buff: vec![0; size] }
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

impl AddressSpace for Rom {
    fn peek(&self, ptr: u16) -> u8 {
        return self.buff[ptr as usize];
    }
    fn poke(&mut self, ptr: u16, v: u8) {
    }
}

pub struct PagingRegister{
    pub ram_select_register: u8,
    pub page_0_bank : u8,
    pub page_1_bank : u8,
    pub page_2_bank : u8,
}

pub struct MemMap {
    pub paging_register : PagingRegister,
    pub ram: Ram,

}


impl MemMap {
    pub fn new(
    ) -> MemMap {
        MemMap {
            paging_register : PagingRegister{
                ram_select_register: 0,
                page_0_bank : 0,
                page_1_bank : 0,
                page_2_bank : 0,
            },
            ram: Ram::new(8192),
        }
    }

}

impl Mem for MemMap{
    fn load(&self, addr: u16) -> u8{
        println!("ask for {:x}",addr);
        return 0;
    }
    fn store(&mut self, addr: u16, v: u8){
        println!("write {:x} for {:x}",addr,v);
    }
}


pub fn load_rom(file : PathBuf) -> Result<Rom,Error> {
        println!("load {}",file.to_string_lossy());
        println!("Current Path {}", std::env::current_dir()?.display());
        let rd = &mut File::open(&Path::new(&file)).unwrap();
        let rdSize = rd.metadata().unwrap().len();
        let mut prg_rom = vec![0u8; rdSize as usize];
        let mut total = 0;
        while total < prg_rom.len() {
            let count = rd.read(&mut prg_rom[total..])?;
            if count == 0 {
                // Buffer not yet filled, but EOF reached
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "eof reached prematurely",
                ));
            }
            total += count;
        }
    
        println!("Rom Loaded");
        Ok(Rom {
            buff: prg_rom,
        })
}
  

