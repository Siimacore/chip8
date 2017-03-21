use std::fs::File;
use std::io::Read;
use std::iter::Enumerate;
use rand;
use rand::Rng;
use display::Display;
use std::mem::transmute;

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xe0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0x80, // C
    0xF0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Cpu<'a> {
	memory: Box<[u8; 4096]>,
	opcode: u16,
	v: [u8; 16],
	i: u16,
	delay: u8,
	sound: u8,
	pc: u16,
	sp: u8,
	stack: [u16; 16],
	screen: Display<'a>,
}

impl<'a> Cpu<'a> {

	pub fn new() -> Self {
		let mut memory = Box::new([0; 4096]);
		memory[0..80].copy_from_slice(&FONT[0..80]);
		Cpu {
			memory: memory,
			opcode: 0,
			v: [0; 16],
			i: 0,
			delay: 0,
			sound: 0,
			pc: 512,
			sp: 0,
			stack: [0; 16],
			screen: Display::new(),
		}
	}

	pub fn load_rom(&mut self, file: &str){
		let mut file = File::open(file).unwrap();
		let mut buf = Vec::new();
		file.read_to_end(&mut buf).unwrap();

		let len = buf.len();
		for i in 0..len {
			self.memory[i + 512] = buf[i];
			println!("{:?}", (i+512, buf[i], self.memory[i+512]));
		}
	}

	pub fn get_opcode(&mut self){
		self.opcode = (self.memory[self.pc as usize] as u16) << 8
					| (self.memory[self.pc as usize + 1] as u16);
		self.pc += 2;
	}

	fn treat_0_opcode(&mut self){
		match self.opcode {
		    0x00E0 => self.screen.clear(),
		    0x00EE => {
		    	println!("{:?}", "test");
		    	self.sp -= 1;
		    	self.pc = self.stack[self.sp as usize];
		    },
		    _      => {},
		}
	}

	fn treat_1_opcode(&mut self){
		self.pc = self.opcode & 0x0fff;
	}

	fn treat_2_opcode(&mut self){
		self.stack[self.sp as usize] = self.pc;
		self.sp += 1;
		self.pc = self.opcode & 0x0fff;
	}

	fn treat_3_opcode(&mut self){
		let kk = (self.opcode & 0x00ff) as u8;
		let x = ((self.opcode & 0x0f00)  >> 8) as usize;
		if self.v[x] == kk {
			self.pc += 2;
		}
	}

	fn treat_4_opcode(&mut self){
		let kk = (self.opcode & 0x00ff) as u8;
		let x = ((self.opcode & 0x0f00)  >> 8) as usize;
		if self.v[x] != kk {
			self.pc += 2;
		}
	}

	fn treat_5_opcode(&mut self) {
		let x = ((self.opcode & 0x0f00) >> 8) as usize;
		let y = ((self.opcode & 0x00f0) >> 4) as usize;
		if self.v[x] == self.v[y]{
			self.pc += 2;
		}
	}

	fn treat_6_opcode(&mut self){
		let kk = (self.opcode & 0x00ff) as u8;
		let x = ((self.opcode & 0x0f00)  >> 8) as usize;
		self.v[x] = kk;
	}

	fn treat_7_opcode(&mut self){
		let kk = (self.opcode & 0x00ff) as u8;
		let x = ((self.opcode & 0x0f00)  >> 8) as usize;
		self.v[x] = self.v[x].wrapping_add(kk);
	}

	fn treat_8_opcode(&mut self){
		let x = ((self.opcode & 0x0f00) >> 8) as usize;
		let y = ((self.opcode & 0x00f0) >> 4) as usize;
		match self.opcode & 0x000f {
			0 => {
				self.v[x] = self.v[y];
			},
			1 => {
				self.v[x] |= self.v[y];
			},
			2 => {
				self.v[x] &= self.v[y];
			},
			3 => {
				self.v[x] ^= self.v[y];
			},
			4 => {
				if self.v[x] + self.v[y] > 255 {
					self.v[0xf] = 1;
				}
				self.v[x] = self.v[x].wrapping_add(self.v[y]);
			},
			5 => {
				self.v[0xf] = 0;
				if self.v[x] > self.v[y] {
					self.v[0xf] = 1;
				}
				self.v[x] = self.v[x].wrapping_sub(self.v[y]);
			}
			6 => {
				self.v[0xf] = self.v[x] & 1;
				self.v[x] >>= 1;
			},
			7 => {
				self.v[0xf] = 0;
				if self.v[y] > self.v[x] {
					self.v[0xf] = 1;
				}
				self.v[y] = self.v[y].wrapping_sub(self.v[x]);
			},
			0xE => {
				self.v[0xf] = (self.v[x] >> 7) & 1;
				self.v[x] <<= 1;
			},
			_ => unimplemented!(),
		}
	}

