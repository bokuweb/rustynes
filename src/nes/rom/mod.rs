#[derive(Debug)]
pub struct Rom {
    vec: Vec<u8>,
}

impl Rom {
    pub fn new(buf: Vec<u8>) -> Rom {
        Rom { vec: buf.clone() }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.vec[addr as usize]
    }

    pub fn size(&self) -> usize {
        self.vec.len()
    }
}
