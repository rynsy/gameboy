#![allow(dead_code)]

use std::fmt; 
use crate::mmu::MMUnit;

#[derive(Default)]
pub struct CPU {
    reg: Registers,
    pub mem: MMUnit,    //TODO not sure it needs to be public. 
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
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

pub enum Flag {
    Z,
    N,
    H,
    C,
}

impl Registers {
    pub fn get_af(&self) -> u16 {
        u16::from(self.a) << 8 | u16::from(self.f) 
    }
    pub fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8; 
        self.f = (val & 0x0F) as u8; 
    }
    pub fn get_bc(&self) -> u16 {
        u16::from(self.b) << 8 | u16::from(self.c) 
    }
    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8; 
        self.c = (val & 0x0F) as u8; 
    }
    pub fn get_de(&self) -> u16 {
        u16::from(self.d) << 8 | u16::from(self.e) 
    }
    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8; 
        self.e = (val & 0x0F) as u8; 
    }
    pub fn get_hl(&self) -> u16 {
        u16::from(self.h) << 8 | u16::from(self.l) 
    }
    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8; 
        self.l = (val & 0x0F) as u8; 
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(PC: {}, SP: {}, A: {}, B: {}, C: {}, D: {}, E: {}, F: {}, H: {}, L: {}, AF: {}, BC: {}, DE: {}, HL: {})", 
            self.reg.pc, self.reg.sp, self.reg.a, self.reg.b, self.reg.c, self.reg.d,
            self.reg.e, self.reg.f, self.reg.h, self.reg.l, self.reg.get_af(), self.reg.get_bc(), 
            self.reg.get_de(), self.reg.get_hl()
            )
    }
}

impl Default for Registers {
    fn default() -> Registers {
        Registers {
            pc: 0x100,
            sp: 0xFFFE,      /* Should be the highest available address in memory. Decrements before putting something on the stack. */
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
        }
    }
}

impl CPU {

    fn imm(&mut self) -> u8 {
        let val = self.mem.get(self.reg.pc);
        self.reg.pc += 1;
        val
    }

    fn imm_hw(&mut self) -> u16 {
        let val = self.mem.get_hw(self.reg.pc);
        self.reg.pc += 2;
        val
    }

    fn stack_push(&mut self, val: u16) {
        self.reg.sp -= 2;
        self.mem.set_hw(self.reg.sp, val);
    }

    fn stack_pop(&mut self) -> u16 {
        let val = self.mem.get_hw(self.reg.sp);
        self.reg.sp += 2;
        val
    }
  
    fn alu_add(&mut self, v: u8) {
        let carry = if self.flag_c() { 1 } else { 0 };
        let a = self.reg.a;
        let res = a.wrapping_add(v).wrapping_add(carry);
        self.set_flag(Flag::Z, res == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, ((a & 0xF) + (v & 0xF) + carry) > 0xF);
        self.set_flag(Flag::C, (u16::from(a) + u16::from(v) + u16::from(carry)) > 0xFF);
        self.reg.a = res;
    }

    fn alu_sub(&mut self, v: u8) {
        let carry = if self.flag_c() { 1 } else { 0 };
        let a = self.reg.a;
        let res = a.wrapping_sub(v).wrapping_sub(carry);
        self.set_flag(Flag::Z, res == 0);
        self.set_flag(Flag::N, true);
        self.set_flag(Flag::H, (a & 0x0F) < ((v & 0x0F) + carry));
        self.set_flag(Flag::C, u16::from(a) < (u16::from(v) + u16::from(carry)));
        self.reg.a = res;
    }

    fn alu_and(&mut self, v: u8) {
        let res = self.reg.a & v;
        self.set_flag(Flag::Z, res == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, true);
        self.set_flag(Flag::C, false);
        self.reg.a = res;
    }
    
    fn alu_or(&mut self, v: u8) {
        let res = self.reg.a | v;
        self.set_flag(Flag::Z, res == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, false);
        self.set_flag(Flag::C, false);
        self.reg.a = res;
    }

    fn alu_xor(&mut self, v: u8) {
        let res = self.reg.a | v;
        self.set_flag(Flag::Z, res == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, false);
        self.set_flag(Flag::C, false);
        self.reg.a = res;
    }

