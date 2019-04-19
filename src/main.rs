extern crate gameboy;

use gameboy::cpu::opcodes::OpCode;
use gameboy::rom::*;

fn main() {
    let a = OpCode::from_u32(0);
    match a {
        Some(OpCode::NOP) => println!("Found a NOP"),
        _ => println!("Dunno what I found"),
    }
}
