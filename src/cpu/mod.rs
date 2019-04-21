#![allow(dead_code)]

pub mod opcodes;

use std::fmt; 
use crate::mmu::MMUnit;
use self::opcodes::*;

#[derive(Default)]
pub struct CPU {
    reg: Registers,
}

/*
 *  TODO: May be able to refactor this using RefCell to get interior mutability.
 *  Reading: https://ricardomartins.cc/2016/06/08/interior-mutability
 */
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
struct Registers {
    /*
     *      Registers are all 16 bit, CPU is 8-bit. Individual registers
     *      are referenced/modified with masking.
     *
     *      Flag (F Register):
     *          Z N H C 0 0 0 0
     */
    PC: u16,
    SP: u16,
    A: u8,
    B: u8,
    C: u8,
    D: u8,
    E: u8,
    F: u8,
    H: u8,
    L: u8,
}

impl Registers {
    pub fn get_af(&self) -> u16 {
        u16::from(self.A) << 8 | u16::from(self.F) 
    }
    pub fn set_af(&mut self, val: u16) {
        self.A = (val >> 8) as u8; 
        self.F = (val & 0x0F) as u8; 
    }
    pub fn get_bc(&self) -> u16 {
        u16::from(self.B) << 8 | u16::from(self.C) 
    }
    pub fn set_bc(&mut self, val: u16) {
        self.B = (val >> 8) as u8; 
        self.C = (val & 0x0F) as u8; 
    }
    pub fn get_de(&self) -> u16 {
        u16::from(self.D) << 8 | u16::from(self.E) 
    }
    pub fn set_de(&mut self, val: u16) {
        self.D = (val >> 8) as u8; 
        self.E = (val & 0x0F) as u8; 
    }
    pub fn get_hl(&self) -> u16 {
        u16::from(self.H) << 8 | u16::from(self.L) 
    }
    pub fn set_hl(&mut self, val: u16) {
        self.H = (val >> 8) as u8; 
        self.L = (val & 0x0F) as u8; 
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(PC: {}, SP: {}, A: {}, B: {}, C: {}, D: {}, E: {}, F: {}, H: {}, L: {}, AF: {}, BC: {}, DE: {}, HL: {})", 
            self.reg.PC, self.reg.SP, self.reg.A, self.reg.B, self.reg.C, self.reg.D,
            self.reg.E, self.reg.F, self.reg.H, self.reg.L, self.reg.get_af(), self.reg.get_bc(), 
            self.reg.get_de(), self.reg.get_hl()
            )
    }
}

impl Default for Registers {
    fn default() -> Registers {
        Registers {
            PC: 0x100,
            SP: 0xFFFE,      /* Should be the highest available address in memory. Decrements before putting something on the stack. */
            A: 0,
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            F: 0,
            H: 0,
            L: 0,
        }
    }
}

impl CPU {

    fn imm(&mut self, mem: &mut MMUnit) -> u8 {
        let val = mem.get(self.reg.PC);
        self.reg.PC += 1;
        val
    }

    fn imm_hw(&mut self, mem: &mut MMUnit) -> u16 {
        let val = mem.get_hw(self.reg.PC);
        self.reg.PC += 2;
        val
    }

    fn stack_push(&mut self, mem: &mut MMUnit, val: u16) {
        self.reg.SP -= 2;
        mem.set_hw(self.reg.SP, val);
    }

    fn stack_pop(&mut self, mem: &mut MMUnit) -> u16 {
        let val = mem.get_hw(self.reg.SP);
        self.reg.SP += 2;
        val
    }
   

    pub fn toggle_flag(&mut self, flag: &str) {
        match flag {
            "Z" => {
                self.reg.F ^= 1 << 7
            },
            "N" => {
                self.reg.F ^= 1 << 6
            },
            "H" => {
                self.reg.F ^= 1 << 5
            },
            "C" => {
                self.reg.F ^= 1 << 4
            },
            _ => println!("Couldn't match a flag!"),
        }
    }

