extern crate gameboy;

use std::fs;
use std::path::Path;
use gameboy::cpu::instructions::Instruction::*;

struct ROM {
    filename: String,
    title: String,
    data: Vec<u8>,
}

fn load_rom() -> ROM {
    let dir = String::from("C:\\Code\\rust\\gameboy\\data\\cpu_instrs\\individual\\01-special.gb");
    let rom_data: Vec<u8> = fs::read(dir).expect("Unable to read file"); 
    ROM {
        filename: "test_file".to_string(),
        title: "test".to_string(),
        data: rom_data,
    }
}

fn main() {
    println!("{:?}", NOP);
}
