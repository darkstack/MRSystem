
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
        if addr > 0xC000 && addr < 0xE000 {
            let ret = self.ram.peek(addr);
            ret
        }
        else{
            let ret = self.rom.peek(addr);
            ret
        }
    }
    fn store(&mut self, addr: u16, v: u8) {
        println!("write at {:x} value {:x}", addr, v);
        //RAM
        if (addr > 0xC000 && addr < 0xE000) || addr >= 0xE000{
            self.ram.poke(addr & 0x0FFF,v);
        }
        else{
            panic!("{:04X} : NOT MAPPED",addr);
        }
    }
    fn storew(&mut self, addr: u16, v: u16) {
        println!("writew at {:x} value {:x}", addr, v);
        //RAM

        if (addr > 0xC000 && addr < 0xE000 ) || addr >= 0xE000 {
            let (a,b) = get_2_u8_from_u16(v);
            self.ram.poke(addr & 0x0FFF,a);
            self.ram.poke((addr & 0x0FFF)+1,b);
            println!("{:?}",self.ram.buff);
        }
        else{
            panic!("{:04X} : NOT MAPPED",addr);
        }
    }

    
}

