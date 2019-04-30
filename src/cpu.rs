#![allow(dead_code)]

use super::mmu::MMUnit;
use super::register::Flag;
use super::register::Flag::{C, H, N, Z};
use super::register::Register;
use std::fmt;

#[derive(Default)]
pub struct CPU {
    reg: Register,
    pub mem: MMUnit, //TODO not sure it needs to be public.
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(PC: {}, SP: {}, A: {}, B: {}, C: {}, D: {}, E: {}, F: {}, H: {}, L: {}, AF: {}, BC: {}, DE: {}, HL: {})",
            self.reg.pc,
            self.reg.sp,
            self.reg.a,
            self.reg.b,
            self.reg.c,
            self.reg.d,
            self.reg.e,
            self.reg.f,
            self.reg.h,
            self.reg.l,
            self.reg.get_af(),
            self.reg.get_bc(),
            self.reg.get_de(),
            self.reg.get_hl()
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

    /*
     * Add register to A
     *
     * Z - Set if result = 0
     * N - Reset
     * H - Set if lower nibble of A + reg. overflows bit 3
     * C - Set if A + reg. overflows bit 7
     */
    fn alu_add(&mut self, v: u8) {
        let a = self.reg.a;
        let res = a.wrapping_add(v);
        self.set_flag(Z, res == 0);
        self.set_flag(N, false);
        self.set_flag(H, ((a & 0xF) + (v & 0xF)) > 0xF);
        self.set_flag(C, (u16::from(a) + u16::from(v)) > 0xFF);
        self.reg.a = res;
    }

    /*
     * Add register to A through carry
     *
     * Z - Set if result = 0
     * N - Reset
     * H - Set if lower nibble of A + reg. + carry overflows bit 3
     * C - Set if A + reg. + carry overflows bit 7
     */
    fn alu_adc(&mut self, v: u8) {
        let a = self.reg.a;
        let c = u8::from(self.flag_c());
        let res = a.wrapping_add(v).wrapping_add(c);
        self.set_flag(Z, res == 0);
        self.set_flag(N, false);
        self.set_flag(H, ((a & 0xF) + (v & 0xF) + (c & 0xF)) > 0xF);
        self.set_flag(C, (u16::from(a) + u16::from(v) + u16::from(c)) > 0xFF);
        self.reg.a = res;
    }

    /*
     * Sub register from A
     *
     * Z - Set if result = 0
     * N - Set
     * H - Set if lower nibble of A < lower nibble of register
     * C - Set if reg A < register
     */
    fn alu_sub(&mut self, v: u8) {
        let a = self.reg.a;
        let res = a.wrapping_sub(v);
        self.set_flag(Z, res == 0);
        self.set_flag(N, true);
        self.set_flag(H, (a & 0x0F) < (v & 0x0F));
        self.set_flag(C, u16::from(a) < u16::from(v));
        self.reg.a = res;
    }

    /*
     * Sub register from A through carry
     *
     * Z - Set if result = 0
     * N - Set
     * H - Set if lower nibble of A < lower nibble of register + carry
     * C - Set if reg A < register + carry
     */
    fn alu_sbc(&mut self, v: u8) {
        let c = u8::from(self.flag_c());
        let a = self.reg.a;
        let res = a.wrapping_sub(v).wrapping_sub(c);
        self.set_flag(Z, res == 0);
        self.set_flag(N, true);
        self.set_flag(H, (a & 0x0F) < ((v & 0x0F) + c));
        self.set_flag(C, u16::from(a) < (u16::from(v) + u16::from(c)));
        self.reg.a = res;
    }

    /*
     * AND register with A
     *
     * Z - Set if result = 0
     * N - Reset
     * H - Set
     * C - Reset
     */
    fn alu_and(&mut self, v: u8) {
        let res = self.reg.a & v;
        self.set_flag(Z, res == 0);
        self.set_flag(N, false);
        self.set_flag(H, true);
        self.set_flag(C, false);
        self.reg.a = res;
    }

