// use std::collections::HashMap;

pub mod opecode {

    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Opecode {
        pub name: Instruction,
        pub mode: Addressing,
        pub cycle: u8,
    }

    #[derive(Debug)]
    pub enum Instruction {
        LDA,
        LDX,
        LDY,
        STA,
        STX,
        STY,
        TXA,
        TYA,
        TXS,
        TAY,
        TAX,
        TSX,
        PHP,
        PLP,
        PHA,
        PLA,
        ADC,
        SBC,
        CPX,
        CPY,
        CMP,
        AND,
        EOR,
        ORA,
        BIT,
        ASL,
        LSR,
        ROL,
        ROR,
        INX,
        INY,
        INC,
        DEX,
        DEY,
        DEC,
        CLC,
        CLI,
        CLV,
        SEC,
        SEI,
        NOP,
        BRK,
        JSR,
        JMP,
        RTI,
        RTS,
        BPL,
        BMI,
        BVC,
        BVS,
        BCC,
        BCS,
        BNE,
        BEQ,
        SED,
        CLD,
        LAX,
        SAX,
        DCP,
        ISB,
        SLO,
        RLA,
        SRE,
        RRA,
    }

    #[derive(Debug)]
    pub enum Addressing {
        Immediate,
        ZeroPage,
        Relative,
        Implied,
        Absolute,
        Accumulator,
        ZeroPageX,
        ZeroPageY,
        AbsoluteX,
        AbsoluteY,
        PreIndexedIndirect,
        PostIndexedIndirect,
        IndirectAbsolute,
    }