    pub fn flag_z(&self) -> bool {
        // is Zero Flag set?
        (self.reg.F & 1 << 7) != 0
    }
    pub fn flag_n(&self) -> bool {
        // is Subtract Flag set?
        (self.reg.F & 1 << 6) != 0
    }
    pub fn flag_h(&self) -> bool {
        // is Half Carry Flag set?
        (self.reg.F & 1 << 5) != 0
    }
    pub fn flag_c(&self) -> bool {
        // is Carry Flag set?
        (self.reg.F & 1 << 4) != 0
    }

    #[allow(non_snake_case)]
    pub fn decode(&mut self) {  // TODO: Need a stream of instructions here
        let a = OpCode::from_u32(0);
        match a {
            Some(NOP) => {
                println!("Found a NOP");
            },
            Some(LD_BC_d16) => {
                println!("Found LD_BC_d16");
            },
            Some(LD_BC_ptr_d16) => {
                println!("Found LD_BC_ptr_d16");
            },
            Some(INC_BC) => {
                println!("Found INC_BC");
                self.reg.set_bc(self.reg.get_bc() + 1);
            },
            Some(INC_B) => {
                println!("Found INC_B");
                self.reg.B += 1;
            },
            Some(DEC_B) => {
                println!("Found DEC_B");
                self.reg.B -= 1;
            },
            Some(LD_B_d8) => {
                println!("Found LD_B_d8");
            },
            Some(RLCA) => {
                println!("Found RLCA");
            },
            Some(LD_a16_ptr_SP) => {
                println!("Found LD_a16_ptr_SP");
            },
            Some(ADD_HL_BC) => {
                println!("Found ADD_HL_BC");
            },
            Some(LD_A_BC_ptr) => {
                println!("Found LD_A_BC_ptr");
            },
            Some(DEC_BC) => {
                println!("Found DEC_BC");
                self.reg.set_bc(self.reg.get_bc() - 1);
            },
            Some(INC_C) => {
                println!("Found INC_C");
                self.reg.B += 1;
            },
            Some(DEC_C) => {
                println!("Found DEC_C");
                self.reg.B -= 1;
            },
            Some(LD_C_d8) => {
                println!("Found LD_C_d8");
            },
            Some(RRCA) => {
                println!("Found RRCA");
            },
            _ => println!("Dunno what I found"),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::CPU;

    #[test]
    fn test_toggle_flags() {
        let mut a: CPU = Default::default();
        assert_eq!(a.flag_n(), false);
        assert_eq!(a.flag_z(), false);
        assert_eq!(a.flag_c(), false);
        assert_eq!(a.flag_h(), false);
        a.toggle_flag("N");
        assert_eq!(a.flag_n(), true);
        assert_eq!(a.flag_z(), false);
        assert_eq!(a.flag_c(), false);
        assert_eq!(a.flag_h(), false);
        a.toggle_flag("N");
        assert_eq!(a.flag_n(), false);
        assert_eq!(a.flag_z(), false);
        assert_eq!(a.flag_c(), false);
        assert_eq!(a.flag_h(), false);
        a.toggle_flag("Z");
        assert_eq!(a.flag_n(), false);
        assert_eq!(a.flag_z(), true);
        assert_eq!(a.flag_c(), false);
        assert_eq!(a.flag_h(), false);
        a.toggle_flag("Z");
        assert_eq!(a.flag_n(), false);
        assert_eq!(a.flag_z(), false);
        assert_eq!(a.flag_c(), false);
        assert_eq!(a.flag_h(), false);
        a.toggle_flag("asdlfkj");
        assert_eq!(a.flag_n(), false);
        assert_eq!(a.flag_z(), false);
        assert_eq!(a.flag_c(), false);
        assert_eq!(a.flag_h(), false);
    }
}
