use crate::mmu::MMUnit;
use crate::cpu::CPU;

#[derive(Default)]
pub struct GameBoy {
    cpu: CPU,
    mmu: MMUnit,
}
