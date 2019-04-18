use std::fs;
use std::fmt; 

/*
 *  TODO: May be able to refactor this using RefCell to get interior mutability.
 *  Reading: https://ricardomartins.cc/2016/06/08/interior-mutability
 */

pub struct ROM {
    filename: String,
    title: String,
    data: Vec<u8>,
    data_ptr: u32,      //FIXME: doesn't need to be this big.
}

impl Default for ROM {
    fn default() -> ROM {
        ROM {
            filename: "test_file".to_string(),
            title: "test".to_string(),
            data: vec![],
            data_ptr: 0,
        }
    }
}

impl fmt::Display for ROM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"ROM:\n\tFilename: {}\n\tTitle: {}\n\thas_data: {}\n\tdata_ptr: {}", 
               self.filename, self.title, !self.data.is_empty(), self.data_ptr)
    }
}

impl ROM {
    pub fn load_rom(&mut self) { //TODO pass filename/path
        let dir = String::from("C:\\Code\\rust\\gameboy\\data\\cpu_instrs\\individual\\01-special.gb");
        let rom_data: Vec<u8> = fs::read(&dir).expect("Unable to read file"); 
        self.filename = dir;
        self.title = "test".to_string();
        self.data = rom_data;
    }
}
