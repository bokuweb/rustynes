use nes::rom::Rom;
use nes::ram::Ram;
use nes::ppu::Ppu;

pub struct CpuBus<'a> {
    program_rom: &'a Rom,
    character_memory: &'a Ram,
    work_ram: &'a mut Ram,
    ppu: &'a Ppu,
}

impl<'a> CpuBus<'a> {
    pub fn new(
        program_rom: &'a Rom,
        character_memory: &'a Ram,
        work_ram: &'a mut Ram,
        ppu: &'a Ppu,
    ) -> CpuBus<'a> {
        CpuBus {
            program_rom,
            character_memory,
            work_ram,
            ppu,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000...0x07FF => self.work_ram.read(addr),
            0x0800...0x1FFF => self.work_ram.read(addr - 0x0800),
            0x2000...0x3FFF => 0, // TODO: PPU
            0x4016 => 0, // TODO: keypad
            0x8000...0xBFFF => self.program_rom.read(addr - 0x8000),
            0xC000...0xFFFF if self.program_rom.size() <= 0x4000 => {
                self.program_rom.read(addr - 0xC000)
            }
            0xC000...0xFFFF => self.program_rom.read(addr - 0x8000),
            _ => panic!("There is an illegal address access."),
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.work_ram.write(addr, data);
    }
}


/*
  write(addr: Word, data: Byte) {
    if (addr < 0x0800) {
      // RAM
      this.ram.write(addr, data);
    } else if (addr < 0x2000) {
      // mirror
      this.ram.write(addr - 0x0800, data);
    } else if (addr < 0x2008) {
      // PPU
      this.ppu.write(addr - 0x2000, data);
    } else if (addr >= 0x4000 && addr < 0x4020) {
      if (addr === 0x4014) {
        this.dma.write(data);
      } else if (addr === 0x4016) {
        // TODO Add 2P
        this.keypad.write(data);
      } else {
        // APU
        this.apu.write(addr - 0x4000, data);
      }
    }
  }
}
*/