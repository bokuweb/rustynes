#[derive(Debug)]
pub struct Mmc {
    mapper: u8,
    bank: u8,
}

impl Mmc {
    pub fn new(mapper: u8, bank: u8) -> Self {
        Mmc { bank, mapper }
    }

    pub fn set_bank(&mut self, bank: u8) {
        self.bank = bank;
    }

    pub fn get_bank(&self) -> u8 {
        self.bank
    }

    pub fn create_chram_addr(&self, addr: u16) -> u16 {
        // TODO: Support mapper3 for now, add other mappers later.
        addr + (self.bank as u16) * 0x2000
    }

    pub fn get_mapper(&self) -> u8 {
        self.mapper
    }
}
