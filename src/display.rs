extern crate sdl2;

use self::sdl2::rect::Rect;
use self::sdl2::pixels::Color;


pub const rows: i32 = 32;
pub const cols: i32  = 64;
pub const HEIGHT: i32 = 512;
pub const WIDTH: i32 = 256;

pub struct Display<'a> {
	renderer: sdl2::render::Renderer<'a>,
	pixels: [[bool; rows as usize] ; cols as usize],
	rect : Rect,
}

impl<'a> Display<'a> {
    pub fn new () -> Self{
    	let sdl_context = sdl2::init().unwrap();
    	let video = sdl_context.video()
    					.expect("SDL2 init failed");
    	let window = video.window("Chip 8", HEIGHT as u32, WIDTH as u32)
    					.position_centered()
    					.build()
    					.expect("Window creation failed");
    	let mut renderer = window.renderer()
    					.accelerated()
    					.build()
    					.expect("Init window failed");
    	renderer.clear();
    	renderer.present();
    	renderer.set_draw_color(Color::RGB(255,255,255));
    	let mut rect = Rect::new(0, 0, 8, 8);
    	renderer.fill_rect(rect)
    		.expect("Failed to fill rect");
    	Display {
    		renderer : renderer,
    		pixels : [[false; rows as usize] ; cols as usize],
    		rect : rect,
    	}
    }

    fn draw_unit(&mut self, x: i32, y: i32){
    	self.rect.set_x(x);
    	self.rect.set_y(y);
    	self.renderer.draw_rect(self.rect)
    		.expect("Fail to draw");
    }

    pub fn update_display(&mut self, x: i32, y: i32, val: bool) -> bool{
    	let old = self.pixels[x as usize][y as usize];
    	self.pixels[x as usize][y as usize] ^= val;
    	old == self.pixels[x as usize][y as usize]
    }

    pub fn clear(&mut self){
    	self.pixels = [[false; rows as usize] ; cols as usize];
    	self.renderer.set_draw_color(Color::RGB(0,0,0));
    	self.renderer.clear();
	   	self.renderer.set_draw_color(Color::RGB(255,255,255));
    }

    pub fn draw(&mut self){
    	for x in 0..cols {
    	    for y in 0..rows {
    	    	if self.pixels[x as usize][y as usize] {
    	    		self.draw_unit(x*8,y*8);
    	    	}
    	    }
    	}
    	self.renderer.present();
    }

}