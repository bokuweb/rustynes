pub struct Ram(Vec<u8>);

impl Ram {
    pub fn new(buf: Vec<u8>) -> Ram {
        Ram(buf)
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.0[addr as usize] = data;
    }
}
