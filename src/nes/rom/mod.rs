pub struct Rom(Vec<u8>);

impl Rom {
    pub fn new(buf: Vec<u8>) -> Rom {
        Rom(buf)
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }
}
