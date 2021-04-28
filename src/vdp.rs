use crate::{gfx::{self, SCREEN_SIZE}, };
use crate::vram::*;
use crate::mem::*;

const VRAM_SIZE: usize = 16384; 


pub struct Vdp{
    pub screen:Box<[u8; SCREEN_SIZE]>,
    pub vram:Box<Vram>,
}

impl Vdp {
    pub fn new() -> Vdp {
        Vdp {
            screen: Box::new([0; gfx::SCREEN_SIZE]),
            vram: Box::new(Vram::new(VRAM_SIZE)),
        }
    }

    pub fn step(){

    }
}

impl Mem for Vdp{
    fn load(&mut self, addr: u16) -> u8 {
        todo!();
    }
    fn loadw(&mut self, addr: u16) -> u16{
        todo!();
    }
    fn store(&mut self, addr: u16, v: u8) {
        println!("VDP : addr: {:04x} v: {:01x}",addr,v);
    }

    fn storew(&mut self, addr: u16, v: u16) {
        todo!()
    }
}
