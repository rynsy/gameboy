extern crate gameboy;

use gameboy::gb::GameBoy;

fn main() {
    let mut a = GameBoy::default();
    a.load_rom();
}
