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
    FLAG: u8,
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"(PC: {}, SP: {}, A: {}, B: {}, C: {}, D: {}, E: {}, F: {}, H: {}, L: {}, FLAG: {})", 
               self.PC, self.SP, self.A, self.B, self.C, self.D,
               self.E, self.F, self.H, self.L, self.FLAG)
    }
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            PC: 0,
            SP: 0,
            A: 0,
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            F: 0,
            H: 0,
            L: 0,
            FLAG: 0,
        }
    }
}

impl CPU {
    fn step(&mut self) {
        self.PC += 1;
    }
}

fn main() {
    let mut a: CPU = Default::default(); 
    a.step();
    println!("CPU 1: {}",a);
    let y = a;
    println!("CPU 2: {}",y);
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_step() {
        let mut a: CPU = Default::default();
        a.step();
        assert_eq!(a.PC, 1);
    }

    #[test]
    fn test_copy() {
        let mut a: CPU = Default::default(); 
        a.step();
        let b = a;
        assert_eq!(a.PC, 1);
        assert_eq!(a.PC, b.PC);
    }
}
