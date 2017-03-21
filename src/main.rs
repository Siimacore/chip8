mod cpu;

extern crate rand;


use cpu::Cpu;

fn main() {
	let mut cpu = Cpu::new();
	cpu.load_rom("res/Maze.ch8");
	for i in 0..50 {
	    cpu.get_opcode();
	    cpu.treat_opcode();
	}
}
