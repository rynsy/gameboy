use std::fmt; 

struct CPU {
    pc: u8,
    sp: u8,
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"(PC: {}, SP: {})", self.pc, self.sp)
    }
}

fn main() {
    let mut a = CPU{pc:0,sp:0};
    a.pc |= 1;
    println!("CPU: {}",a);
}
