#![allow(dead_code)]


mod cpu {
    use std::fmt; 

    pub struct CPU {
        reg: Registers,
        flag: u8,
    }

    #[allow(non_snake_case)]
    #[derive(Copy, Clone)]
    struct Registers {
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
        AF: u16,    /* Couldn't find a good way to overlay these with the 8 bit reg */
        BC: u16,
        DE: u16,
        HL: u16,
    }

    impl fmt::Display for CPU {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "(PC: {}, SP: {}, A: {}, B: {}, C: {}, D: {}, E: {}, F: {}, H: {}, L: {}, AF: {}, BC: {}, DE: {}, HL: {}, flag: {})", 
                self.reg.PC, self.reg.SP, self.reg.A, self.reg.B, self.reg.C, self.reg.D,
                self.reg.E, self.reg.F, self.reg.H, self.reg.L, self.reg.AF, self.reg.BC, 
                self.reg.DE, self.reg.HL, self.flag
                )
        }
    }

    impl Default for CPU {
        fn default() -> CPU {
            CPU {
                reg: Default::default(),
                flag: 0,
            }
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
                AF: 0,
                BC: 0,
                DE: 0,
                HL: 0,
            }
        }
    }

    impl CPU {

        pub fn step(&mut self) {
            self.reg.PC += 1;
        }
        
        pub fn load(&mut self,  reg: &str, val: u16) {
            match reg {
                "A" => {
                   self.reg.A = (val & 0xF) as u8;
                },
                "B" => {
                   self.reg.B = (val & 0xF) as u8;
                },
                "C" => {
                   self.reg.C = (val & 0xF) as u8;
                },
                "D" => {
                   self.reg.D = (val & 0xF) as u8;
                },
                "E" => {
                   self.reg.E = (val & 0xF) as u8;
                },
                "F" => {
                   self.reg.F = (val & 0xF) as u8;
                },
                "H" => {
                   self.reg.H = (val & 0xF) as u8;
                },
                "L" => {
                   self.reg.L = (val & 0xF) as u8;
                },
                "AF" => {
                   self.reg.AF = val;
                },
                "BC" => {
                   self.reg.BC = val;
                },
                "DE" => {
                   self.reg.DE = val;
                },
                "HL" => {
                   self.reg.HL = val;
                },
                _ => println!("Couldn't match a register!"),
            }
        }

        pub fn toggle_flag(&mut self, flag: &str) {
            match flag {
                "Z" => {
                    self.flag ^= 0b1000000 
                },
                "N" => {
                    self.flag ^= 0b0100000 
                },
                "H" => {
                    self.flag ^= 0b0010000 
                },
                "C" => {
                    self.flag ^= 0b0001000 
                },
                _ => println!("Couldn't match a flag!"),
            }
        }

        pub fn flag_z(&self) -> bool {
            // is Zero Flag set?
            (self.flag & 0b1000000) != 0
        }
        pub fn flag_n(&self) -> bool {
            // is Subtract Flag set?
            (self.flag & 0b0100000) != 0
        }
        pub fn flag_h(&self) -> bool {
            // is Half Carry Flag set?
            (self.flag & 0b0010000) != 0
        }
        pub fn flag_c(&self) -> bool {
            // is Carry Flag set?
            (self.flag & 0b0001000) != 0
        }
    }
    #[cfg(test)]
    mod tests {
        use super::CPU;
        #[test]
        fn test_step() {
            let mut a: CPU = Default::default();
            a.step();
            assert_eq!(a.reg.PC, 0x100 + 1);
        }

        #[test]
        fn test_copy() {
            let mut a: CPU = Default::default(); 
            let mut b: CPU = Default::default(); 
            a.step();
            b.reg = a.reg;
            assert_eq!(a.reg.PC, 0x100 + 1);
            assert_eq!(a.reg.PC, b.reg.PC);
        }

        #[test]
        fn test_load_a() {
            let mut a: CPU = Default::default();
            a.load("A", 4);
            assert_eq!(a.reg.A, 4);
            a.load("A", 3);
            assert_eq!(a.reg.A, 3);
            a.load("A", 2);
            assert_eq!(a.reg.A, 2);
            a.load("A", 1);
            assert_eq!(a.reg.A, 1);
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

}

fn main() {
    let mut a: cpu::CPU = Default::default();
    a.load("AF", 1);
    println!("{}", a);
}
