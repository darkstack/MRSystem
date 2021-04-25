#![allow(dead_code)]
use std::rc::Rc;

use structopt::StructOpt;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod mem;
pub mod ram;
pub mod vram;
pub mod rom;
pub mod gfx;
pub mod vdp;
pub mod cpu;
pub mod bus;

pub mod utils;

#[derive(StructOpt,Clone)]
struct Cli {
    /// The pattern to look for
    #[structopt(short = "s", long = "debug")]
    debug : Option<String>,
    /// The path to the file to read
    #[structopt(short = "r", long = "rom",parse(from_os_str))]
    rom: std::path::PathBuf,
}

fn main() -> ! {


    let cli = Cli::from_args();
    let rom_path = &cli.rom;
    let rom = rom::load_rom(rom_path.to_path_buf()).expect("rom error");

    let (mut gfx, sdl)= gfx::Gfx::new();
    

    let mut vdp = vdp::Vdp::new();
    let memmap = mem::MemMap::new(rom,vdp);
    //let mut cpu = cpu::Cpu::new(memmap,cli.debug.is_some());
    let mut cpu = cpu::Cpu::new(memmap,cli.debug.is_some());
    gfx.tick();
    gfx.composite(&mut cpu.mem.vdp.screen);
    loop{
  
        cpu.step();
        //Event pump
        while let Some(ev) = sdl.event_pump().unwrap().poll_event() {

            match ev{
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    match key { 
                        Keycode::Escape => {
                            std::process::exit(0);
                        }
                        _ => continue,
                    }
                },
                Event::Quit { .. } => std::process::exit(0),
                _ => continue,
            }

        }
 
        
        // gfx.tick();
        // gfx.composite(&mut screen)
    }
}
