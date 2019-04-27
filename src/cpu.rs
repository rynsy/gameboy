#![allow(dead_code)]

use std::fmt; 
use crate::mmu::MMUnit;
use crate::register::Register;
use crate::register::Flag;
use crate::register::Flag::{Z,H,N,C};

#[derive(Default)]
pub struct CPU {
    reg: Register,
    pub mem: MMUnit,    //TODO not sure it needs to be public. 
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
        self.set_flag(Z, res == 0);
        self.set_flag(N, false);
        self.set_flag(H, ((a & 0xF) + (v & 0xF) + carry) > 0xF);
        self.set_flag(C, (u16::from(a) + u16::from(v) + u16::from(carry)) > 0xFF);
        self.reg.a = res;
    }

    fn alu_sub(&mut self, v: u8) {
        let carry = if self.flag_c() { 1 } else { 0 };
        let a = self.reg.a;
        let res = a.wrapping_sub(v).wrapping_sub(carry);
        self.set_flag(Z, res == 0);
        self.set_flag(N, true);
        self.set_flag(H, (a & 0x0F) < ((v & 0x0F) + carry));
        self.set_flag(C, u16::from(a) < (u16::from(v) + u16::from(carry)));
        self.reg.a = res;
    }

    fn alu_and(&mut self, v: u8) {
        let res = self.reg.a & v;
        self.set_flag(Z, res == 0);
        self.set_flag(N, false);
        self.set_flag(H, true);
        self.set_flag(C, false);
        self.reg.a = res;
    }
    
    fn alu_or(&mut self, v: u8) {
        let res = self.reg.a | v;
        self.set_flag(Z, res == 0);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, false);
        self.reg.a = res;
    }

    fn alu_xor(&mut self, v: u8) {
        let res = self.reg.a | v;
        self.set_flag(Z, res == 0);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, false);
        self.reg.a = res;
    }

    fn alu_cp(&mut self, v: u8) {
        let res = self.reg.a;
        self.alu_sub(v);
        self.reg.a = res;
    }

    fn alu_dec(&mut self, v: u8) -> u8 {
        let res = v.wrapping_add(1);
        self.set_flag(Z, res == 0);
        self.set_flag(N, true);
        self.set_flag(H, (res & 0xF) == 0);
        res
    }

    fn alu_inc(&mut self, v: u8) -> u8 {
        let res = v.wrapping_add(1);
        self.set_flag(Z, res == 0);
        self.set_flag(H, ((res & 0xF) + 1) > 0xF );
        self.set_flag(N, false);
        res
    }

    fn alu_add_hw(&mut self, v: u16) {
        let a = self.reg.get_hl();
        let res = a.wrapping_add(v);
        self.set_flag(N, false);
        self.set_flag(H, ((a & 0x07FF) + (v & 0x07FF)) > 0x07FF );
        self.set_flag(C, a > (0xFFFF - v));
        self.reg.set_hl(res);
    }

    fn alu_add_hw_imm(&mut self, v: u16) -> u16 {
        let a = self.imm() as i8 as i16 as u16;  // TODO This is signed, need to convert
        self.set_flag(Z, false);
        self.set_flag(N, false);
        self.set_flag(H, ((a & 0x000F) + (v & 0x000F)) > 0x000F );
        self.set_flag(C, ((a & 0x00FF) + (v & 0x00FF)) > 0x00FF );
        a.wrapping_add(v)
    }

    fn alu_inc_hw(&mut self, v: u16) -> u16 {
        v.wrapping_add(1)    
    }

    fn alu_dec_hw(&mut self, v: u16) -> u16 {
        v.wrapping_sub(1)    
    }

    fn alu_swap(&mut self, v: u8) -> u8 {
        self.set_flag(Z, v == 0);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, false);
        (v >> 4) | (v << 4)
    }

    fn alu_daa(&mut self) {
        // decimal adjust register A
        let mut a = self.reg.a;
        let mut adjust = if self.flag_c() { 0x60 } else { 0x00 };
        if self.flag_h() { adjust |= 0x06; }

        if !self.flag_n() {
            if (a & 0x0F) > 0x09 { adjust |= 0x06 }
            if a > 0x99 { adjust |= 0x60 }
            a = a.wrapping_add(adjust);
        } else {
            a = a.wrapping_sub(adjust);
        }

        self.set_flag(Z, a == 0);
        self.set_flag(H, false);
        self.set_flag(C, adjust >= 0x60);
    }

    fn alu_cpl(&mut self) {
        self.reg.a = !(self.reg.a);
    }

    fn alu_ccf(&mut self) {
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, !self.flag_c());
    }

    fn alu_scf(&mut self) {
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, true);
    }

    fn alu_rlca(&mut self) {
        //TODO writeme
        self.set_flag(Z, false);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, true);
    }

    fn alu_rla(&mut self) {
        //TODO writeme
        self.set_flag(Z, false);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, true);
    }

    fn alu_rrca(&mut self) {
        //TODO writeme
        self.set_flag(Z, false);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, true);
    }

    fn alu_rra(&mut self) {
        //TODO writeme
        self.set_flag(Z, false);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, true);
    }

    fn alu_rlc(&mut self, v: u8) -> u8 {
        //TODO writeme
        self.set_flag(Z, false);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, true);
        0
    }

    fn alu_rl(&mut self, v: u8) -> u8 {
        //TODO writeme
        self.set_flag(Z, false);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, true);
        0
    }

    fn alu_rrc(&mut self, v: u8) -> u8 {
        //TODO writeme
        self.set_flag(Z, false);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, true);
        0
    }

    fn alu_rr(&mut self, v: u8) -> u8 {
        //TODO writeme
        self.set_flag(Z, false);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, true);
        0
    }

    fn alu_sla(&mut self, v: u8) -> u8 {
        //TODO writeme
        self.set_flag(Z, false);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, true);
        0
    }

    fn alu_sra(&mut self, v: u8) -> u8 {
        //TODO writeme
        self.set_flag(Z, false);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, true);
        0
    }

    fn alu_srl(&mut self, v: u8) -> u8 {
        //TODO writeme
        self.set_flag(Z, false);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, true);
        0
    }

    fn alu_bit(&mut self, b: u8, r: u8) -> bool{
        //TODO writeme
        self.set_flag(Z, false);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, true);
        true
    }

    pub fn set_flag(&mut self, flag: Flag, val: bool) {
        let shift_val = match flag {
            Z => 7,
            N => 6,
            H => 5,
            C => 4,
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
        a.set_flag(N, true);
        assert_eq!(a.flag_n(), true);
        assert_eq!(a.flag_z(), false);
        assert_eq!(a.flag_c(), false);
        assert_eq!(a.flag_h(), false);
        a.set_flag(N, false);
        assert_eq!(a.flag_n(), false);
        assert_eq!(a.flag_z(), false);
        assert_eq!(a.flag_c(), false);
        assert_eq!(a.flag_h(), false);
        a.set_flag(Z, true);
        assert_eq!(a.flag_n(), false);
        assert_eq!(a.flag_z(), true);
        assert_eq!(a.flag_c(), false);
        assert_eq!(a.flag_h(), false);
        a.set_flag(Z, false);
        assert_eq!(a.flag_n(), false);
        assert_eq!(a.flag_z(), false);
        assert_eq!(a.flag_c(), false);
        assert_eq!(a.flag_h(), false);
    }
}
