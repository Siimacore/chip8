use std::fs::File;
use std::io::Read;
use std::iter::Enumerate;
use rand;
use rand::Rng;

pub struct Cpu {
	memory: Box<[u8; 4096]>,
	opcode: u16,
	v: [u8; 16],
	i: u16,
	delay: u8,
	sound: u8,
	pc: u16,
	sp: u8,
	stack: [u16; 16], 
}

impl Cpu {

	pub fn new() -> Self {
		Cpu {
			memory: Box::new([0; 4096]),
			opcode: 0,
			v: [0; 16],
			i: 0,
			delay: 0,
			sound: 0,
			pc: 512,
			sp: 0,
			stack: [0; 16],
		}
	}

	pub fn load_rom(&mut self, file: &str){
		let mut file = File::open(file).unwrap();
		let mut buf = vec![0; 10];
		file.read_to_end(&mut buf).unwrap();

		let len = buf.len();

		for i in 0..len {
			println!("{} : {}", i, buf[i]);
			self.memory[i + 512] = buf[i];
		}
	}

	pub fn get_opcode(&mut self){
		self.opcode = (self.memory[self.pc as usize] as u16) << 8
					| (self.memory[self.pc as usize + 1] as u16);
		self.pc += 2;
	}

	fn treat_a_opcode(&mut self){
		self.i = self.opcode & 0x0fff;
	}

	fn treat_0_opcode(&mut self){
		match self.opcode {
		    0x00E0 => println!("CLS"),
		    0x00EE => {
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

	fn treat_7_opcode(&mut self){
		let kk = (self.opcode & 0x00ff) as u8;
		let x = ((self.opcode & 0x0f00)  >> 8) as usize;
		self.v[x] += kk;
	}

	fn treat_c_opcode(&mut self){
		let mut rng = rand::thread_rng();
		let random = rng.gen_range(0, 255) as u8;
		let kk = (self.opcode & 0x00ff) as u8;
		let x = ((self.opcode & 0x0f00)  >> 8) as usize;
		self.v[x] = random & kk;
	}

	fn treat_d_opcode(&mut self){

	}


	pub fn treat_opcode(&mut self) {
		match (self.opcode & 0xf000) >> 12{
			0x0 => self.treat_0_opcode(),
			0x1 => self.treat_1_opcode(),
			0x2 => self.treat_2_opcode(),
			0x3 => self.treat_3_opcode(),
			0x7 => self.treat_7_opcode(),
			0xA => self.treat_a_opcode(),
			0xC => self.treat_c_opcode(),
			//0xD => self.treat_d_opcode(),
			_ => println!("{:X}", self.opcode)	,
		}
	}
}