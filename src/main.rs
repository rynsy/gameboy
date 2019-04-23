extern crate gameboy;

use gameboy::gb::GameBoy;

fn main() {
    let mut a = GameBoy::default();
    a.mmu.load_rom();
}