    lazy_static! {
        
        pub static ref MAP: HashMap<u8, Opecode> = {

            #[cfg_attr(rustfmt, rustfmt_skip)]
            let cycles: Vec<u8> =
                vec![7, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7,
                     4, 4, 6, 7, 6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6,
                     2, 4, 2, 7, 4, 4, 6, 7, 6, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 3, 4, 6, 6, 2, 5, 2, 8,
                     4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7, 6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 5, 4, 6, 6,
                     2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7, 2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2,
                     4, 4, 4, 4, 2, 6, 2, 6, 4, 4, 4, 4, 2, 4, 2, 5, 5, 4, 5, 5, 2, 6, 2, 6, 3, 3, 3, 3,
                     2, 2, 2, 2, 4, 4, 4, 4, 2, 5, 2, 5, 4, 4, 4, 4, 2, 4, 2, 4, 4, 4, 4, 4, 2, 6, 2, 8,
                     3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
                     2, 6, 3, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7,
                     4, 4, 7, 7];            
            let mut m = HashMap::new();
            m.insert(0xA9, Opecode { name: Instruction::LDA, mode: Addressing::Immediate, cycle: cycles[0xA9] });
            m.insert(0xA5, Opecode { name: Instruction::LDA, mode: Addressing::ZeroPage, cycle: cycles[0xA5] });
            m.insert(0xB5, Opecode { name: Instruction::LDA, mode: Addressing::ZeroPageX, cycle: cycles[0xB5] });
            m.insert(0xBD, Opecode { name: Instruction::LDA, mode: Addressing::AbsoluteX, cycle: cycles[0xBD] });
            m.insert(0xB9, Opecode { name: Instruction::LDA, mode: Addressing::AbsoluteY, cycle: cycles[0xB9] });
            m.insert(0xA1, Opecode { name: Instruction::LDA, mode: Addressing::PreIndexedIndirect, cycle: cycles[0xA1] });
            m.insert(0xB1, Opecode { name: Instruction::LDA, mode: Addressing::PostIndexedIndirect, cycle: cycles[0xB1] });
            m.insert(0xA2, Opecode { name: Instruction::LDX, mode: Addressing::Immediate, cycle: cycles[0xA2] });
            m.insert(0xA6, Opecode { name: Instruction::LDX, mode: Addressing::ZeroPage, cycle: cycles[0xA6] });
            m.insert(0xAE, Opecode { name: Instruction::LDX, mode: Addressing::Absolute, cycle: cycles[0xAE] });
            m.insert(0xB6, Opecode { name: Instruction::LDX, mode: Addressing::ZeroPageY, cycle: cycles[0xB6] });
            m.insert(0xBE, Opecode { name: Instruction::LDX, mode: Addressing::AbsoluteY, cycle: cycles[0xBE] });
            m.insert(0xA0, Opecode { name: Instruction::LDY, mode: Addressing::Immediate, cycle: cycles[0xA0] });
            m.insert(0xA4, Opecode { name: Instruction::LDY, mode: Addressing::ZeroPage, cycle: cycles[0xA4] });
            m.insert(0xAC, Opecode { name: Instruction::LDY, mode: Addressing::Absolute, cycle: cycles[0xAC] });
            m.insert(0xB4, Opecode { name: Instruction::LDY, mode: Addressing::ZeroPageX, cycle: cycles[0xB4] });
            m.insert(0xBC, Opecode { name: Instruction::LDY, mode: Addressing::AbsoluteX, cycle: cycles[0xBC] });
            m.insert(0x85, Opecode { name: Instruction::STA, mode: Addressing::ZeroPage, cycle: cycles[0x85] });
            m.insert(0x8D, Opecode { name: Instruction::STA, mode: Addressing::Absolute, cycle: cycles[0x8D] });
            m.insert(0x95, Opecode { name: Instruction::STA, mode: Addressing::ZeroPageX, cycle: cycles[0x95] });
            m.insert(0x9D, Opecode { name: Instruction::STA, mode: Addressing::AbsoluteX, cycle: cycles[0x9D] });
            m.insert(0x99, Opecode { name: Instruction::STA, mode: Addressing::AbsoluteY, cycle: cycles[0x99] });
            m.insert(0x81, Opecode { name: Instruction::STA, mode: Addressing::PreIndexedIndirect, cycle: cycles[0x81] });
            m.insert(0x91, Opecode { name: Instruction::STA, mode: Addressing::PostIndexedIndirect, cycle: cycles[0x91] });
            m.insert(0x86, Opecode { name: Instruction::STX, mode: Addressing::ZeroPage, cycle: cycles[0x86] });
            m.insert(0x8E, Opecode { name: Instruction::STX, mode: Addressing::Absolute, cycle: cycles[0x8E] });
            m.insert(0x96, Opecode { name: Instruction::STX, mode: Addressing::ZeroPageY, cycle: cycles[0x96] });
            m.insert(0x84, Opecode { name: Instruction::STY, mode: Addressing::ZeroPage, cycle: cycles[0x84] });
            m.insert(0x8C, Opecode { name: Instruction::STY, mode: Addressing::Absolute, cycle: cycles[0x8C] });
            m.insert(0x94, Opecode { name: Instruction::STY, mode: Addressing::ZeroPageX, cycle: cycles[0x94] });
            m.insert(0x8A, Opecode { name: Instruction::TXA, mode: Addressing::Implied, cycle: cycles[0x8A] });
            m.insert(0x98, Opecode { name: Instruction::TYA, mode: Addressing::Implied, cycle: cycles[0x98] });
            m.insert(0x9A, Opecode { name: Instruction::TXS, mode: Addressing::Implied, cycle: cycles[0x9A] });
            m.insert(0xA8, Opecode { name: Instruction::TAY, mode: Addressing::Implied, cycle: cycles[0xA8] });
            m.insert(0xAA, Opecode { name: Instruction::TAX, mode: Addressing::Implied, cycle: cycles[0xAA] });
            m.insert(0xBA, Opecode { name: Instruction::TSX, mode: Addressing::Implied, cycle: cycles[0xBA] });
            m.insert(0x08, Opecode { name: Instruction::PHP, mode: Addressing::Implied, cycle: cycles[0x08] });
            m.insert(0x28, Opecode { name: Instruction::PLP, mode: Addressing::Implied, cycle: cycles[0x28] });
            m.insert(0x48, Opecode { name: Instruction::PHA, mode: Addressing::Implied, cycle: cycles[0x48] });
            m.insert(0x68, Opecode { name: Instruction::PLA, mode: Addressing::Implied, cycle: cycles[0x68] });
            m.insert(0x69, Opecode { name: Instruction::ADC, mode: Addressing::Immediate, cycle: cycles[0x69] });
            m.insert(0x65, Opecode { name: Instruction::ADC, mode: Addressing::ZeroPage, cycle: cycles[0x65] });
            m.insert(0x6D, Opecode { name: Instruction::ADC, mode: Addressing::Absolute, cycle: cycles[0x6D] });
            m.insert(0x75, Opecode { name: Instruction::ADC, mode: Addressing::ZeroPageX, cycle: cycles[0x75] });
            m.insert(0x7D, Opecode { name: Instruction::ADC, mode: Addressing::AbsoluteX, cycle: cycles[0x7D] });
            m.insert(0x79, Opecode { name: Instruction::ADC, mode: Addressing::AbsoluteY, cycle: cycles[0x79] });
            m.insert(0x61, Opecode { name: Instruction::ADC, mode: Addressing::PreIndexedIndirect, cycle: cycles[0x61] });
            m.insert(0x71, Opecode { name: Instruction::ADC, mode: Addressing::PostIndexedIndirect, cycle: cycles[0x71] });
            m.insert(0xE9, Opecode { name: Instruction::SBC, mode: Addressing::Immediate, cycle: cycles[0xE9] });
            m.insert(0xE5, Opecode { name: Instruction::SBC, mode: Addressing::ZeroPage, cycle: cycles[0xE5] });
            m.insert(0xED, Opecode { name: Instruction::SBC, mode: Addressing::Absolute, cycle: cycles[0xED] });
            m.insert(0xF5, Opecode { name: Instruction::SBC, mode: Addressing::ZeroPageX, cycle: cycles[0xF5] });
            m.insert(0xFD, Opecode { name: Instruction::SBC, mode: Addressing::AbsoluteX, cycle: cycles[0xFD] });
            m.insert(0xF9, Opecode { name: Instruction::SBC, mode: Addressing::AbsoluteY, cycle: cycles[0xF9] });
            m.insert(0xE1, Opecode { name: Instruction::SBC, mode: Addressing::PreIndexedIndirect, cycle: cycles[0xE1] });
            m.insert(0xF1, Opecode { name: Instruction::SBC, mode: Addressing::PostIndexedIndirect, cycle: cycles[0xF1] });
            m.insert(0xE0, Opecode { name: Instruction::CPX, mode: Addressing::Immediate, cycle: cycles[0xE0] });
            m.insert(0xE4, Opecode { name: Instruction::CPX, mode: Addressing::ZeroPage, cycle: cycles[0xE4] });
            m.insert(0xEC, Opecode { name: Instruction::CPX, mode: Addressing::Absolute, cycle: cycles[0xEC] });
            m.insert(0xC0, Opecode { name: Instruction::CPY, mode: Addressing::Immediate, cycle: cycles[0xC0] });
            m.insert(0xC4, Opecode { name: Instruction::CPY, mode: Addressing::ZeroPage, cycle: cycles[0xC4] });
            m.insert(0xCC, Opecode { name: Instruction::CPY, mode: Addressing::Absolute, cycle: cycles[0xCC] });
            m.insert(0xC9, Opecode { name: Instruction::CMP, mode: Addressing::Immediate, cycle: cycles[0xC9] });
            m.insert(0xC5, Opecode { name: Instruction::CMP, mode: Addressing::ZeroPage, cycle: cycles[0xC5] });
            m.insert(0xCD, Opecode { name: Instruction::CMP, mode: Addressing::Absolute, cycle: cycles[0xCD] });
            m.insert(0xD5, Opecode { name: Instruction::CMP, mode: Addressing::ZeroPageX, cycle: cycles[0xD5] });
            m.insert(0xDD, Opecode { name: Instruction::CMP, mode: Addressing::AbsoluteX, cycle: cycles[0xDD] });
            m.insert(0xD9, Opecode { name: Instruction::CMP, mode: Addressing::AbsoluteY, cycle: cycles[0xD9] });
            m.insert(0xC1, Opecode { name: Instruction::CMP, mode: Addressing::PreIndexedIndirect, cycle: cycles[0xC1] });
            m.insert(0xD1, Opecode { name: Instruction::CMP, mode: Addressing::PostIndexedIndirect, cycle: cycles[0xD1] });
            m.insert(0x29, Opecode { name: Instruction::AND, mode: Addressing::Immediate, cycle: cycles[0x29] });
            m.insert(0x25, Opecode { name: Instruction::AND, mode: Addressing::ZeroPage, cycle: cycles[0x25] });
            m.insert(0x2D, Opecode { name: Instruction::AND, mode: Addressing::Absolute, cycle: cycles[0x2D] });
            m.insert(0x35, Opecode { name: Instruction::AND, mode: Addressing::ZeroPageX, cycle: cycles[0x35] });
            m.insert(0x3D, Opecode { name: Instruction::AND, mode: Addressing::AbsoluteX, cycle: cycles[0x3D] });
            m.insert(0x39, Opecode { name: Instruction::AND, mode: Addressing::AbsoluteY, cycle: cycles[0x39] });
            m.insert(0x21, Opecode { name: Instruction::AND, mode: Addressing::PreIndexedIndirect, cycle: cycles[0x21] });
            m.insert(0x31, Opecode { name: Instruction::AND, mode: Addressing::PostIndexedIndirect, cycle: cycles[0x31] });
            m.insert(0x49, Opecode { name: Instruction::EOR, mode: Addressing::Immediate, cycle: cycles[0x49] });
            m.insert(0x45, Opecode { name: Instruction::EOR, mode: Addressing::ZeroPage, cycle: cycles[0x45] });
            m.insert(0x4D, Opecode { name: Instruction::EOR, mode: Addressing::Absolute, cycle: cycles[0x4D] });
            m.insert(0x55, Opecode { name: Instruction::EOR, mode: Addressing::ZeroPageX, cycle: cycles[0x55] });
            m.insert(0x5D, Opecode { name: Instruction::EOR, mode: Addressing::AbsoluteX, cycle: cycles[0x5D] });
            m.insert(0x59, Opecode { name: Instruction::EOR, mode: Addressing::AbsoluteY, cycle: cycles[0x59] });
            m.insert(0x41, Opecode { name: Instruction::EOR, mode: Addressing::PreIndexedIndirect, cycle: cycles[0x41] });
            m.insert(0x51, Opecode { name: Instruction::EOR, mode: Addressing::PostIndexedIndirect, cycle: cycles[0x51] });
            m.insert(0x09, Opecode { name: Instruction::ORA, mode: Addressing::Immediate, cycle: cycles[0x09] });
            m.insert(0x05, Opecode { name: Instruction::ORA, mode: Addressing::ZeroPage, cycle: cycles[0x05] });
            m.insert(0x0D, Opecode { name: Instruction::ORA, mode: Addressing::Absolute, cycle: cycles[0x0D] });
            m.insert(0x15, Opecode { name: Instruction::ORA, mode: Addressing::ZeroPageX, cycle: cycles[0x15] });
            m.insert(0x1D, Opecode { name: Instruction::ORA, mode: Addressing::AbsoluteX, cycle: cycles[0x1D] });
            m.insert(0x19, Opecode { name: Instruction::ORA, mode: Addressing::AbsoluteY, cycle: cycles[0x19] });
            m.insert(0x01, Opecode { name: Instruction::ORA, mode: Addressing::PreIndexedIndirect, cycle: cycles[0x01] });
            m.insert(0x11, Opecode { name: Instruction::ORA, mode: Addressing::PostIndexedIndirect, cycle: cycles[0x11] });
            m.insert(0x24, Opecode { name: Instruction::BIT, mode: Addressing::ZeroPage, cycle: cycles[0x24] });
            m.insert(0x2C, Opecode { name: Instruction::BIT, mode: Addressing::Absolute, cycle: cycles[0x2C] });
            m.insert(0x0A, Opecode { name: Instruction::ASL, mode: Addressing::Accumulator, cycle: cycles[0x0A] });
            m.insert(0x06, Opecode { name: Instruction::ASL, mode: Addressing::ZeroPage, cycle: cycles[0x06] });
            m.insert(0x0E, Opecode { name: Instruction::ASL, mode: Addressing::Absolute, cycle: cycles[0x0E] });
            m.insert(0x16, Opecode { name: Instruction::ASL, mode: Addressing::ZeroPageX, cycle: cycles[0x16] });
            m.insert(0x1E, Opecode { name: Instruction::ASL, mode: Addressing::AbsoluteX, cycle: cycles[0x1E] });
            m.insert(0x4A, Opecode { name: Instruction::LSR, mode: Addressing::Accumulator, cycle: cycles[0x4A] });
            m.insert(0x46, Opecode { name: Instruction::LSR, mode: Addressing::ZeroPage, cycle: cycles[0x46] });
            m.insert(0x4E, Opecode { name: Instruction::LSR, mode: Addressing::Absolute, cycle: cycles[0x4E] });
            m.insert(0x56, Opecode { name: Instruction::LSR, mode: Addressing::ZeroPageX, cycle: cycles[0x56] });
            m.insert(0x5E, Opecode { name: Instruction::LSR, mode: Addressing::AbsoluteX, cycle: cycles[0x5E] });
            m.insert(0x2A, Opecode { name: Instruction::ROL, mode: Addressing::Accumulator, cycle: cycles[0x2A] });
            m.insert(0x26, Opecode { name: Instruction::ROL, mode: Addressing::ZeroPage, cycle: cycles[0x26] });
            m.insert(0x2E, Opecode { name: Instruction::ROL, mode: Addressing::Absolute, cycle: cycles[0x2E] });
            m.insert(0x36, Opecode { name: Instruction::ROL, mode: Addressing::ZeroPageX, cycle: cycles[0x36] });
            m.insert(0x3E, Opecode { name: Instruction::ROL, mode: Addressing::AbsoluteX, cycle: cycles[0x3E] });
            m.insert(0x6A, Opecode { name: Instruction::ROR, mode: Addressing::Accumulator, cycle: cycles[0x6A] });
            m.insert(0x66, Opecode { name: Instruction::ROR, mode: Addressing::ZeroPage, cycle: cycles[0x66] });
            m.insert(0x6E, Opecode { name: Instruction::ROR, mode: Addressing::Absolute, cycle: cycles[0x6E] });
            m.insert(0x76, Opecode { name: Instruction::ROR, mode: Addressing::ZeroPageX, cycle: cycles[0x76] });
            m.insert(0x7E, Opecode { name: Instruction::ROR, mode: Addressing::AbsoluteX, cycle: cycles[0x7E] });
            m.insert(0xE8, Opecode { name: Instruction::INX, mode: Addressing::Implied, cycle: cycles[0xE8] });
            m.insert(0xC8, Opecode { name: Instruction::INY, mode: Addressing::Implied, cycle: cycles[0xC8] });
            m.insert(0xE6, Opecode { name: Instruction::INC, mode: Addressing::ZeroPage, cycle: cycles[0xE6] });
            m.insert(0xEE, Opecode { name: Instruction::INC, mode: Addressing::Absolute, cycle: cycles[0xEE] });
            m.insert(0xF6, Opecode { name: Instruction::INC, mode: Addressing::ZeroPageX, cycle: cycles[0xF6] });
            m.insert(0xFE, Opecode { name: Instruction::INC, mode: Addressing::AbsoluteX, cycle: cycles[0xFE] });
            m.insert(0xCA, Opecode { name: Instruction::DEX, mode: Addressing::Implied, cycle: cycles[0xCA] });
            m.insert(0x88, Opecode { name: Instruction::DEY, mode: Addressing::Implied, cycle: cycles[0x88] });
            m.insert(0xC6, Opecode { name: Instruction::DEC, mode: Addressing::ZeroPage, cycle: cycles[0xC6] });
            m.insert(0xCE, Opecode { name: Instruction::DEC, mode: Addressing::Absolute, cycle: cycles[0xCE] });
            m.insert(0xD6, Opecode { name: Instruction::DEC, mode: Addressing::ZeroPageX, cycle: cycles[0xD6] });
            m.insert(0xDE, Opecode { name: Instruction::DEC, mode: Addressing::AbsoluteX, cycle: cycles[0xDE] });
            m.insert(0x18, Opecode { name: Instruction::CLC, mode: Addressing::Implied, cycle: cycles[0x18] });
            m.insert(0x58, Opecode { name: Instruction::CLI, mode: Addressing::Implied, cycle: cycles[0x58] });
            m.insert(0xB8, Opecode { name: Instruction::CLV, mode: Addressing::Implied, cycle: cycles[0xB8] });
            m.insert(0x38, Opecode { name: Instruction::SEC, mode: Addressing::Implied, cycle: cycles[0x38] });
            m.insert(0x78, Opecode { name: Instruction::SEI, mode: Addressing::Implied, cycle: cycles[0x78] });
            m.insert(0xEA, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: cycles[0xEA] });
            m.insert(0x00, Opecode { name: Instruction::BRK, mode: Addressing::Implied, cycle: cycles[0x00] });
            m.insert(0x20, Opecode { name: Instruction::JSR, mode: Addressing::Absolute, cycle: cycles[0x20] });
            m.insert(0x4C, Opecode { name: Instruction::JMP, mode: Addressing::Absolute, cycle: cycles[0x4C] });
            m.insert(0x6C, Opecode { name: Instruction::JMP, mode: Addressing::IndirectAbsolute, cycle: cycles[0x6C] });
            m.insert(0x40, Opecode { name: Instruction::RTI, mode: Addressing::Implied, cycle: cycles[0x40] });
            m.insert(0x60, Opecode { name: Instruction::RTS, mode: Addressing::Implied, cycle: cycles[0x60] });
            m.insert(0x10, Opecode { name: Instruction::BPL, mode: Addressing::Relative, cycle: cycles[0x10] });
            m.insert(0x30, Opecode { name: Instruction::BMI, mode: Addressing::Relative, cycle: cycles[0x30] });
            m.insert(0x50, Opecode { name: Instruction::BVC, mode: Addressing::Relative, cycle: cycles[0x50] });
            m.insert(0x70, Opecode { name: Instruction::BVS, mode: Addressing::Relative, cycle: cycles[0x70] });
            m.insert(0x90, Opecode { name: Instruction::BCC, mode: Addressing::Relative, cycle: cycles[0x90] });
            m.insert(0xB0, Opecode { name: Instruction::BCS, mode: Addressing::Relative, cycle: cycles[0xB0] });
            m.insert(0xD0, Opecode { name: Instruction::BNE, mode: Addressing::Relative, cycle: cycles[0xD0] });
            m.insert(0xF0, Opecode { name: Instruction::BEQ, mode: Addressing::Relative, cycle: cycles[0xF0] });
            m.insert(0xF8, Opecode { name: Instruction::SED, mode: Addressing::Implied, cycle: cycles[0xF8] });
            m.insert(0xD8, Opecode { name: Instruction::CLD, mode: Addressing::Implied, cycle: cycles[0xD8] });
            m.insert(0x1A, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: cycles[0x1A] });
            m.insert(0x3A, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: cycles[0x3A] });
            m.insert(0x5A, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: cycles[0x5A] });
            m.insert(0x7A, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: cycles[0x7A] });
            m.insert(0xDA, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: cycles[0xDA] });
            m.insert(0xFA, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: cycles[0xFA] });
            m.insert(0x02, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: cycles[0x02] });
            m.insert(0x12, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: cycles[0x12] });
            m.insert(0x22, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: cycles[0x22] });
            m.insert(0x32, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: cycles[0x32] });
            m.insert(0x42, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: cycles[0x42] });
            m.insert(0x52, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: cycles[0x52] });
            m.insert(0x62, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: cycles[0x62] });
            m.insert(0x72, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: cycles[0x72] });
            m.insert(0x92, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: cycles[0x92] });
            m.insert(0xB2, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: cycles[0xB2] });
            m.insert(0xD2, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: cycles[0xD2] });
            m.insert(0xF2, Opecode { name: Instruction::NOP, mode: Addressing::Implied, cycle: cycles[0xF2] });
            m.insert(0x80, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0x80] });
            m.insert(0x82, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0x82] });
            m.insert(0x89, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0x89] });
            m.insert(0xC2, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0xC2] });
            m.insert(0xE2, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0xE2] });
            m.insert(0x04, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0x04] });
            m.insert(0x44, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0x44] });
            m.insert(0x64, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0x64] });
            m.insert(0x14, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0x14] });
            m.insert(0x34, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0x34] });
            m.insert(0x54, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0x54] });
            m.insert(0x74, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0x74] });
            m.insert(0xD4, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0xD4] });
            m.insert(0xF4, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0xF4] });
            m.insert(0x0C, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0x0C] });
            m.insert(0x1C, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0x1C] });
            m.insert(0x3C, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0x3C] });
            m.insert(0x5C, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0x5C] });
            m.insert(0x7C, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0x7C] });
            m.insert(0xDC, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0xDC] });
            m.insert(0xFC, Opecode { name: Instruction::NOP, mode:Addressing::Implied, cycle: cycles[0xFC] });
            m.insert(0xA7, Opecode { name: Instruction::LAX, mode: Addressing::ZeroPage, cycle: cycles[0xA7] });
            m.insert(0xB7, Opecode { name: Instruction::LAX, mode: Addressing::ZeroPageY, cycle: cycles[0xB7] });
            m.insert(0xAF, Opecode { name: Instruction::LAX, mode: Addressing::Absolute, cycle: cycles[0xAF] });
            m.insert(0xBF, Opecode { name: Instruction::LAX, mode: Addressing::AbsoluteY, cycle: cycles[0xBF] });
            m.insert(0xA3, Opecode { name: Instruction::LAX, mode: Addressing::PreIndexedIndirect, cycle: cycles[0xA3] });
            m.insert(0xB3, Opecode { name: Instruction::LAX, mode: Addressing::PostIndexedIndirect, cycle: cycles[0xB3] });
            m.insert(0x87, Opecode { name: Instruction::SAX, mode: Addressing::ZeroPage, cycle: cycles[0x87] });
            m.insert(0x97, Opecode { name: Instruction::SAX, mode: Addressing::ZeroPageY, cycle: cycles[0x97] });
            m.insert(0x8F, Opecode { name: Instruction::SAX, mode: Addressing::Absolute, cycle: cycles[0x8F] });
            m.insert(0x83, Opecode { name: Instruction::SAX, mode: Addressing::PreIndexedIndirect, cycle: cycles[0x83] });
            m.insert(0xEB, Opecode { name: Instruction::SBC, mode: Addressing::Immediate, cycle: cycles[0xEB] });
            m.insert(0xC7, Opecode { name: Instruction::DCP, mode: Addressing::ZeroPage, cycle: cycles[0xC7] });
            m.insert(0xD7, Opecode { name: Instruction::DCP, mode: Addressing::ZeroPageX, cycle: cycles[0xD7] });
            m.insert(0xCF, Opecode { name: Instruction::DCP, mode: Addressing::Absolute, cycle: cycles[0xCF] });
            m.insert(0xDF, Opecode { name: Instruction::DCP, mode: Addressing::AbsoluteX, cycle: cycles[0xDF] });
            m.insert(0xDB, Opecode { name: Instruction::DCP, mode: Addressing::AbsoluteY, cycle: cycles[0xD8] });
            m.insert(0xC3, Opecode { name: Instruction::DCP, mode: Addressing::PreIndexedIndirect, cycle: cycles[0xC3] });
            m.insert(0xD3, Opecode { name: Instruction::DCP, mode: Addressing::PostIndexedIndirect, cycle: cycles[0xD3] });
            m.insert(0xE7, Opecode { name: Instruction::ISB, mode: Addressing::ZeroPage, cycle: cycles[0xE7] });
            m.insert(0xF7, Opecode { name: Instruction::ISB, mode: Addressing::ZeroPageX, cycle: cycles[0xF7] });
            m.insert(0xEF, Opecode { name: Instruction::ISB, mode: Addressing::Absolute, cycle: cycles[0xEF] });
            m.insert(0xFF, Opecode { name: Instruction::ISB, mode: Addressing::AbsoluteX, cycle: cycles[0xFF] });
            m.insert(0xFB, Opecode { name: Instruction::ISB, mode: Addressing::AbsoluteY, cycle: cycles[0xF8] });
            m.insert(0xE3, Opecode { name: Instruction::ISB, mode: Addressing::PreIndexedIndirect, cycle: cycles[0xE3] });
            m.insert(0xF3, Opecode { name: Instruction::ISB, mode: Addressing::PostIndexedIndirect, cycle: cycles[0xF3] });
            m.insert(0x07, Opecode { name: Instruction::SLO, mode: Addressing::ZeroPage, cycle: cycles[0x07] });
            m.insert(0x17, Opecode { name: Instruction::SLO, mode: Addressing::ZeroPageX, cycle: cycles[0x17] });
            m.insert(0x0F, Opecode { name: Instruction::SLO, mode: Addressing::Absolute, cycle: cycles[0x0F] });
            m.insert(0x1F, Opecode { name: Instruction::SLO, mode: Addressing::AbsoluteX, cycle: cycles[0x1F] });
            m.insert(0x1B, Opecode { name: Instruction::SLO, mode: Addressing::AbsoluteY, cycle: cycles[0x1B] });
            m.insert(0x03, Opecode { name: Instruction::SLO, mode: Addressing::PreIndexedIndirect, cycle: cycles[0x03] });
            m.insert(0x13, Opecode { name: Instruction::SLO, mode: Addressing::PostIndexedIndirect, cycle: cycles[0x13] });
            m.insert(0x27, Opecode { name: Instruction::RLA, mode: Addressing::ZeroPage, cycle: cycles[0x27] });
            m.insert(0x37, Opecode { name: Instruction::RLA, mode: Addressing::ZeroPageX, cycle: cycles[0x37] });
            m.insert(0x2F, Opecode { name: Instruction::RLA, mode: Addressing::Absolute, cycle: cycles[0x2F] });
            m.insert(0x3F, Opecode { name: Instruction::RLA, mode: Addressing::AbsoluteX, cycle: cycles[0x3F] });
            m.insert(0x3B, Opecode { name: Instruction::RLA, mode: Addressing::AbsoluteY, cycle: cycles[0x3B] });
            m.insert(0x23, Opecode { name: Instruction::RLA, mode: Addressing::PreIndexedIndirect, cycle: cycles[0x23] });
            m.insert(0x33, Opecode { name: Instruction::RLA, mode: Addressing::PostIndexedIndirect, cycle: cycles[0x33] }, );
            m.insert(0x47, Opecode { name: Instruction::SRE, mode: Addressing::ZeroPage, cycle: cycles[0x47] });
            m.insert(0x57, Opecode { name: Instruction::SRE, mode: Addressing::ZeroPageX, cycle: cycles[0x57] });
            m.insert(0x4F, Opecode { name: Instruction::SRE, mode: Addressing::Absolute, cycle: cycles[0x4F] });
            m.insert(0x5F, Opecode { name: Instruction::SRE, mode: Addressing::AbsoluteX, cycle: cycles[0x5F] });
            m.insert(0x5B, Opecode { name: Instruction::SRE, mode: Addressing::AbsoluteY, cycle: cycles[0x5B] });
            m.insert(0x43, Opecode { name: Instruction::SRE, mode: Addressing::PreIndexedIndirect, cycle: cycles[0x43] });
            m.insert(0x53, Opecode { name: Instruction::SRE, mode: Addressing::PostIndexedIndirect, cycle: cycles[0x53] }, );
            m.insert(0x67, Opecode { name: Instruction::RRA, mode: Addressing::ZeroPage, cycle: cycles[0x67] });
            m.insert(0x77, Opecode { name: Instruction::RRA, mode: Addressing::ZeroPageX, cycle: cycles[0x77] });
            m.insert(0x6F, Opecode { name: Instruction::RRA, mode: Addressing::Absolute, cycle: cycles[0x6F] });
            m.insert(0x7F, Opecode { name: Instruction::RRA, mode: Addressing::AbsoluteX, cycle: cycles[0x7F] });
            m.insert(0x7B, Opecode { name: Instruction::RRA, mode: Addressing::AbsoluteY, cycle: cycles[0x7B] });
            m.insert(0x63, Opecode { name: Instruction::RRA, mode: Addressing::PreIndexedIndirect, cycle: cycles[0x63] });
            m.insert(0x73, Opecode { name: Instruction::RRA, mode: Addressing::PostIndexedIndirect, cycle: cycles[0x73] });            
            m
        };        
    }
}