    fn alu_cp(&mut self, v: u8) {
        let res = self.reg.a;
        self.alu_sub(v);
        self.reg.a = res;
    }

    fn alu_dec(&mut self, v: u8) -> u8 {
        let res = v.wrapping_add(1);
        self.set_flag(Flag::Z, res == 0);
        self.set_flag(Flag::N, true);
        self.set_flag(Flag::H, (res & 0xF) == 0);
        res
    }

    fn alu_inc(&mut self, v: u8) -> u8 {
        let res = v.wrapping_add(1);
        self.set_flag(Flag::Z, res == 0);
        self.set_flag(Flag::H, ((res & 0xF) + 1) > 0xF );
        self.set_flag(Flag::N, false);
        res
    }

    fn alu_add_hw(&mut self, v: u16) {
        let a = self.reg.get_hl();
        let res = a.wrapping_add(v);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, ((a & 0x07FF) + (v & 0x07FF)) > 0x07FF );
        self.set_flag(Flag::C, a > (0xFFFF - v));
        self.reg.set_hl(res);
    }

    fn alu_add_hw_imm(&mut self, v: u16) -> u16 {
        let a = self.imm() as i8 as i16 as u16;  // TODO This is signed, need to convert
        self.set_flag(Flag::Z, false);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, ((a & 0x000F) + (v & 0x000F)) > 0x000F );
        self.set_flag(Flag::C, ((a & 0x00FF) + (v & 0x00FF)) > 0x00FF );
        a.wrapping_add(v)
    }

    fn alu_inc_hw(&mut self, v: u16) -> u16 {
        v.wrapping_add(1)    
    }

    fn alu_dec_hw(&mut self, v: u16) -> u16 {
        v.wrapping_sub(1)    
    }

    fn alu_swap(&mut self, v: u8) -> u8 {
        self.set_flag(Flag::Z, v == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, false);
        self.set_flag(Flag::C, false);
        (v >> 4) | (v << 4)
    }

    pub fn set_flag(&mut self, flag: Flag, val: bool) {
        let shift_val = match flag {
            Flag::Z => 7,
            Flag::N => 6,
            Flag::H => 5,
            Flag::C => 4,
        };
        if val {
            self.reg.f |= 1 << shift_val;
        } else {
            self.reg.f &= !(1 << shift_val);
        }
    }

    pub fn flag_z(&self) -> bool {
        // is Zero Flag set?
        (self.reg.f & 1 << 7) != 0
    }
    pub fn flag_n(&self) -> bool {
        // is Subtract Flag set?
        (self.reg.f & 1 << 6) != 0
    }
    pub fn flag_h(&self) -> bool {
        // is Half Carry Flag set?
        (self.reg.f & 1 << 5) != 0
    }
    pub fn flag_c(&self) -> bool {
        // is Carry Flag set?
        (self.reg.f & 1 << 4) != 0
    }

    #[allow(non_snake_case)]
    pub fn ex(&mut self) {
        let op = self.imm();
        match op {
            0x00 => { 1; },
            0x01 => { 1; },
            _ => println!("Dunno what I found"),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_flags() {
        let mut a: CPU = Default::default();
        assert_eq!(a.flag_n(), false);
        assert_eq!(a.flag_z(), false);
        assert_eq!(a.flag_c(), false);
        assert_eq!(a.flag_h(), false);
        a.set_flag(Flag::N, true);
        assert_eq!(a.flag_n(), true);
        assert_eq!(a.flag_z(), false);
        assert_eq!(a.flag_c(), false);
        assert_eq!(a.flag_h(), false);
        a.set_flag(Flag::N, false);
        assert_eq!(a.flag_n(), false);
        assert_eq!(a.flag_z(), false);
        assert_eq!(a.flag_c(), false);
        assert_eq!(a.flag_h(), false);
        a.set_flag(Flag::Z, true);
        assert_eq!(a.flag_n(), false);
        assert_eq!(a.flag_z(), true);
        assert_eq!(a.flag_c(), false);
        assert_eq!(a.flag_h(), false);
        a.set_flag(Flag::Z, false);
        assert_eq!(a.flag_n(), false);
        assert_eq!(a.flag_z(), false);
        assert_eq!(a.flag_c(), false);
        assert_eq!(a.flag_h(), false);
    }
}
