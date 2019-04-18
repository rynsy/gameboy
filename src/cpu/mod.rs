#![allow(dead_code)]

pub mod instructions;

pub mod cpu {
    use std::fmt; 

    pub struct CPU {
        reg: Registers,
        flag: u8,
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
        AF: u16,    
        BC: u16,
        DE: u16,
        HL: u16,
    }

    impl fmt::Display for CPU {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "CPU \n{{\n\t PC: {:#018b},\n\t SP: {:#018b},\n\t A: {:#010b},\n\t B: {:#010b},\n\t C: {:#010b},\n\t D: {:#010b},\n\t E: {:#010b},\n\t F: {:#010b},\n\t H: {:#010b},\n\t L: {:#010b},\n\t AF: {:#018b},\n\t BC: {:#018b},\n\t DE: {:#018b},\n\t HL: {:#018b},\n\t flags (F): {:#010b}\n}}", 
                self.reg.PC, 
                self.reg.SP, 
                ((self.reg.AF & 0xF0) >> 8) as u8, 
                ((self.reg.BC & 0xF0) >> 8) as u8, 
                (self.reg.BC & 0x0F) as u8, 
                ((self.reg.DE & 0xF0) >> 8) as u8, 
                (self.reg.DE & 0x0F) as u8, 
                (self.reg.AF & 0x0F) as u8, 
                ((self.reg.HL & 0xF0) >> 8) as u8, 
                (self.reg.HL & 0x0F) as u8,
                self.reg.AF, 
                self.reg.BC, 
                self.reg.DE, 
                self.reg.HL, 
                (self.reg.AF & 0x0F) as u8,
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
       
        /*
         *  Assumes value is in first 8 bits when loading 8-bit register.
         */
        pub fn load(&mut self,  reg: &str, val: u16) {
            match reg {
                "A" => {
                    self.reg.AF &= 0x0F; //Clear high-bits
                    self.reg.AF ^= val << 8;  //Set just the high-bits
                },
                "F" => {
                    self.reg.AF &= 0xF0; //Clear low-bits
                    self.reg.AF ^= val & 0x0F;  //Set just the low-bits
                },
                "B" => {
                    self.reg.BC &= 0x0F;
                    self.reg.BC ^= val << 8;
                },
                "C" => {
                    self.reg.BC &= 0xF0;
                    self.reg.BC ^= val & 0x0F;
                },
                "D" => {
                    self.reg.DE &= 0x0F;
                    self.reg.DE ^= val << 8;
                },
                "E" => {
                    self.reg.DE &= 0xF0;
                    self.reg.DE ^= val & 0x0F;
                },
                "H" => {
                    self.reg.HL &= 0x0F;
                    self.reg.HL ^= val << 8; 
                },
                "L" => {
                    self.reg.HL &= 0xF0;
                    self.reg.HL ^= val & 0x0F;
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
                    self.reg.AF ^= 1 << 7
                },
                "N" => {
                    self.reg.AF ^= 1 << 6
                },
                "H" => {
                    self.reg.AF ^= 1 << 5
                },
                "C" => {
                    self.reg.AF ^= 1 << 4
                },
                _ => println!("Couldn't match a flag!"),
            }
        }

        pub fn reg_a(&self) -> u8 
        {
            (self.reg.AF >> 8) as u8
        }
        pub fn reg_b(&self) -> u8 
        {
            (self.reg.BC >> 8) as u8
        }
        pub fn reg_c(&self) -> u8 
        {
            (self.reg.BC & 0x0F) as u8
        }
        pub fn reg_d(&self) -> u8 
        {
            (self.reg.DE >> 8) as u8
        }
        pub fn reg_e(&self) -> u8 
        {
            (self.reg.DE & 0x0F) as u8
        }
        pub fn reg_f(&self) -> u8 
        {
            (self.reg.AF & 0x0F) as u8
        }
        pub fn reg_h(&self) -> u8 
        {
            (self.reg.HL >> 8) as u8
        }
        pub fn reg_l(&self) -> u8 
        {
            (self.reg.HL & 0x0F) as u8
        }
        pub fn reg_af(&self) -> u16 
        {
            self.reg.AF
        }
        pub fn reg_bc(&self) -> u16 
        {
            self.reg.BC
        }
        pub fn reg_de(&self) -> u16 
        {
            self.reg.DE
        }
        pub fn reg_hl(&self) -> u16 
        {
            self.reg.HL
        }
        pub fn flag_z(&self) -> bool 
        {
            // is Zero Flag set?
            (self.reg.AF & 1 << 7) != 0
        }
        pub fn flag_n(&self) -> bool 
        {
            // is Subtract Flag set?
            (self.reg.AF & 1 << 6) != 0
        }
        pub fn flag_h(&self) -> bool 
        {
            // is Half Carry Flag set?
            (self.reg.AF & 1 << 5) != 0
        }
        pub fn flag_c(&self) -> bool 
        {
            // is Carry Flag set?
            (self.reg.AF & 1 << 4) != 0
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
            assert_eq!(a.reg.AF >> 8 as u8, 4);
            assert_eq!(a.reg_a(), 4);
            a.load("A", 3);
            assert_eq!(a.reg.AF >> 8 as u8, 3);
            assert_eq!(a.reg_a(), 3);
            a.load("A", 2);
            assert_eq!(a.reg.AF >> 8 as u8, 2);
            assert_eq!(a.reg_a(), 2);
            a.load("A", 1);
            assert_eq!(a.reg.AF >> 8 as u8, 1);
            assert_eq!(a.reg_a(), 1);
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
