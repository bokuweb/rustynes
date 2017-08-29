use std::str;

const NES_HEADER_SIZE: usize = 0x0010;
const PROGRAM_ROM_SIZE: usize = 0x4000;
const CHARACTER_ROM_SIZE: usize = 0x2000;

pub struct NesROM {
    is_horizontal_mirror: bool,
    characterMemory: Vec<u8>,
    programROM: Vec<u8>,
}

pub fn parse(nes: &mut [u8]) -> u8 {
    println!("{:?}", nes);
    let name = nes[0..3].to_vec();
    let s = match str::from_utf8(&name) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    if s != "NES" {
        panic!("Invalid UTF-8 sequence: {}")
    };
    println!("result: {}", s);
    10
}

/*


export const parse = (nesBuffer: ArrayBuffer): NesROM => {
  const nes = new Uint8Array(nesBuffer);
  if ([].slice.call(nes, 0, 3).map(v => String.fromCharCode(v)).join('') !== 'NES') {
    throw new Error('This file is not NES format.');
  }
  const programROMPages = nes[4];
  log.info('prom pages =', programROMPages);
  const characterROMPages = nes[5];
  log.info('crom pages =', characterROMPages);
  const isHorizontalMirror = !(nes[6] & 0x01);
  const mapper = (((nes[6] & 0xF0) >> 4) | nes[7] & 0xF0);
  log.info('mapper', mapper);
  const characterROMStart = NES_HEADER_SIZE + programROMPages * PROGRAM_ROM_SIZE;
  const characterROMEnd = characterROMStart + characterROMPages * CHARACTER_ROM_SIZE;

  // console.log('prom pages = ', programROMPages);
  const nesROM: NesROM = {
    isHorizontalMirror,
    programROM: nes.slice(NES_HEADER_SIZE, characterROMStart - 1),
    characterROM: nes.slice(characterROMStart, characterROMEnd - 1),
  };
  return nesROM;
};
*/