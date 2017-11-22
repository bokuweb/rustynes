use std::cell::RefCell;

#[derive(Debug)]
pub struct Ram {
    pub field: RefCell<Vec<u8>>
}

impl Ram {
    pub fn new(buf: Vec<u8>) -> Ram {
        Ram{
            field: RefCell::new(buf)
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.field.borrow()[addr as usize]
    }

    pub fn write(&self, addr: u16, data: u8) {
        self.field.borrow_mut()[addr as usize] = data;
    }
}
