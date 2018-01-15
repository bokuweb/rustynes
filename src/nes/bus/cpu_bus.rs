use nes::rom::Rom;
use nes::ram::Ram;
use nes::ppu::Ppu;
use nes::keypad::Keypad;
use nes::dma::Dma;

pub struct Bus<'a> {
    program_rom: &'a Box<Rom>,
    work_ram: &'a Box<Ram>,
    ppu: &'a mut Ppu,
    keypad: &'a mut Keypad,
    dma: &'a mut Dma,
}

pub trait CpuBus {
    fn read_word(&mut self, addr: u16) -> u16;

    fn read(&mut self, addr: u16) -> u8;

    fn write(&mut self, addr: u16, data: u8);
}

impl<'a> Bus<'a> {
    pub fn new(program_rom: &'a Box<Rom>,
               work_ram: &'a Box<Ram>,
               ppu: &'a mut Ppu,
               keypad: &'a mut Keypad,
               dma: &'a mut Dma)
               -> Bus<'a> {
        Self {
            program_rom,
            work_ram,
            ppu,
            keypad,
            dma,
        }
    }
}

impl<'a> CpuBus for Bus<'a> {
    fn read_word(&mut self, addr: u16) -> u16 {
        let lower = self.read(addr) as u16;
        let upper = self.read(addr + 1) as u16;
        (upper << 8 | lower) as u16
    }

    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000...0x07FF => self.work_ram.read(addr),
            0x0800...0x1FFF => self.work_ram.read(addr - 0x0800),
            0x2000...0x3FFF => self.ppu.read(addr - 0x2000),
            0x4016 => self.keypad.read(),
            0x4000...0x401F => 0, // TODO: APU
            0x8000...0xBFFF => self.program_rom.read(addr - 0x8000),
            0xC000...0xFFFF if self.program_rom.size() <= 0x4000 => {
                self.program_rom.read(addr - 0xC000)
            }
            0xC000...0xFFFF => self.program_rom.read(addr - 0x8000),
            _ => panic!("There is an illegal address ({}) access.", addr),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000...0x07FF => self.work_ram.write(addr, data),
            0x0800...0x1FFF => self.work_ram.write(addr - 0x0800, data),
            0x2000...0x3FFF => self.ppu.write(addr - 0x2000, data),
            0x4014 => self.dma.write(data),
            0x4016 => self.keypad.write(data),
            0x4000...0x401F => {} // TODO: APU
            _ => panic!("There is an illegal address ({}) access.", addr),
        };
    }
}
