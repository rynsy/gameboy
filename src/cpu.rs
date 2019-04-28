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

    fn alu_add_hw_hl(&mut self, v: u16) {
        let a = self.reg.get_hl();
        let res = a.wrapping_add(v);
        self.set_flag(N, false);
        self.set_flag(H, ((a & 0x07FF) + (v & 0x07FF)) > 0x07FF );
        self.set_flag(C, a > (0xFFFF - v));
        self.reg.set_hl(res);
    }

    fn alu_add_hw_imm(&mut self, v: u16) -> u16 {
        let a = self.imm() as i8 as i16 as u16;  // TODO This is signed, test this conversion. 
        self.set_flag(Z, false);
        self.set_flag(N, false);
        self.set_flag(H, ((a & 0x000F) + (v & 0x000F)) > 0x000F );
        self.set_flag(C, ((a & 0x00FF) + (v & 0x00FF)) > 0x00FF );
        a.wrapping_add(v)
    }

    /*
     *  Swap upper/lower nibbles of n
     *  n = A,B,C,D,E,H,L,(HL)
     *
     *  Z - set if res = 0
     *  N - Reset
     *  H - Reset
     *  C - Reset
     */
    fn alu_swap(&mut self, v: u8) -> u8 {
        self.set_flag(Z, v == 0);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, false);
        (v >> 4) | (v << 4)
    }

    /*
     *  Decimal Adjust register A
     *  Adjust A so that it's in Binary Coded Decimal (BCD) format
     *
     *  Z - Set if A = 0
     *  N - N/A
     *  H - Reset 
     *  C - Set/Reset according to operation
     */
    fn alu_daa(&mut self) {
        // convert register A to BCD
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

    /*
     *  Complement Register A
     *
     *  Z - N/A
     *  N - Set
     *  H - Set 
     *  C - N/A
     */
    fn alu_cpl(&mut self) {
        self.reg.a = !(self.reg.a);
    }

    /*
     *  Complement Carry Flag (C = !C)
     *
     *  Z - N/A
     *  N - Reset
     *  H - Reset
     *  C - Complemented
     */
    fn alu_ccf(&mut self) {
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, !self.flag_c());
    }

    /*
     *  Set Carry Flag
     *
     *  Z - N/A
     *  N - Reset
     *  H - Reset
     *  C - Set
     */
    fn alu_scf(&mut self) {
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, true);
    }

    /*
     * Rotate A left, old bit 7 to carry flag
     *
     *  Z - Set if res = 0
     *  N - Reset
     *  H - Reset
     *  C - Contains old bit 7 data
     */
    fn alu_rlc(&mut self, v: u8) -> u8 {
        //TODO check this
        let c_res = (v & (1 << 7)) != 0;
        let res = v << 1;
        self.set_flag(Z, res == 0 );
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, c_res );
        res
    }

    /*
     * Rotate A left through carry flag
     *
     *  Z - Set if res = 0
     *  N - Reset
     *  H - Reset
     *  C - Contains old bit 0 data
     */
    fn alu_rl(&mut self, v: u8) -> u8 {
        let c_res = (v & (1 << 7)) != 0;
        let res = v << 1 + u8::from(self.flag_c());
        self.set_flag(Z, res == 0 );
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, c_res );
        res
    }

    /*
     * Rotate A right, old bit 0 to carry flag
     *
     *  Z - Set if res = 0
     *  N - Reset
     *  H - Reset
     *  C - Contains old bit 0 data
     */
    fn alu_rrc(&mut self, v: u8) -> u8 {
        let c_res = (v & 0x01) == 0x01;
        let res = if c_res { 0x80 | (v >> 1) } else { v >> 1 };
        self.set_flag(Z, res == 0 );
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, c_res);
        res
    }

    /*
     * Rotate A right through carry flag
     *
     *  Z - Set if res = 0
     *  N - Reset
     *  H - Reset
     *  C - Contains old bit 0 data
     */
    fn alu_rr(&mut self, v: u8) -> u8 {
        let res = if self.flag_c() { 0x80 | (v >> 1) } else { v >> 1 };
        self.set_flag(Z, res == 0 );
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, (v & 0x01) == 0x01 );
        res
    }

    /*
     * Shift n left into Carry. LSB = 0
     *  n = A,B,C,D,E,H,L,(HL) 
     *
     *  Z - Set if res = 0
     *  N - Reset
     *  H - Reset
     *  C - Contains old bit 7 data
     */
    fn alu_sla(&mut self, v: u8) -> u8 {
        let c_res = (v & (1 << 7)) != 0;
        let res = v << 1;
        self.set_flag(Z, res == 0 );
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, c_res );
        res
    }

    /*
     * Shift n right into Carry. MSB unchanged
     *  n = A,B,C,D,E,H,L,(HL) 
     *
     *  Z - Set if res = 0
     *  N - Reset
     *  H - Reset
     *  C - Contains old bit 0 data
     */
    fn alu_sra(&mut self, v: u8) -> u8 {
        let c_res = (1 & v) != 0;
        let res = (v >> 1) | (v & (1 << 7));
        self.set_flag(Z, res == 0 );
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, c_res );
        res
    }

    /*
     * Shift n right into Carry. MSB = 0
     *  n = A,B,C,D,E,H,L,(HL) 
     *
     *  Z - Set if res = 0
     *  N - Reset
     *  H - Reset
     *  C - Contains old bit 0 data
     */
    fn alu_srl(&mut self, v: u8) -> u8 {
        let c_res = (1 & v) != 0;
        let res = (v >> 1) & !(1 << 7); //MSB may be zero from shift, doing it manually just cause 
        self.set_flag(Z, res == 0 );
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, c_res );
        res 
    }

    /*
     *  Test bit (b) in register (r)
     *
     *  Z - Set if b of r == 0
     *  N - Reset
     *  H - Set
     *  C - N/A
     */
    fn alu_bit(&mut self, b: u8, r: u8) {
        // NOTE: Handle the (HL) case in ex()
        self.set_flag(Z, ((1 << b) & r) == 0 );
        self.set_flag(N, false);
        self.set_flag(H, true);
    }

    /*
     *  Set bit (b) in register (r)
     */
    fn alu_set(&self, b: u8, r: u8) -> u8 {
        (1 << b) | r
    }

    /*
     *  Reset bit (b) in register (r)
     */
    fn alu_reset(&self, b: u8, r: u8) -> u8 {
        !(1 << b) & r
    }

    fn alu_jr(&mut self) {
        // TODO writeme
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
            0x00 => { },
            0x01 => {
                let v = self.imm_hw();
                self.reg.set_bc(v);
            },
            0x02 => {
                self.mem.set(self.reg.get_bc(), self.reg.a); 
            },
            0x03 => {
                let v = self.reg.get_bc().wrapping_add(1);
                self.reg.set_bc(v);
            },
            0x04 => {
                self.reg.b = self.alu_inc(self.reg.b);
            },
            0x05 => {
                self.reg.b = self.alu_dec(self.reg.b);
            },
            0x06 => {
                self.reg.b = self.imm();
            },
            0x07 => {
                self.reg.a = self.alu_rlc(self.reg.a);
                self.set_flag(Z, false);
            },
            0x08 => {
                let v = self.imm_hw();
                self.mem.set_hw(v, self.reg.sp);
            },
            0x09 => {
                self.alu_add_hw_hl(self.reg.get_bc());
            },
            0x0A => {
                self.reg.a = self.mem.get(self.reg.get_bc());
            },
            0x0B => {
                let v = self.reg.get_bc().wrapping_sub(1);
                self.reg.set_bc(v);
            },
            0x0C => {
                self.reg.c = self.alu_inc(self.reg.c);
            },
            0x0D => {
                self.reg.c = self.alu_dec(self.reg.c);
            },
            0x0E => {
                self.reg.c = self.imm();
            },
            0x0F => {
                self.reg.a = self.alu_rrc(self.reg.a);
                self.set_flag(Z, false);
            },
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
