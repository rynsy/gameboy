extern crate gameboy;

use gameboy::cpu::instructions::Instruction;
use gameboy::rom::*;

fn main() {
    let a = Instruction::from_u32(0);
    match a {
        Some(Instruction::NOP) => println!("Found a NOP"),
        _ => println!("Dunno what I found"),
    }
}