    /*
     * OR register with A
     *
     * Z - Set if result = 0
     * N - Reset
     * H - Reset
     * C - Reset
     */
    fn alu_or(&mut self, v: u8) {
        let res = self.reg.a | v;
        self.set_flag(Z, res == 0);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, false);
        self.reg.a = res;
    }

    /*
     * XOR register with A
     *
     * Z - Set if result = 0
     * N - Reset
     * H - Reset
     * C - Reset
     */
    fn alu_xor(&mut self, v: u8) {
        let res = self.reg.a | v;
        self.set_flag(Z, res == 0);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, false);
        self.reg.a = res;
    }

    /*
     * Compare register to A
     *  Subtract reg from A, then set A
     *  back to original value
     */
    fn alu_cp(&mut self, v: u8) {
        let res = self.reg.a;
        self.alu_sub(v);
        self.reg.a = res;
    }

    /*
     * Decrement value
     *
     * Z - Set if result = 0
     * H - Set if lower nibble = 0
     * N - Set
     * C - N/A
     */
    fn alu_dec(&mut self, v: u8) -> u8 {
        let res = v.wrapping_add(1);
        self.set_flag(Z, res == 0);
        self.set_flag(H, (res & 0xF) == 0);
        self.set_flag(N, true);
        res
    }

    /*
     * Increment value
     *
     * Z - Set if result = 0
     * H - Set if bit 3 overflow
     * N - Reset
     * C - N/A
     */
    fn alu_inc(&mut self, v: u8) -> u8 {
        let res = v.wrapping_add(1);
        self.set_flag(Z, res == 0);
        self.set_flag(H, ((res & 0xF) + 1) > 0xF);
        self.set_flag(N, false);
        res
    }

    /*
     * Add half-word to HL register
     *
     * Z - N/A
     * N - Reset
     * H - Set if bit 11 overflow
     * C - Set if bit 15 overflow
     */
    fn alu_add_hw_hl(&mut self, v: u16) {
        let a = self.reg.get_hl();
        let res = a.wrapping_add(v);
        self.set_flag(N, false);
        self.set_flag(H, ((a & 0x07FF) + (v & 0x07FF)) > 0x07FF);
        self.set_flag(C, a > (0xFFFF - v));
        self.reg.set_hl(res);
    }

    /*
     *  Add a half-word (2 bytes) to the immediate
     *  value.
     *
     *  Z - Reset
     *  N - Reset
     *  H - Set if bit 3 overflow
     *  C - Set if bit 7 overflow
     */
    fn alu_add_hw_imm(&mut self, v: u16) -> u16 {
        let a = self.imm() as i8 as i16 as u16; // TODO This is signed, test this conversion.
        self.set_flag(Z, false);
        self.set_flag(N, false);
        self.set_flag(H, ((a & 0x000F) + (v & 0x000F)) > 0x000F);
        self.set_flag(C, ((a & 0x00FF) + (v & 0x00FF)) > 0x00FF);
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
        if self.flag_h() {
            adjust |= 0x06;
        }

        if !self.flag_n() {
            if (a & 0x0F) > 0x09 {
                adjust |= 0x06
            }
            if a > 0x99 {
                adjust |= 0x60
            }
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
        let c_res = (v & (1 << 7)) != 0;
        let res = v << 1;
        self.set_flag(Z, res == 0);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, c_res);
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
        self.set_flag(Z, res == 0);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, c_res);
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
        self.set_flag(Z, res == 0);
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
        let res = if self.flag_c() {
            0x80 | (v >> 1)
        } else {
            v >> 1
        };
        self.set_flag(Z, res == 0);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, (v & 0x01) == 0x01);
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
        self.set_flag(Z, res == 0);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, c_res);
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
        self.set_flag(Z, res == 0);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, c_res);
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
        self.set_flag(Z, res == 0);
        self.set_flag(N, false);
        self.set_flag(H, false);
        self.set_flag(C, c_res);
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
        self.set_flag(Z, ((1 << b) & r) == 0);
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
            0x00 => {}
            0x01 => {
                let v = self.imm_hw();
                self.reg.set_bc(v);
            }
            0x02 => self.mem.set(self.reg.get_bc(), self.reg.a),
            0x03 => {
                let v = self.reg.get_bc().wrapping_add(1);
                self.reg.set_bc(v);
            }
            0x04 => self.reg.b = self.alu_inc(self.reg.b),
            0x05 => self.reg.b = self.alu_dec(self.reg.b),
            0x06 => self.reg.b = self.imm(),
            0x07 => {
                self.reg.a = self.alu_rlc(self.reg.a);
                self.set_flag(Z, false);
            }
            0x08 => {
                let v = self.imm_hw();
                self.mem.set_hw(v, self.reg.sp);
            }
            0x09 => self.alu_add_hw_hl(self.reg.get_bc()),
            0x0A => self.reg.a = self.mem.get(self.reg.get_bc()),
            0x0B => {
                let v = self.reg.get_bc().wrapping_sub(1);
                self.reg.set_bc(v);
            }
            0x0C => self.reg.c = self.alu_inc(self.reg.c),
            0x0D => self.reg.c = self.alu_dec(self.reg.c),
            0x0E => self.reg.c = self.imm(),
            0x0F => {
                self.reg.a = self.alu_rrc(self.reg.a);
                self.set_flag(Z, false);
            }
            0x10 => {
                //TODO ...nothing? (STOP inst.)
            }
            0x11 => {
                let v = self.imm_hw();
                self.reg.set_de(v);
            }
            0x12 => self.mem.set(self.reg.get_de(), self.reg.a),
            0x13 => self.reg.set_de(self.reg.get_de().wrapping_add(1)),
            0x14 => self.reg.d = self.alu_inc(self.reg.d),
            0x15 => self.reg.d = self.alu_dec(self.reg.d),
            0x16 => self.reg.d = self.imm(),
            0x17 => {
                self.reg.a = self.alu_rl(self.reg.a);
                self.set_flag(Z, false);
            }
            0x18 => self.alu_jr(),
            0x19 => self.alu_add_hw_hl(self.reg.get_de()),
            0x1A => self.reg.a = self.mem.get(self.reg.get_de()),
            0x1B => self.reg.set_de(self.reg.get_de().wrapping_sub(1)),
            0x1C => self.reg.e = self.alu_inc(self.reg.e),
            0x1D => self.reg.e = self.alu_dec(self.reg.e),
            0x1E => self.reg.e = self.imm(),
            0x1F => {
                //TODO RRA
            }
            0x20 => {
                //TODO JR NC,r8
            }
            0x21 => {
                let v = self.imm_hw();
                self.reg.set_hl(v);
            }
            0x22 => {
                //TODO LD (HL+), A
            }
            0x23 => self.reg.set_hl(self.reg.get_hl().wrapping_add(1)),
            0x24 => self.reg.h = self.alu_inc(self.reg.h),
            0x25 => self.reg.h = self.alu_dec(self.reg.h),
            0x26 => self.reg.h = self.imm(),
            0x27 => self.alu_daa(),
            0x28 => {
                //TODO JR Z, r8
            }
            0x29 => self.alu_add_hw_hl(self.reg.get_hl()),
            0x2A => {
                //TODO LD A, (HL+)
            }
            0x2B => self.reg.set_hl(self.reg.get_hl().wrapping_sub(1)),
            0x2C => self.reg.l = self.alu_inc(self.reg.l),
            0x2D => self.reg.l = self.alu_dec(self.reg.l),
            0x2E => self.reg.l = self.imm(),
            0x2F => self.alu_cpl(),
            0x30 => {
                //TODO JR NC, r8
            }
            0x31 => {
                let v = u16::from(self.imm());
                self.reg.sp = v;
            }
            0x32 => {
                //TODO LD (HL-) , A
            }
            0x33 => self.reg.sp += 1,
            0x34 => {
                let v = self.reg.get_hl().wrapping_add(1);
                self.reg.set_hl(v);
            }
            0x35 => {
                let v = self.reg.get_hl().wrapping_sub(1);
                self.reg.set_hl(v);
            }
            0x36 => {
                let v = self.imm();
                self.reg.set_hl(u16::from(v));
            }
            0x37 => self.alu_scf(),
            0x38 => {
                //TODO JR C, r8
            }
            0x39 => self.alu_add_hw_hl(self.reg.sp),
            0x3A => {
                // TODO LD A, (HL-)
            }
            0x3B => {
                let v = self.reg.sp.wrapping_sub(1);
                self.reg.sp = v;
            }
            0x3C => self.reg.a = self.alu_inc(self.reg.a),
            0x3D => self.reg.a = self.alu_dec(self.reg.a),
            0x3E => {
                let v = self.imm();
                self.reg.a = v;
            }
            0x3F => self.alu_ccf(),
            0x40 => self.reg.b = self.reg.b,
            0x41 => self.reg.b = self.reg.c,
            0x42 => self.reg.b = self.reg.d,
            0x43 => self.reg.b = self.reg.e,
            0x44 => self.reg.b = self.reg.h,
            0x45 => self.reg.b = self.reg.l,
            0x46 => {
                let v = self.mem.get(self.reg.get_hl());
                self.reg.b = v;
            }
            0x47 => self.reg.b = self.reg.a,
            0x48 => self.reg.c = self.reg.b,
            0x49 => self.reg.c = self.reg.c,
            0x4A => self.reg.c = self.reg.d,
            0x4B => self.reg.c = self.reg.e,
            0x4C => self.reg.c = self.reg.h,
            0x4D => self.reg.c = self.reg.l,
            0x4E => {
                let v = self.mem.get(self.reg.get_hl());
                self.reg.c = v;
            }
            0x4F => self.reg.c = self.reg.a,
            0x50 => self.reg.d = self.reg.b,
            0x51 => self.reg.d = self.reg.c,
            0x52 => self.reg.d = self.reg.d,
            0x53 => self.reg.d = self.reg.e,
            0x54 => self.reg.d = self.reg.h,
            0x55 => self.reg.d = self.reg.l,
            0x56 => {
                let v = self.mem.get(self.reg.get_hl());
                self.reg.d = v;
            }
            0x57 => self.reg.d = self.reg.a,
            0x58 => self.reg.e = self.reg.b,
            0x59 => self.reg.e = self.reg.c,
            0x5A => self.reg.e = self.reg.d,
            0x5B => self.reg.e = self.reg.e,
            0x5C => self.reg.e = self.reg.h,
            0x5D => self.reg.e = self.reg.l,
            0x5E => {
                let v = self.mem.get(self.reg.get_hl());
                self.reg.e = v;
            }
            0x5F => self.reg.e = self.reg.a,
            0x60 => self.reg.h = self.reg.b,
            0x61 => self.reg.h = self.reg.c,
            0x62 => self.reg.h = self.reg.d,
            0x63 => self.reg.h = self.reg.e,
            0x64 => self.reg.h = self.reg.h,
            0x65 => self.reg.h = self.reg.l,
            0x66 => {
                let v = self.mem.get(self.reg.get_hl());
                self.reg.h = v;
            }
            0x67 => self.reg.h = self.reg.a,
            0x68 => self.reg.l = self.reg.b,
            0x69 => self.reg.l = self.reg.c,
            0x6A => self.reg.l = self.reg.d,
            0x6B => self.reg.l = self.reg.e,
            0x6C => self.reg.l = self.reg.h,
            0x6D => self.reg.l = self.reg.l,
            0x6E => {
                let v = self.mem.get(self.reg.get_hl());
                self.reg.l = v;
            }
            0x6F => self.reg.l = self.reg.a,
            0x70 => self.mem.set(self.reg.get_hl(), self.reg.b),
            0x71 => self.mem.set(self.reg.get_hl(), self.reg.c),
            0x72 => self.mem.set(self.reg.get_hl(), self.reg.d),
            0x73 => self.mem.set(self.reg.get_hl(), self.reg.e),
            0x74 => self.mem.set(self.reg.get_hl(), self.reg.h),
            0x75 => self.mem.set(self.reg.get_hl(), self.reg.l),
            0x76 => {
                // TODO: HALT
            }
            0x77 => self.mem.set(self.reg.get_hl(), self.reg.a),
            0x78 => self.reg.a = self.reg.b,
            0x79 => self.reg.a = self.reg.c,
            0x7A => self.reg.a = self.reg.d,
            0x7B => self.reg.a = self.reg.e,
            0x7C => self.reg.a = self.reg.h,
            0x7D => self.reg.a = self.reg.l,
            0x7E => {
                let v = self.mem.get(self.reg.get_hl());
                self.reg.a = v;
            }
            0x7F => self.reg.a = self.reg.a,
            0x80 => self.alu_add(self.reg.b),
            0x81 => self.alu_add(self.reg.c),
            0x82 => self.alu_add(self.reg.d),
            0x83 => self.alu_add(self.reg.e),
            0x84 => self.alu_add(self.reg.h),
            0x85 => self.alu_add(self.reg.l),
            0x86 => {
                let v = self.mem.get(self.reg.get_hl());
                self.alu_add(v);
            }
            0x87 => self.alu_add(self.reg.a),
            0x88 => self.alu_adc(self.reg.b),
            0x89 => self.alu_adc(self.reg.c),
            0x8A => self.alu_adc(self.reg.d),
            0x8B => self.alu_adc(self.reg.e),
            0x8C => self.alu_adc(self.reg.h),
            0x8D => self.alu_adc(self.reg.l),
            0x8E => {
                let v = self.mem.get(self.reg.get_hl());
                self.alu_adc(v);
            }
            0x8F => self.alu_adc(self.reg.a),
            0x90 => self.alu_sub(self.reg.b),
            0x91 => self.alu_sub(self.reg.c),
            0x92 => self.alu_sub(self.reg.d),
            0x93 => self.alu_sub(self.reg.e),
            0x94 => self.alu_sub(self.reg.h),
            0x95 => self.alu_sub(self.reg.l),
            0x96 => {
                let v = self.mem.get(self.reg.get_hl());
                self.alu_sub(v);
            }
            0x97 => self.alu_sub(self.reg.a),
            0x98 => self.alu_sbc(self.reg.b),
            0x99 => self.alu_sbc(self.reg.c),
            0x9A => self.alu_sbc(self.reg.d),
            0x9B => self.alu_sbc(self.reg.e),
            0x9C => self.alu_sbc(self.reg.h),
            0x9D => self.alu_sbc(self.reg.l),
            0x9E => {
                let v = self.mem.get(self.reg.get_hl());
                self.alu_sbc(v);
            }
            0x9F => self.alu_sbc(self.reg.a),
            0xA0 => self.alu_and(self.reg.b),
            0xA1 => self.alu_and(self.reg.c),
            0xA2 => self.alu_and(self.reg.d),
            0xA3 => self.alu_and(self.reg.e),
            0xA4 => self.alu_and(self.reg.h),
            0xA5 => self.alu_and(self.reg.l),
            0xA6 => {
                let v = self.mem.get(self.reg.get_hl());
                self.alu_and(v);
            }
            0xA7 => self.alu_and(self.reg.a),
            0xA8 => self.alu_xor(self.reg.b),
            0xA9 => self.alu_xor(self.reg.c),
            0xAA => self.alu_xor(self.reg.d),
            0xAB => self.alu_xor(self.reg.e),
            0xAC => self.alu_xor(self.reg.h),
            0xAD => self.alu_xor(self.reg.l),
            0xAE => {
                let v = self.mem.get(self.reg.get_hl());
                self.alu_xor(v);
            }
            0xAF => self.alu_xor(self.reg.a),
            0xB0 => self.alu_or(self.reg.b),
            0xB1 => self.alu_or(self.reg.c),
            0xB2 => self.alu_or(self.reg.d),
            0xB3 => self.alu_or(self.reg.e),
            0xB4 => self.alu_or(self.reg.h),
            0xB5 => self.alu_or(self.reg.l),
            0xB6 => {
                let v = self.mem.get(self.reg.get_hl());
                self.alu_xor(v);
            }
            0xB7 => self.alu_or(self.reg.a),
            0xB8 => self.alu_cp(self.reg.b),
            0xB9 => self.alu_cp(self.reg.c),
            0xBA => self.alu_cp(self.reg.d),
            0xBB => self.alu_cp(self.reg.e),
            0xBC => self.alu_cp(self.reg.h),
            0xBD => self.alu_cp(self.reg.l),
            0xBE => {
                let v = self.mem.get(self.reg.get_hl());
                self.alu_or(v);
            }
            0xBF => self.alu_cp(self.reg.a),
            0xC0 => {
                //TODO RET NZ
            }
            0xC1 => {
                let v = self.stack_pop();
                self.reg.set_bc(v);
            }
            0xC2 => {
                //TODO JP NZ, a16
            }
            0xC3 => {
                //TODO JP a16
            }
            0xC4 => {
                //TODO CALL NZ, a16
            }
            0xC5 => self.stack_push(self.reg.get_bc()),
            0xC6 => {
                let v = self.imm();
                self.alu_add(v);
            }
            0xC7 => {
                //TODO RST 00H
            }
            0xC8 => {
                //TODO RET Z
            }
            0xC9 => {
                //TODO RET
            }
            0xCA => {
                //TODO JP Z, a16
            }
            0xCB => {
                //TODO PREFIX CB
            }
            0xCC => {
                //TODO CALL Z, a16
            }
            0xCD => {
                //TODO CALL a16
            }
            0xCE => {
                let v = self.imm();
                self.alu_adc(v);
            }
            0xCF => {
                //TODO RST 08H
            }
            0xD0 => {
                //TODO RET NC
            }
            0xD1 => {
                let v = self.stack_pop();
                self.reg.set_de(v);
            }
            0xD2 => {
                //TODO JP NC, a16
            }
            0xD3 => {} // Not Impl.
            0xD4 => {
                //TODO CALL NC, a16
            }
            0xD5 => self.stack_push(self.reg.get_de()),
            0xD6 => {
                let v = self.imm();
                self.alu_sub(v);
            }
            0xD7 => {
                //TODO RST 10H
            }
            0xD8 => {
                //TODO RET C
            }
            0xD9 => {
                //TODO RETI
            }
            0xDA => {
                //TODO JP C, a16
            }
            0xDB => {} // Not Impl.
            0xDC => {
                //TODO CALL C, a16
            }
            0xDD => {} // Not Impl.
            0xDE => {
                let v = self.imm();
                self.alu_sbc(v);
            }
            0xDF => {
                //TODO RST 18H
            }
            0xE0 => {
                //TODO LDH (a8), A
            }
            0xE1 => {
                let v = self.stack_pop();
                self.reg.set_hl(v);
            }
            0xE2 => {
                //TODO LD (C), A
            }
            0xE3 => {} // Not Impl.
            0xE4 => {} // Not Impl.
            0xE5 => self.stack_push(self.reg.get_hl()),
            0xE6 => {
                let v = self.imm();
                self.alu_and(v);
            }
            0xE7 => {
                // TODO RST 20H
            }
            0xE8 => {
                //TODO ADD SP, r8
            }
            0xE9 => {
                //TODO JP (HL)
            }
            0xEA => {
                //TODO LD (a16) A
            }
            0xEB => {} // Not Impl.
            0xEC => {} // Not Impl.
            0xED => {} // Not Impl.
            0xEE => {
                let v = self.imm();
                self.alu_xor(v);
            }
            0xEF => {
                //TODO RST 28H
            }
            0xF0 => {
                //TODO LDH A, (a8)
            }
            0xF1 => {
                let v = self.stack_pop();
                self.reg.set_af(v);
            }
            0xF2 => {
                let v = self.mem.get(u16::from(self.reg.c));
                self.reg.a = v;
            }
            0xF3 => {
                //TODO DI
            }
            0xF4 => {} // Not Impl.
            0xF5 => self.stack_push(self.reg.get_af()),
            0xF6 => {
                let v = self.imm();
                self.alu_or(v);
            }
            0xF7 => {
                //TODO RST 30H
            }
            0xF8 => {
                //TODO LD HL, SP+r8
                let v = self.reg.sp + u16::from(self.imm());
                self.reg.set_hl(v);
            }
            0xF9 => {
                self.reg.sp = self.reg.get_hl();
            }
            0xFA => {
                //TODO check this.
                let addr = self.imm_hw();
                let v = self.mem.get(addr);
                self.reg.a = v;
            }
            0xFB => {
                //TODO EI
            }
            0xFC => {} // Not Impl.
            0xFD => {} // Not Impl.
            0xFE => {
                let v = self.imm();
                self.alu_cp(v);
            }
            0xFF => {
                //TODO RST 38H
            }
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
