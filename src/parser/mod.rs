use std::str;

const NES_HEADER_SIZE: usize = 0x0010;
const PROGRAM_ROM_SIZE: usize = 0x4000;
const CHARACTER_ROM_SIZE: usize = 0x2000;

pub struct NesROM {
    // is_horizontal_mirror: bool,
    pub character_memory: Vec<u8>,
    pub program_rom: Vec<u8>,
}

pub fn parse(nes: &mut [u8]) -> NesROM {
    // println!("{:?}", nes);
    let name = nes[0..3].to_vec();
    let ines = match str::from_utf8(&name) {
        Ok(v) => v,
        Err(e) => panic!("Invalid *.nes file"),
    };
    if ines != "NES" {
        panic!("Invalid *.nes file.")
    };
    let program_rom_pages = nes[4] as usize;
    println!("program rom size is {}", program_rom_pages);
    let charcter_rom_pages = nes[5] as usize;
    println!("character rom size is {}", charcter_rom_pages);
    // TODO: mirror flag, mapper number, etc.....
    // const isHorizontalMirror = !(nes[6] & 0x01);
    // const mapper = (((nes[6] & 0xF0) >> 4) | nes[7] & 0xF0)
    let character_rom_start = NES_HEADER_SIZE + program_rom_pages * PROGRAM_ROM_SIZE;
    let character_rom_end = character_rom_start + charcter_rom_pages * CHARACTER_ROM_SIZE;
    NesROM {
        program_rom: nes[NES_HEADER_SIZE..character_rom_start].to_vec(),
        character_memory: nes[character_rom_start..character_rom_end].to_vec(),
    }
}
