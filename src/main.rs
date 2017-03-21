pub mod cpu;
pub mod display;

extern crate rand;

use cpu::Cpu;
use std::{thread, time};



fn main() {
	let mut cpu = Cpu::new();
	let mut cpt = 0;
	cpu.load_rom("res/Maze.ch8");
	//let ten_millis = time::Duration::from_millis();
	loop {
	    cpu.get_opcode();
	    cpu.treat_opcode();
	    //thread::sleep(ten_millis);
	}
}
