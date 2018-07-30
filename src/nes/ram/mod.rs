#[derive(Debug)]
pub struct Ram {
    pub field: Vec<u8>,
}

impl Ram {
    pub fn new(buf: Vec<u8>) -> Ram {
        Ram { field: buf }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.field[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.field[addr as usize] = data;
    }
}
