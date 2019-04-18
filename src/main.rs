extern crate gameboy;

use gameboy::cpu::instructions::Instruction::*;
use gameboy::rom::*;

fn main() {
    let mut rom: ROM = Default::default();
    println!("{}", rom);
    rom.load_rom();
    println!("{}", rom);
}
