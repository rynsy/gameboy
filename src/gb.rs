use crate::mmu::MMUnit;
use crate::cpu::CPU;

#[derive(Default)]
pub struct GameBoy {
    pub cpu: CPU,
    pub mmu: MMUnit,
}
