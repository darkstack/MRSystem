use std::fs::File;
use std::io::Error;
use std::io::{self, Read};
use std::path::Path;
use std::path::PathBuf;
use crate::{mem::{AddressSpace, Mem}};


pub struct Rom {
    buff: Vec<u8>,
    sms_header: Header,
}

impl Rom {
    pub fn new(smsheader : Header, size: usize) -> Rom {
        Rom {
            buff: vec![0; size],
            sms_header: smsheader,
        }
    }
}

impl AddressSpace for Rom {
    fn peek(&self, ptr: u16) -> u8 {
        return self.buff[ptr as usize];
    }
    fn poke(&mut self, ptr: u16, v: u8) {}
}

impl Mem for Rom{
    fn load(&mut self, addr: u16) -> u8 {
        let v = self.buff[addr as usize];
        v
    }
    //TODO : unused ? 
    fn loadw(&mut self, addr: u16) -> u16{
        0
    }

    fn store(&mut self, addr: u16, v: u8) {
        panic!("Write to rom")
    }
    fn storew(&mut self, addr: u16, v: u16) {
        panic!("Write to rom")
    }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ROMTYPE {
    _8K = 0x4A,
    _16K = 0x4B,
    _32K = 0x4C,
    _48K = 0x4D,
    _128K = 0x4E,
    _256K = 0x4F,
    _NOTSMS = 0x00,
}

impl From<u8> for ROMTYPE {
    fn from(v: u8) -> Self {
        match v {
            0x4A => ROMTYPE::_8K,
            0x4B => ROMTYPE::_16K,
            0x4C => ROMTYPE::_32K,
            0x4D => ROMTYPE::_48K,
            0x4E => ROMTYPE::_128K,
            0x4F => ROMTYPE::_256K,
            _ => ROMTYPE::_NOTSMS,
        }
    }
}

pub struct Header {
    header: [u8; 10],
    checksum: u16,
    serial: u16,
    revision: u8,
    rom_size: ROMTYPE,
}

const HEADER_OFFSET: usize = 0x7FF0;
use std::convert::TryInto;

pub fn load_rom(file: PathBuf) -> Result<Rom, Error> {
    println!("load {}", file.to_string_lossy());
    println!("Current Path {}", std::env::current_dir()?.display());
    let rd = &mut File::open(&Path::new(&file)).unwrap();
    let rd_size = rd.metadata().unwrap().len();
    let mut prg_rom = vec![0u8; rd_size as usize];
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
    if prg_rom.len()>HEADER_OFFSET+15 {
        let smsheader = Header {
            header: prg_rom[HEADER_OFFSET..HEADER_OFFSET + 10].try_into().expect("Error"),
            checksum: (prg_rom[HEADER_OFFSET + 10] as u16)
                | ((prg_rom[HEADER_OFFSET + 11] as u16) << 8),
            serial: (prg_rom[HEADER_OFFSET + 12] as u16) | ((prg_rom[HEADER_OFFSET + 13] as u16) << 8),
            revision: prg_rom[HEADER_OFFSET + 14],
            rom_size: prg_rom[HEADER_OFFSET + 15].into(),
        };
        println!("Rom Loaded");
        Ok(Rom {sms_header:smsheader, buff: prg_rom })
    }
    else
    {
        let smsheader = Header {
            header: [0u8;10],
            checksum: 0,
            serial: 0,
            revision: 0,
            rom_size: ROMTYPE::_32K,
        };
        println!("Rom Loaded (Sub 32K)");
        Ok(Rom {sms_header:smsheader, buff: prg_rom })
    }

    //Check Rom Header


}
