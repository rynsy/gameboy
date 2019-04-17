extern crate gameboy;

use std::fs;
use std::path::Path;
//use gameboy::cpu::cpu::*;

fn main() {
    /*
     * TODO: Open file in test directory.
     */
    let dir = String::from("C:\\Code\\rust\\gameboy\\data\\cpu_instrs\\individual\\01-special.gb");
    let data: Vec<u8> = fs::read(dir).expect("Unable to read file"); 
    println!("{:?}", data);
}
