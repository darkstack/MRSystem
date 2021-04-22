

use sdl2::render::{Canvas, Texture, TextureAccess};
use sdl2::Sdl;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;


const SCREEN_WIDTH: usize = 256;
const SCREEN_HEIGHT: usize = 192;
pub const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 3;

pub struct Gfx {
    pub renderer: Box<Canvas<Window>>,
    pub texture: Texture<'static>,
    _texture_creator: TextureCreator<WindowContext>,
}


impl Gfx {
    pub fn new() -> (Gfx, Sdl) {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let mut window_builder = video_subsystem.window(
            "MRSystem",
            (SCREEN_WIDTH) as u32,
            (SCREEN_HEIGHT) as u32,
        );
        let window = window_builder.position_centered().build().unwrap();
        let renderer =  window
                        .into_canvas()
                        .accelerated()
                        .present_vsync()
                        .build()
                        .unwrap();
        let texture_creator = renderer.texture_creator();
        let texture_creator_pointer = &texture_creator as *const TextureCreator<WindowContext>;
        let texture = unsafe { &*texture_creator_pointer }
            .create_texture(
                PixelFormatEnum::BGR24,
                TextureAccess::Streaming,
                SCREEN_WIDTH as u32,
                SCREEN_HEIGHT as u32,
            )
            .unwrap();

        (
            Gfx {
                renderer: Box::new(renderer),
                texture,
                _texture_creator: texture_creator,
            },
            sdl,
        )
    }

    pub fn tick(&mut self) {
    }

    /// Copies the overlay onto the given screen and displays it to the SDL window.
    pub fn composite(&mut self, ppu_screen: &mut [u8; SCREEN_SIZE]) {
        self.blit(ppu_screen);
        self.renderer.clear();
        let _ = self.renderer.copy(&self.texture, None, None);
        self.renderer.present();
    }

    /// Updates the window texture with new screen data.
    fn blit(&mut self, ppu_screen: &[u8; SCREEN_SIZE]) {
        self.texture
            .update(None, ppu_screen, SCREEN_WIDTH * 3)
            .unwrap()
    }
}