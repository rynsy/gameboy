#![allow(dead_code)]

use std::fmt; 

#[derive(Copy, Clone)]
#[allow(non_snake_case)]
struct CPU {
    /*
     *      Paired like this in 2-byte words:
     *          AF
     *          BC
     *          DE
     *          HL
     *          SP
     *          PC
     *      Flag:
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
    AF: u16,    /* Couldn't find a good way to overlay these with the 8 bit regs */
    BC: u16,
    DE: u16,
    HL: u16,
    FLAG: u8,
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(PC: {}, SP: {}, A: {}, B: {}, C: {}, D: {}, E: {}, F: {}, H: {}, L: {}, AF: {}, BC: {}, DE: {}, HL: {}, FLAG: {})", 
            self.PC, self.SP, self.A, self.B, self.C, self.D,
            self.E, self.F, self.H, self.L, self.AF, self.BC, 
            self.DE, self.HL, self.FLAG
            )
    }
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
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
            AF: 0,
            BC: 0,
            DE: 0,
            HL: 0,
            FLAG: 0,
        }
    }
}

impl CPU {
    fn step(&mut self) {
        self.PC += 1;
    }
    
    fn load(&mut self, reg: &str, val: u16) {
        match reg {
            "A" => {
               self.A = (val & 0xF) as u8;
            },
            "B" => {
               self.B = (val & 0xF) as u8;
            },
            "C" => {
               self.C = (val & 0xF) as u8;
            },
            "D" => {
               self.D = (val & 0xF) as u8;
            },
            "E" => {
               self.E = (val & 0xF) as u8;
            },
            "F" => {
               self.F = (val & 0xF) as u8;
            },
            "H" => {
               self.H = (val & 0xF) as u8;
            },
            "L" => {
               self.L = (val & 0xF) as u8;
            },
            "AF" => {
               self.AF = val;
            },
            "BC" => {
               self.BC = val;
            },
            "DE" => {
               self.DE = val;
            },
            "HL" => {
               self.HL = val;
            },
            _ => println!("Couldn't match a register!"),
        }
    }

    fn toggle_flag(&mut self, flag: &str) {
        match flag {
            "Z" => {
                self.FLAG ^= 0b1000000 
            },
            "N" => {
                self.FLAG ^= 0b0100000 
            },
            "H" => {
                self.FLAG ^= 0b0010000 
            },
            "C" => {
                self.FLAG ^= 0b0001000 
            },
            _ => println!("Couldn't match a flag!"),
        }
    }

    fn flag_z(&self) -> bool {
        // is Zero Flag set?
        (self.FLAG & 0b1000000) != 0
    }
    fn flag_n(&self) -> bool {
        // is Subtract Flag set?
        (self.FLAG & 0b0100000) != 0
    }
    fn flag_h(&self) -> bool {
        // is Half Carry Flag set?
        (self.FLAG & 0b0010000) != 0
    }
    fn flag_c(&self) -> bool {
        // is Carry Flag set?
        (self.FLAG & 0b0001000) != 0
    }
}

fn main() {
    let mut a: CPU = Default::default();
    a.load("AF", 1);
    println!("{}", a);
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_step() {
        let mut a: CPU = Default::default();
        a.step();
        assert_eq!(a.PC, 0x100 + 1);
    }

    #[test]
    fn test_copy() {
        let mut a: CPU = Default::default(); 
        a.step();
        let b = a;
        assert_eq!(a.PC, 0x100 + 1);
        assert_eq!(a.PC, b.PC);
    }

    #[test]
    fn test_load_a() {
        let mut a: CPU = Default::default();
        a.load("A", 4);
        assert_eq!(a.A, 4);
        a.load("A", 3);
        assert_eq!(a.A, 3);
        a.load("A", 2);
        assert_eq!(a.A, 2);
        a.load("A", 1);
        assert_eq!(a.A, 1);
    }

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
