/*
 *  TODO: May be able to refactor this using RefCell to get interior mutability.
 *  Reading: https://ricardomartins.cc/2016/06/08/interior-mutability
 */
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct Register {
    /*
     *      Register are all 16 bit, CPU is 8-bit. Individual registers
     *      are referenced/modified with masking.
     *
     *      Flag (F Register):
     *          Z N H C 0 0 0 0
     */
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
}

pub enum Flag {
    Z,
    N,
    H,
    C,
}

impl Register {
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

impl Default for Register {
    fn default() -> Register {
        Register {
            pc: 0x100,
            sp: 0xFFFE, /* Should be the highest available address in memory. Decrements before putting something on the stack. */
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
