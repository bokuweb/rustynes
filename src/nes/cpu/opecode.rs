use std::collections::HashMap;

#[derive(Debug)]
pub struct Opecode {
    pub name: Instraction,
    pub mode: Addressing,
    pub cycle: u8,
}

#[derive(Debug)]
pub enum Instraction {
    LDA,
    LDX,
}

#[derive(Debug)]
pub enum Addressing {
    Immediate,
    ZeroPage,
}

pub fn build_opecode_map() -> HashMap<String, u8> {

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let cycles: Vec<u8> = vec![
      7, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 4, 4, 6, 6,
      2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7,
      6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 4, 4, 6, 6,
      2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7,
      6, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 3, 4, 6, 6,
      2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7,
      6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 5, 4, 6, 6,
      2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7,
      2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4,
      2, 6, 2, 6, 4, 4, 4, 4, 2, 4, 2, 5, 5, 4, 5, 5,
      2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4,
      2, 5, 2, 5, 4, 4, 4, 4, 2, 4, 2, 4, 4, 4, 4, 4,
      2, 6, 2, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6,
      2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
      2, 6, 3, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6,
      2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
    ];
    let mut map = HashMap::new();
    map.insert(String::from("0xA9"), 10);
    //map.insert(0xA5, Opecode { name: "LDA", mode: "zeroPage", cycle: cycles[0xA5] });
    //map.insert(0xAD, Opecode { name: "LDA", mode: "absolute", cycle: cycles[0xAD] });
    //map.insert(0xB5, Opecode { name: "LDA", mode: "zeroPageX", cycle: cycles[0xB5] });
    //map.insert(0xBD, Opecode { name: "LDA", mode: "absoluteX", cycle: cycles[0xBD] });
    //map.insert(0xB9, Opecode { name: "LDA", mode: "absoluteY", cycle: cycles[0xB9] });
    //map.insert(0xA1, Opecode { name: "LDA", mode: "preIndexedIndirect", cycle: cycles[0xA1] });
    //map.insert(0xB1, Opecode { name: "LDA", mode: "postIndexedIndirect", cycle: cycles[0xB1] });
    //map.insert(0xA2, Opecode { name: "LDX", mode: "immediate", cycle: cycles[0xA2] });
    //map.insert(0xA6, Opecode { name: "LDX", mode: "zeroPage", cycle: cycles[0xA6] });
    //map.insert(0xAE, Opecode { name: "LDX", mode: "absolute", cycle: cycles[0xAE] });
    //map.insert(0xB6, Opecode { name: "LDX", mode: "zeroPageY", cycle: cycles[0xB6] });
    //map.insert(0xBE, Opecode { name: "LDX", mode: "absoluteY", cycle: cycles[0xBE] });
    //map.insert(0xA0, Opecode { name: "LDY", mode: "immediate", cycle: cycles[0xA0] });
    //map.insert(0xA4, Opecode { name: "LDY", mode: "zeroPage", cycle: cycles[0xA4] });
    //map.insert(0xAC, Opecode { name: "LDY", mode: "absolute", cycle: cycles[0xAC] });
    //map.insert(0xB4, Opecode { name: "LDY", mode: "zeroPageX", cycle: cycles[0xB4] });
    //map.insert(0xBC, Opecode { name: "LDY", mode: "absoluteX", cycle: cycles[0xBC] });
    //map.insert(0x85, Opecode { name: "STA", mode: "zeroPage", cycle: cycles[0x85] });
    //map.insert(0x8D, Opecode { name: "STA", mode: "absolute", cycle: cycles[0x8D] });
    //map.insert(0x95, Opecode { name: "STA", mode: "zeroPageX", cycle: cycles[0x95] });
    //map.insert(0x9D, Opecode { name: "STA", mode: "absoluteX", cycle: cycles[0x9D] });
    //map.insert(0x99, Opecode { name: "STA", mode: "absoluteY", cycle: cycles[0x99] });
    //map.insert(0x81, Opecode { name: "STA", mode: "preIndexedIndirect", cycle: cycles[0x81] });
    //map.insert(0x91, Opecode { name: "STA", mode: "postIndexedIndirect", cycle: cycles[0x91] });
    //map.insert(0x86, Opecode { name: "STX", mode: "zeroPage", cycle: cycles[0x86] });
    //map.insert(0x8E, Opecode { name: "STX", mode: "absolute", cycle: cycles[0x8E] });
    //map.insert(0x96, Opecode { name: "STX", mode: "zeroPageY", cycle: cycles[0x96] });
    //map.insert(0x84, Opecode { name: "STY", mode: "zeroPage", cycle: cycles[0x84] });
    //map.insert(0x8C, Opecode { name: "STY", mode: "absolute", cycle: cycles[0x8C] });
    //map.insert(0x94, Opecode { name: "STY", mode: "zeroPageX", cycle: cycles[0x94] });
    //map.insert(0x8A, Opecode { name: "TXA", mode: "implied", cycle: cycles[0x8A] });
    //map.insert(0x98, Opecode { name: "TYA", mode: "implied", cycle: cycles[0x98] });
    //map.insert(0x9A, Opecode { name: "TXS", mode: "implied", cycle: cycles[0x9A] });
    //map.insert(0xA8, Opecode { name: "TAY", mode: "implied", cycle: cycles[0xA8] });
    //map.insert(0xAA, Opecode { name: "TAX", mode: "implied", cycle: cycles[0xAA] });
    //map.insert(0xBA, Opecode { name: "TSX", mode: "implied", cycle: cycles[0xBA] });
    //map.insert(0x08, Opecode { name: "PHP", mode: "implied", cycle: cycles[0x08] });
    //map.insert(0x28, Opecode { name: "PLP", mode: "implied", cycle: cycles[0x28] });
    //map.insert(0x48, Opecode { name: "PHA", mode: "implied", cycle: cycles[0x48] });
    //map.insert(0x68, Opecode { name: "PLA", mode: "implied", cycle: cycles[0x68] });
    //map.insert(0x69, Opecode { name: "ADC", mode: "immediate", cycle: cycles[0x69] });
    //map.insert(0x65, Opecode { name: "ADC", mode: "zeroPage", cycle: cycles[0x65] });
    //map.insert(0x6D, Opecode { name: "ADC", mode: "absolute", cycle: cycles[0x6D] });
    //map.insert(0x75, Opecode { name: "ADC", mode: "zeroPageX", cycle: cycles[0x75] });
    //map.insert(0x7D, Opecode { name: "ADC", mode: "absoluteX", cycle: cycles[0x7D] });
    //map.insert(0x79, Opecode { name: "ADC", mode: "absoluteY", cycle: cycles[0x79] });
    //map.insert(0x61, Opecode { name: "ADC", mode: "preIndexedIndirect", cycle: cycles[0x61] });
    //map.insert(0x71, Opecode { name: "ADC", mode: "postIndexedIndirect", cycle: cycles[0x71] });
    //map.insert(0xE9, Opecode { name: "SBC", mode: "immediate", cycle: cycles[0xE9] });
    //map.insert(0xE5, Opecode { name: "SBC", mode: "zeroPage", cycle: cycles[0xE5] });
    //map.insert(0xED, Opecode { name: "SBC", mode: "absolute", cycle: cycles[0xED] });
    //map.insert(0xF5, Opecode { name: "SBC", mode: "zeroPageX", cycle: cycles[0xF5] });
    //map.insert(0xFD, Opecode { name: "SBC", mode: "absoluteX", cycle: cycles[0xFD] });
    //map.insert(0xF9, Opecode { name: "SBC", mode: "absoluteY", cycle: cycles[0xF9] });
    //map.insert(0xE1, Opecode { name: "SBC", mode: "preIndexedIndirect", cycle: cycles[0xE1] });
    //map.insert(0xF1, Opecode { name: "SBC", mode: "postIndexedIndirect", cycle: cycles[0xF1] });
    //map.insert(0xE0, Opecode { name: "CPX", mode: "immediate", cycle: cycles[0xE0] });
    //map.insert(0xE4, Opecode { name: "CPX", mode: "zeroPage", cycle: cycles[0xE4] });
    //map.insert(0xEC, Opecode { name: "CPX", mode: "absolute", cycle: cycles[0xEC] });
    //map.insert(0xC0, Opecode { name: "CPY", mode: "immediate", cycle: cycles[0xC0] });
    //map.insert(0xC4, Opecode { name: "CPY", mode: "zeroPage", cycle: cycles[0xC4] });
    //map.insert(0xCC, Opecode { name: "CPY", mode: "absolute", cycle: cycles[0xCC] });
    //map.insert(0xC9, Opecode { name: "CMP", mode: "immediate", cycle: cycles[0xC9] });
    //map.insert(0xC5, Opecode { name: "CMP", mode: "zeroPage", cycle: cycles[0xC5] });
    //map.insert(0xCD, Opecode { name: "CMP", mode: "absolute", cycle: cycles[0xCD] });
    //map.insert(0xD5, Opecode { name: "CMP", mode: "zeroPageX", cycle: cycles[0xD5] });
    //map.insert(0xDD, Opecode { name: "CMP", mode: "absoluteX", cycle: cycles[0xDD] });
    //map.insert(0xD9, Opecode { name: "CMP", mode: "absoluteY", cycle: cycles[0xD9] });
    //map.insert(0xC1, Opecode { name: "CMP", mode: "preIndexedIndirect", cycle: cycles[0xC1] });
    //map.insert(0xD1, Opecode { name: "CMP", mode: "postIndexedIndirect", cycle: cycles[0xD1] });
    //map.insert(0x29, Opecode { name: "AND", mode: "immediate", cycle: cycles[0x29] });
    //map.insert(0x25, Opecode { name: "AND", mode: "zeroPage", cycle: cycles[0x25] });
    //map.insert(0x2D, Opecode { name: "AND", mode: "absolute", cycle: cycles[0x2D] });
    //map.insert(0x35, Opecode { name: "AND", mode: "zeroPageX", cycle: cycles[0x35] });
    //map.insert(0x3D, Opecode { name: "AND", mode: "absoluteX", cycle: cycles[0x3D] });
    //map.insert(0x39, Opecode { name: "AND", mode: "absoluteY", cycle: cycles[0x39] });
    //map.insert(0x21, Opecode { name: "AND", mode: "preIndexedIndirect", cycle: cycles[0x21] });
    //map.insert(0x31, Opecode { name: "AND", mode: "postIndexedIndirect", cycle: cycles[0x31] });
    //map.insert(0x49, Opecode { name: "EOR", mode: "immediate", cycle: cycles[0x49] });
    //map.insert(0x45, Opecode { name: "EOR", mode: "zeroPage", cycle: cycles[0x45] });
    //map.insert(0x4D, Opecode { name: "EOR", mode: "absolute", cycle: cycles[0x4D] });
    //map.insert(0x55, Opecode { name: "EOR", mode: "zeroPageX", cycle: cycles[0x55] });
    //map.insert(0x5D, Opecode { name: "EOR", mode: "absoluteX", cycle: cycles[0x5D] });
    //map.insert(0x59, Opecode { name: "EOR", mode: "absoluteY", cycle: cycles[0x59] });
    //map.insert(0x41, Opecode { name: "EOR", mode: "preIndexedIndirect", cycle: cycles[0x41] });
    //map.insert(0x51, Opecode { name: "EOR", mode: "postIndexedIndirect", cycle: cycles[0x51] });
    //map.insert(0x09, Opecode { name: "ORA", mode: "immediate", cycle: cycles[0x09] });
    //map.insert(0x05, Opecode { name: "ORA", mode: "zeroPage", cycle: cycles[0x05] });
    //map.insert(0x0D, Opecode { name: "ORA", mode: "absolute", cycle: cycles[0x0D] });
    //map.insert(0x15, Opecode { name: "ORA", mode: "zeroPageX", cycle: cycles[0x15] });
    //map.insert(0x1D, Opecode { name: "ORA", mode: "absoluteX", cycle: cycles[0x1D] });
    //map.insert(0x19, Opecode { name: "ORA", mode: "absoluteY", cycle: cycles[0x19] });
    //map.insert(0x01, Opecode { name: "ORA", mode: "preIndexedIndirect", cycle: cycles[0x01] });
    //map.insert(0x11, Opecode { name: "ORA", mode: "postIndexedIndirect", cycle: cycles[0x11] });
    //map.insert(0x24, Opecode { name: "BIT", mode: "zeroPage", cycle: cycles[0x24] });
    //map.insert(0x2C, Opecode { name: "BIT", mode: "absolute", cycle: cycles[0x2C] });
    //map.insert(0x0A, Opecode { name: "ASL", mode: "accumulator", cycle: cycles[0x0A] });
    //map.insert(0x06, Opecode { name: "ASL", mode: "zeroPage", cycle: cycles[0x06] });
    //map.insert(0x0E, Opecode { name: "ASL", mode: "absolute", cycle: cycles[0x0E] });
    //map.insert(0x16, Opecode { name: "ASL", mode: "zeroPageX", cycle: cycles[0x16] });
    //map.insert(0x1E, Opecode { name: "ASL", mode: "absoluteX", cycle: cycles[0x1E] });
    //map.insert(0x4A, Opecode { name: "LSR", mode: "accumulator", cycle: cycles[0x4A] });
    //map.insert(0x46, Opecode { name: "LSR", mode: "zeroPage", cycle: cycles[0x46] });
    //map.insert(0x4E, Opecode { name: "LSR", mode: "absolute", cycle: cycles[0x4E] });
    //map.insert(0x56, Opecode { name: "LSR", mode: "zeroPageX", cycle: cycles[0x56] });
    //map.insert(0x5E, Opecode { name: "LSR", mode: "absoluteX", cycle: cycles[0x5E] });
    //map.insert(0x2A, Opecode { name: "ROL", mode: "accumulator", cycle: cycles[0x2A] });
    //map.insert(0x26, Opecode { name: "ROL", mode: "zeroPage", cycle: cycles[0x26] });
    //map.insert(0x2E, Opecode { name: "ROL", mode: "absolute", cycle: cycles[0x2E] });
    //map.insert(0x36, Opecode { name: "ROL", mode: "zeroPageX", cycle: cycles[0x36] });
    //map.insert(0x3E, Opecode { name: "ROL", mode: "absoluteX", cycle: cycles[0x3E] });
    //map.insert(0x6A, Opecode { name: "ROR", mode: "accumulator", cycle: cycles[0x6A] });
    //map.insert(0x66, Opecode { name: "ROR", mode: "zeroPage", cycle: cycles[0x66] });
    //map.insert(0x6E, Opecode { name: "ROR", mode: "absolute", cycle: cycles[0x6E] });
    //map.insert(0x76, Opecode { name: "ROR", mode: "zeroPageX", cycle: cycles[0x76] });
    //map.insert(0x7E, Opecode { name: "ROR", mode: "absoluteX", cycle: cycles[0x7E] });
    //map.insert(0xE8, Opecode { name: "INX", mode: "implied", cycle: cycles[0xE8] });
    //map.insert(0xC8, Opecode { name: "INY", mode: "implied", cycle: cycles[0xC8] });
    //map.insert(0xE6, Opecode { name: "INC", mode: "zeroPage", cycle: cycles[0xE6] });
    //map.insert(0xEE, Opecode { name: "INC", mode: "absolute", cycle: cycles[0xEE] });
    //map.insert(0xF6, Opecode { name: "INC", mode: "zeroPageX", cycle: cycles[0xF6] });
    //map.insert(0xFE, Opecode { name: "INC", mode: "absoluteX", cycle: cycles[0xFE] });
    //map.insert(0xCA, Opecode { name: "DEX", mode: "implied", cycle: cycles[0xCA] });
    //map.insert(0x88, Opecode { name: "DEY", mode: "implied", cycle: cycles[0x88] });
    //map.insert(0xC6, Opecode { name: "DEC", mode: "zeroPage", cycle: cycles[0xC6] });
    //map.insert(0xCE, Opecode { name: "DEC", mode: "absolute", cycle: cycles[0xCE] });
    //map.insert(0xD6, Opecode { name: "DEC", mode: "zeroPageX", cycle: cycles[0xD6] });
    //map.insert(0xDE, Opecode { name: "DEC", mode: "absoluteX", cycle: cycles[0xDE] });
    //map.insert(0x18, Opecode { name: "CLC", mode: "implied", cycle: cycles[0x18] });
    //map.insert(0x58, Opecode { name: "CLI", mode: "implied", cycle: cycles[0x58] });
    //map.insert(0xB8, Opecode { name: "CLV", mode: "implied", cycle: cycles[0xB8] });
    //map.insert(0x38, Opecode { name: "SEC", mode: "implied", cycle: cycles[0x38] });
    //map.insert(0x78, Opecode { name: "SEI", mode: "implied", cycle: cycles[0x78] });
    //map.insert(0xEA, Opecode { name: "NOP", mode: "implied", cycle: cycles[0xEA] });
    //map.insert(0x00, Opecode { name: "BRK", mode: "implied", cycle: cycles[0x00] });
    //map.insert(0x20, Opecode { name: "JSR", mode: "absolute", cycle: cycles[0x20] });
    //map.insert(0x4C, Opecode { name: "JMP", mode: "absolute", cycle: cycles[0x4C] });
    //map.insert(0x6C, Opecode { name: "JMP", mode: "indirectAbsolute", cycle: cycles[0x6C] });
    //map.insert(0x40, Opecode { name: "RTI", mode: "implied", cycle: cycles[0x40] });
    //map.insert(0x60, Opecode { name: "RTS", mode: "implied", cycle: cycles[0x60] });
    //map.insert(0x10, Opecode { name: "BPL", mode: "relative", cycle: cycles[0x10] });
    //map.insert(0x30, Opecode { name: "BMI", mode: "relative", cycle: cycles[0x30] });
    //map.insert(0x50, Opecode { name: "BVC", mode: "relative", cycle: cycles[0x50] });
    //map.insert(0x70, Opecode { name: "BVS", mode: "relative", cycle: cycles[0x70] });
    //map.insert(0x90, Opecode { name: "BCC", mode: "relative", cycle: cycles[0x90] });
    //map.insert(0xB0, Opecode { name: "BCS", mode: "relative", cycle: cycles[0xB0] });
    //map.insert(0xD0, Opecode { name: "BNE", mode: "relative", cycle: cycles[0xD0] });
    //map.insert(0xF0, Opecode { name: "BEQ", mode: "relative", cycle: cycles[0xF0] });
    //map.insert(0xF8, Opecode { name: "SED", mode: "implied", cycle: cycles[0xF8] });
    //map.insert(0xD8, Opecode { name: "CLD", mode: "implied", cycle: cycles[0xD8] });
    //map.insert(0x1A, Opecode { name: "NOP", mode: "implied", cycle: cycles[0x1A] });
    //map.insert(0x3A, Opecode { name: "NOP", mode: "implied", cycle: cycles[0x3A] });
    //map.insert(0x5A, Opecode { name: "NOP", mode: "implied", cycle: cycles[0x5A] });
    //map.insert(0x7A, Opecode { name: "NOP", mode: "implied", cycle: cycles[0x7A] });
    //map.insert(0xDA, Opecode { name: "NOP", mode: "implied", cycle: cycles[0xDA] });
    //map.insert(0xFA, Opecode { name: "NOP", mode: "implied", cycle: cycles[0xFA] });
    //map.insert(0x02, Opecode { name: "NOP", mode: "implied", cycle: cycles[0x02] });
    //map.insert(0x12, Opecode { name: "NOP", mode: "implied", cycle: cycles[0x12] });
    //map.insert(0x22, Opecode { name: "NOP", mode: "implied", cycle: cycles[0x22] });
    //map.insert(0x32, Opecode { name: "NOP", mode: "implied", cycle: cycles[0x32] });
    //map.insert(0x42, Opecode { name: "NOP", mode: "implied", cycle: cycles[0x42] });
    //map.insert(0x52, Opecode { name: "NOP", mode: "implied", cycle: cycles[0x52] });
    //map.insert(0x62, Opecode { name: "NOP", mode: "implied", cycle: cycles[0x62] });
    //map.insert(0x72, Opecode { name: "NOP", mode: "implied", cycle: cycles[0x72] });
    //map.insert(0x92, Opecode { name: "NOP", mode: "implied", cycle: cycles[0x92] });
    //map.insert(0xB2, Opecode { name: "NOP", mode: "implied", cycle: cycles[0xB2] });
    //map.insert(0xD2, Opecode { name: "NOP", mode: "implied", cycle: cycles[0xD2] });
    //map.insert(0xF2, Opecode { name: "NOP", mode: "implied", cycle: cycles[0xF2] });
    //map.insert(0x80, Opecode { name: "NOPD", mode: "implied", cycle: cycles[0x80] });
    //map.insert(0x82, Opecode { name: "NOPD", mode: "implied", cycle: cycles[0x82] });
    //map.insert(0x89, Opecode { name: "NOPD", mode: "implied", cycle: cycles[0x89] });
    //map.insert(0xC2, Opecode { name: "NOPD", mode: "implied", cycle: cycles[0xC2] });
    //map.insert(0xE2, Opecode { name: "NOPD", mode: "implied", cycle: cycles[0xE2] });
    //map.insert(0x04, Opecode { name: "NOPD", mode: "implied", cycle: cycles[0x04] });
    //map.insert(0x44, Opecode { name: "NOPD", mode: "implied", cycle: cycles[0x44] });
    //map.insert(0x64, Opecode { name: "NOPD", mode: "implied", cycle: cycles[0x64] });
    //map.insert(0x14, Opecode { name: "NOPD", mode: "implied", cycle: cycles[0x14] });
    //map.insert(0x34, Opecode { name: "NOPD", mode: "implied", cycle: cycles[0x34] });
    //map.insert(0x54, Opecode { name: "NOPD", mode: "implied", cycle: cycles[0x54] });
    //map.insert(0x74, Opecode { name: "NOPD", mode: "implied", cycle: cycles[0x74] });
    //map.insert(0xD4, Opecode { name: "NOPD", mode: "implied", cycle: cycles[0xD4] });
    //map.insert(0xF4, Opecode { name: "NOPD", mode: "implied", cycle: cycles[0xF4] });
    //map.insert(0x0C, Opecode { name: "NOPI", mode: "implied", cycle: cycles[0x0C] });
    //map.insert(0x1C, Opecode { name: "NOPI", mode: "implied", cycle: cycles[0x1C] });
    //map.insert(0x3C, Opecode { name: "NOPI", mode: "implied", cycle: cycles[0x3C] });
    //map.insert(0x5C, Opecode { name: "NOPI", mode: "implied", cycle: cycles[0x5C] });
    //map.insert(0x7C, Opecode { name: "NOPI", mode: "implied", cycle: cycles[0x7C] });
    //map.insert(0xDC, Opecode { name: "NOPI", mode: "implied", cycle: cycles[0xDC] });
    //map.insert(0xFC, Opecode { name: "NOPI", mode: "implied", cycle: cycles[0xFC] });
    //map.insert(0xA7, Opecode { name: "LAX", mode: "zeroPage", cycle: cycles[0xA7] });
    //map.insert(0xB7, Opecode { name: "LAX", mode: "zeroPageY", cycle: cycles[0xB7] });
    //map.insert(0xAF, Opecode { name: "LAX", mode: "absolute", cycle: cycles[0xAF] });
    //map.insert(0xBF, Opecode { name: "LAX", mode: "absoluteY", cycle: cycles[0xBF] });
    //map.insert(0xA3, Opecode { name: "LAX", mode: "preIndexedIndirect", cycle: cycles[0xA3] });
    //map.insert(0xB3, Opecode { name: "LAX", mode: "postIndexedIndirect", cycle: cycles[0xB3] });
    //map.insert(0x87, Opecode { name: "SAX", mode: "zeroPage", cycle: cycles[0x87] });
    //map.insert(0x97, Opecode { name: "SAX", mode: "zeroPageY", cycle: cycles[0x97] });
    //map.insert(0x8F, Opecode { name: "SAX", mode: "absolute", cycle: cycles[0x8F] });
    //map.insert(0x83, Opecode { name: "SAX", mode: "preIndexedIndirect", cycle: cycles[0x83] });
    //map.insert(0xEB, Opecode { name: "SBC", mode: "immediate", cycle: cycles[0xEB] });
    //map.insert(0xC7, Opecode { name: "DCP", mode: "zeroPage", cycle: cycles[0xC7] });
    //map.insert(0xD7, Opecode { name: "DCP", mode: "zeroPageX", cycle: cycles[0xD7] });
    //map.insert(0xCF, Opecode { name: "DCP", mode: "absolute", cycle: cycles[0xCF] });
    //map.insert(0xDF, Opecode { name: "DCP", mode: "absoluteX", cycle: cycles[0xDF] });
    //map.insert(0xDB, Opecode { name: "DCP", mode: "absoluteY", cycle: cycles[0xD8] });
    //map.insert(0xC3, Opecode { name: "DCP", mode: "preIndexedIndirect", cycle: cycles[0xC3] });
    //map.insert(0xD3, Opecode { name: "DCP", mode: "postIndexedIndirect", cycle: cycles[0xD3] });
    //map.insert(0xE7, Opecode { name: "ISB", mode: "zeroPage", cycle: cycles[0xE7] });
    //map.insert(0xF7, Opecode { name: "ISB", mode: "zeroPageX", cycle: cycles[0xF7] });
    //map.insert(0xEF, Opecode { name: "ISB", mode: "absolute", cycle: cycles[0xEF] });
    //map.insert(0xFF, Opecode { name: "ISB", mode: "absoluteX", cycle: cycles[0xFF] });
    //map.insert(0xFB, Opecode { name: "ISB", mode: "absoluteY", cycle: cycles[0xF8] });
    //map.insert(0xE3, Opecode { name: "ISB", mode: "preIndexedIndirect", cycle: cycles[0xE3] });
    //map.insert(0xF3, Opecode { name: "ISB", mode: "postIndexedIndirect", cycle: cycles[0xF3] });
    //map.insert(0x07, Opecode { name: "SLO", mode: "zeroPage", cycle: cycles[0x07] });
    //map.insert(0x17, Opecode { name: "SLO", mode: "zeroPageX", cycle: cycles[0x17] });
    //map.insert(0x0F, Opecode { name: "SLO", mode: "absolute", cycle: cycles[0x0F] });
    //map.insert(0x1F, Opecode { name: "SLO", mode: "absoluteX", cycle: cycles[0x1F] });
    //map.insert(0x1B, Opecode { name: "SLO", mode: "absoluteY", cycle: cycles[0x1B] });
    //map.insert(0x03, Opecode { name: "SLO", mode: "preIndexedIndirect", cycle: cycles[0x03] });
    //map.insert(0x13, Opecode { name: "SLO", mode: "postIndexedIndirect", cycle: cycles[0x13] });
    //map.insert(0x27, Opecode { name: "RLA", mode: "zeroPage", cycle: cycles[0x27] });
    //map.insert(0x37, Opecode { name: "RLA", mode: "zeroPageX", cycle: cycles[0x37] });
    //map.insert(0x2F, Opecode { name: "RLA", mode: "absolute", cycle: cycles[0x2F] });
    //map.insert(0x3F, Opecode { name: "RLA", mode: "absoluteX", cycle: cycles[0x3F] });
    //map.insert(0x3B, Opecode { name: "RLA", mode: "absoluteY", cycle: cycles[0x3B] });
    //map.insert(0x23, Opecode { name: "RLA", mode: "preIndexedIndirect", cycle: cycles[0x23] });
    //map.insert(0x33, Opecode { name: "RLA", mode: "postIndexedIndirect", cycle: cycles[0x33] }, );
    //map.insert(0x47, Opecode { name: "SRE", mode: "zeroPage", cycle: cycles[0x47] });
    //map.insert(0x57, Opecode { name: "SRE", mode: "zeroPageX", cycle: cycles[0x57] });
    //map.insert(0x4F, Opecode { name: "SRE", mode: "absolute", cycle: cycles[0x4F] });
    //map.insert(0x5F, Opecode { name: "SRE", mode: "absoluteX", cycle: cycles[0x5F] });
    //map.insert(0x5B, Opecode { name: "SRE", mode: "absoluteY", cycle: cycles[0x5B] });
    //map.insert(0x43, Opecode { name: "SRE", mode: "preIndexedIndirect", cycle: cycles[0x43] });
    //map.insert(0x53, Opecode { name: "SRE", mode: "postIndexedIndirect", cycle: cycles[0x53] }, );
    //map.insert(0x67, Opecode { name: "RRA", mode: "zeroPage", cycle: cycles[0x67] });
    //map.insert(0x77, Opecode { name: "RRA", mode: "zeroPageX", cycle: cycles[0x77] });
    //map.insert(0x6F, Opecode { name: "RRA", mode: "absolute", cycle: cycles[0x6F] });
    //map.insert(0x7F, Opecode { name: "RRA", mode: "absoluteX", cycle: cycles[0x7F] });
    //map.insert(0x7B, Opecode { name: "RRA", mode: "absoluteY", cycle: cycles[0x7B] });
    //map.insert(0x63, Opecode { name: "RRA", mode: "preIndexedIndirect", cycle: cycles[0x63] });
    //map.insert(0x73, Opecode { name: "RRA", mode: "postIndexedIndirect", cycle: cycles[0x73] });
    map
}