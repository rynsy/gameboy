use crate::mmap::MMap;
use crate::cpu::CPU;

#[derive(Default)]
pub struct GameBoy {
    cpu: CPU,
    mmap: MMap,
}
