pub struct Rom {
    vec: Box<Vec<u8>>,
}

impl Rom {
    pub fn new(buf: Box<Vec<u8>>) -> Rom {
        Rom { vec: buf }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.vec[addr as usize]
    }

    pub fn size(&self) -> usize {
        self.vec.len()
    }
}
