use std::fs;
use std::fmt;

pub struct MMUnit {
    data: Vec<u8>,
    rom_info: ROM,
}

impl Default for MMUnit {
    fn default() -> MMUnit {
        let vec: Vec<u8> = vec![0; 0xFFFF];
        MMUnit {
            data: vec,
            rom_info: ROM::default(),
        }
    }
}

struct ROM {
    filename: String,
    title: String,
}

impl Default for ROM {
    fn default() -> ROM {
        ROM {
            filename: "test_file".to_string(),
            title: "test".to_string(),
        }
    }
}

impl fmt::Display for ROM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ROM:\n\tFilename: {}\n\tTitle: {}",
            self.filename,
            self.title
        )
    }
}

impl MMUnit {
    pub fn set(&mut self, addr: u16, val: u8) {
        // TODO: Value may be u8
        self.data[addr as usize] = val;
    }

    pub fn get(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn get_hw(&self, addr: u16) -> u16 {
        ((u16::from(self.data[(addr + 1) as usize]) << 8) |
             u16::from(self.data[(addr) as usize])) as u16
    }

    pub fn set_hw(&mut self, addr: u16, val: u16) {
        self.data[addr as usize] = (val & 0x0F) as u8;
        self.data[(addr + 1) as usize] = (val & 0xF0) as u8;
    }

    pub fn load_rom(&mut self) {
        //TODO pass filename/path
        let dir = String::from(
            "C:\\Code\\rust\\gameboy\\data\\cpu_instrs\\individual\\01-special.gb",
        );
        //let dir = String::from("/home/ryan/code/rust/gameboy/data/cpu_instrs/individual/01-special.gb");
        let rom_data: Vec<u8> = fs::read(&dir).expect("Unable to read file");
        let rom = ROM {
            filename: dir,
            title: "test".to_string(),
        };
        self.rom_info = rom;
        self.data[..rom_data.len()].clone_from_slice(&rom_data);
    }
}

#[cfg(test)]
mod tests {
    use super::MMUnit;

    #[test]
    fn test_write() {
        let mut a = MMUnit::default();
        a.set(0, 10);
    }

    #[test]
    fn test_read() {
        let val = 10;
        let mut a = MMUnit::default();
        a.set(0, val);
        let b = a.get(0);
        assert_eq!(val, b);
    }

    #[test]
    fn test_open_file() {
        let mut a = MMUnit::default();
        a.load_rom();
        assert_eq!(a.data.is_empty(), false);
    }
}
