
use std::fmt::format;

use crate::utils::FormatDataDebug;

/*
Memory map
==========
    $10000 -----------------------------------------------------------
           Paging registers
     $FFFC -----------------------------------------------------------
           Mirror of RAM at $C000-$DFFF
     $E000 -----------------------------------------------------------
           8k of on-board RAM (mirrored at $E000-$FFFF)
     $C000 -----------------------------------------------------------
           16k ROM Page 2, or one of two pages of Cartridge RAM
     $8000 -----------------------------------------------------------
           16k ROM Page 1
     $4000 -----------------------------------------------------------
           15k ROM Page 0
     $0400 -----------------------------------------------------------
           First 1k of ROM Bank 0, never paged out with rest of Page 0
     $0000 -----------------------------------------------------------
*/


//use std::io::{self, Read, Write};
use crate::{bus::BusSpace, ram::Ram, rom::Rom, vdp::Vdp};
use crate::utils::*;
pub trait AddressSpace {
    // Minimal definition
    fn peek(&self, ptr: u16) -> u8;
    fn poke(&mut self, ptr: u16, v: u8);
}


pub trait Mem {
    fn load(&mut self, addr: u16) -> u8;
    fn loadw(&mut self, addr: u16) -> u16;
    fn store(&mut self, addr: u16, v: u8);
    fn storew(&mut self, addr: u16, v: u16);
}



pub struct PagingRegister {
    pub ram_select_register: u8,
    pub page_0_bank: u8,
    pub page_1_bank: u8,
    pub page_2_bank: u8,
}

pub struct MemMap {
    pub paging_register: PagingRegister,
    pub ram: Ram,
    pub rom: Box<Rom>,
    pub vdp: Vdp
}

impl MemMap {
    pub fn new(rom : Rom,vdp: Vdp) -> MemMap {
        MemMap {
            paging_register: PagingRegister {
                ram_select_register: 0,
                page_0_bank: 0,
                page_1_bank: 0,
                page_2_bank: 0,
            },
            ram: Ram::new(8192),
            rom: Box::new(rom),
            vdp: vdp,
        }
    }
}


impl BusSpace for MemMap{
    fn bus_in(&mut self, addr:u8) -> u8 {
        self.vdp.load(addr as u16)
    }
    fn bus_out(&mut self, addr:u8,v:u8){
        self.vdp.store(addr as u16, v);
    }
}


impl Mem for MemMap {

    fn load(&mut self, addr: u16) -> u8 {
        if addr > 0xC000  {
            let ret = self.ram.peek(addr & 0x1FF);
            ret
        }
        else{
            let ret = self.rom.peek(addr);
            ret
        }
    }
    
    fn loadw(&mut self, addr: u16) -> u16 {
        if addr > 0xC000 {
            let ret = get_u16_from_2_u8( self.ram.peek(addr & 0x1FFF), self.ram.peek((addr+1)&0x1FFF) );
            ret
        }
        else{
            let ret = get_u16_from_2_u8( self.rom.peek(addr), self.rom.peek(addr+1));
            ret
        }
    }
    fn store(&mut self, addr: u16, v: u8) {
        println!("write at {:x} value {:x}", addr, v);
        //RAM
        if addr > 0xC000 {
            self.ram.poke(addr & 0x1FFF,v);
        }
        else{
            panic!("{:04X} : NOT MAPPED",addr);
        }
    }
    fn storew(&mut self, addr: u16, v: u16) {
        println!("writew at {:x} value {:x}", addr, v);
        //RAM

        if addr > 0xC000 {
            let (a,b) = get_2_u8_from_u16(v);
            self.ram.poke(addr & 0x1FFF,a);
            self.ram.poke((addr & 0x1FFF)+1,b);
        }
        else{
            panic!("{:04X} : NOT MAPPED",addr);
        }
    }

    
}

use std::fs::File;  
use std::io::Write; 
impl FormatDataDebug for MemMap {
    fn debug(&self) {
        let x = self.ram.buff.clone();
        println!("RAM: {:?}", x);
        let mut f = File::create("ram.hex").expect("Unable to create file"); 
        f.write_all(&x).expect("error");
        // let s = format!("{:x}",self.
    }
}

