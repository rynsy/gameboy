pub struct MMap {
    data: Vec<u16>,
}

impl Default for MMap {
    fn default() -> MMap {
        let vec: Vec<u16> = vec![0; 0xFFFF];
        MMap{
            data: vec,
        }
    }
}

impl MMap {
    pub fn write(&mut self, addr: u16, val: u16 ) {   // TODO: Value may be u8
        self.data[addr as usize] = val;
    }

    pub fn read(&self, addr: u16 ) -> u16 { // TODO: May return u8
        self.data[addr as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::MMap;

    #[test]
    fn test_write() {
        let mut a: MMap = Default::default();
        a.write(0, 10);
    }

    #[test]
    fn test_read() {
        let val = 10;
        let mut a: MMap = Default::default();
        a.write(0, val);
        let b = a.read(0);
        assert_eq!(val, b);
    }
}
