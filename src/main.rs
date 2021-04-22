
use structopt::StructOpt;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod mem;
pub mod gfx;
pub mod cpu;

#[derive(StructOpt,Clone)]
struct Cli {
    /// The pattern to look for
    #[structopt(short = "s", long = "debug")]
    debug : Option<String>,
    /// The path to the file to read
    #[structopt(short = "r", long = "rom",parse(from_os_str))]
    rom: std::path::PathBuf,
}

fn main() {
    let cli = Cli::from_args();
    let rom_path = &cli.rom;
    let rom = mem::load_rom(rom_path.to_path_buf());

    let (mut gfx, sdl)= gfx::Gfx::new();
    let mut screen = Box::new([0; gfx::SCREEN_SIZE]);
    let memmap = mem::MemMap::new();
    let cpu = cpu::Cpu::new(memmap);
    loop{
        //Event pump
        while let Some(ev) = sdl.event_pump().unwrap().poll_event() {
            match ev{
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    match key { 
                        Keycode::Escape => {
                            println!("Exit");
                            return
                        }
                        _ => continue,
                    }
                },
                Event::Quit { .. } => return,
                _ => continue,
            }
        }

        gfx.tick();
        gfx.composite(&mut screen)
    }
}
