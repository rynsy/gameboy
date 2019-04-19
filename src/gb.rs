use crate::rom::ROM;
use crate::mmap::MMap;
use crate::cpu::CPU;

pub struct GameBoy {
    cpu: CPU,
    mmap: MMap,
    rom: ROM,
}

impl Default for GameBoy {
    fn default() -> GameBoy {
        GameBoy {
            cpu: Default::default(),
            mmap: Default::default(),
            rom: Default::default(),
        }
    }
}
