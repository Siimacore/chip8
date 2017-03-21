extern crate sdl2;

use sdl::Sdl;
use sdl2::rect::Rect;

pub const HEIGHT: i32 = 512;
pub const WIDTH: i32 = 256;

pub const RECT : Rect = Rect::new(0, 0, 8, 8);

pub struct Display {
	renderer: sdl2::render::Renderer,
	pixels: [[bool; WIDTH] ; HEIGHT],
}

impl Display {
    pub fn new (sdl_context: &Sdl) -> Self{
    	let video = sdl_context.video()
    					.expect("SDL2 init failed");
    	let window = video.window("Chip 8", WIDTH as u32, HEIGHT as u32)
    					.position_centered()
    					.build()
    					.expect("Window creation failed");
    	let renderer = window.renderer()
    					.accelerated()
    					.build()
    					.expect("Init window failed");
    	Display {
    		renderer : renderer,
    		pixels = [[false; WIDTH] ; HEIGHT],
    	}
    }


}