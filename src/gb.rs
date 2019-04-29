use super::cpu::CPU;

#[derive(Default)]
pub struct GameBoy {
    cpu: CPU,
}

impl GameBoy {
    pub fn load_rom(&mut self) {
        self.cpu.mem.load_rom();
    }
}