	fn treat_9_opcode(&mut self) {
		let x = ((self.opcode & 0x0f00) >> 8) as usize;
		let y = ((self.opcode & 0x00f0) >> 4) as usize;
		if self.v[x] != self.v[y] {
			self.pc += 2;
		}
	}
	fn treat_a_opcode(&mut self){
		self.i = self.opcode & 0x0fff;
	}


	fn treat_b_opcode(&mut self){
		self.pc = (self.v[0] as u16 + (self.opcode & 0x0fff)) as u16;
	}


	fn treat_c_opcode(&mut self){
		let mut rng = rand::thread_rng();
		let random = rng.gen_range(0, 255) as u8;
		let kk = (self.opcode & 0x00ff) as u8;
		let x = ((self.opcode & 0x0f00)  >> 8) as usize;
		self.v[x] = random & kk;
	}

	fn treat_d_opcode(&mut self){
		let n = (self.opcode & 0x000f) as u8;
		let y = ((self.opcode & 0x00f0) >> 4) as usize;
		let x = ((self.opcode & 0x0f00) >> 8) as usize;
		self.v[0xf] = 0;
		let xco = self.v[x];
		let yco = self.v[y];
		for i in 0..n {
			let sprite = self.memory[(self.i + i as u16) as usize];
			println!("{:?}", (sprite));
			for j in 0..8 {
				println!("{:?}", 8-j);
				if self.screen.update_display(((xco + j) % 64) as i32, ((yco + i) % 32) as i32, (sprite & (1 << (7 - j)) > 0)) {
					self.v[0xf] = 1;
				}
			}
		}
		self.screen.draw();
	}

	fn treat_f_opcode(&mut self){
		let x = ((self.opcode & 0x0f00) >> 8) as usize;
		match self.opcode & 0x00ff {
			0x07 => self.v[x] = self.delay,
			0x15 => self.delay = self.v[x],
			0x18 => self.sound = self.v[x],
			0x1E => {
				self.i += self.v[x] as u16;
				println!("{:?}", self.i);
			},
			0x29 => {
				self.i = (self.v[x] as usize * 5) as u16;
				println!("{:?}", (self.i));
			},
			0x33 => {
				self.memory[self.i as usize] = self.v[x] / 100;
				self.memory[(self.i + 1) as usize] = (self.v[x] / 10) - ((self.v[x]/100) * 10);
				self.memory[(self.i + 2) as usize] = (self.v[x]) - ((self.v[x]/10) * 10);
			},
			0x55 => {
				for i in 0..x {
				    self.memory[self.i as usize + i] = self.v[i];
				}
			},
			0x65 => {
				for i in 0..x {
				    self.v[i] = self.memory[self.i as usize + i];
				}
			}
			_ => unimplemented!(),
		}

	}


	pub fn treat_opcode(&mut self) {
		println!("{:?}", (self.opcode, self.pc));
		match (self.opcode & 0xf000) >> 12{
			0x0 => self.treat_0_opcode(),
			0x1 => self.treat_1_opcode(),
			0x2 => self.treat_2_opcode(),
			0x3 => self.treat_3_opcode(),
			0x4 => self.treat_4_opcode(),
			0x5 => self.treat_5_opcode(),
			0x6 => self.treat_6_opcode(),
			0x7 => self.treat_7_opcode(),
			0x8 => self.treat_8_opcode(),
			0x9 => self.treat_9_opcode(),
			0xA => self.treat_a_opcode(),
			0xB => self.treat_b_opcode(),
			0xC => self.treat_c_opcode(),
			0xD => self.treat_d_opcode(),
			0xF => self.treat_f_opcode(),
			_ => println!("{:X}", self.opcode)	,
		}
		if self.delay > 0 {
			self.delay -= 1;
		}
		if self.sound > 0 {
			self.sound -= 1;
		}

	}
}
