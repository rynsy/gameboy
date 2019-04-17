extern crate gameboy;

use gameboy::cpu::cpu::*;

fn main() {
    let mut a: CPU = Default::default();
    a.load("AF", 1);
    println!("{}", a);
}
