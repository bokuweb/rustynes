mod opecode;
mod registers;

use self::opecode::*;
use self::registers::*;
use super::types::{Data, Addr, Word};
use super::helper::*;

#[derive(Debug)]
pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { registers: Registers::new() }
    }

    pub fn reset<R>(&mut self, read: R)
        where R: Fn(Addr) -> Data
    {
        self.registers.reset();
        let pc = self.read_word(&read, 0xFFFC);
        self.registers.set_pc(pc);
    }

    pub fn run<R, W>(&mut self, read: R, write: W) -> Data
        where R: Fn(Addr) -> Data,
              W: Fn(Addr, Data)
    {
        println!("registers {:?}", self.registers);
        let code = self.fetch(&read);
        let ref map = opecode::MAP;
        let code = &*map.get(&code).unwrap();
        let opeland = self.fetch_opeland(&code, &read);
        match code.name {
            Instruction::LDA => self.lda(&code, opeland, &read),
            Instruction::LDX => self.ldx(&code, opeland, &read),
            Instruction::LDY => self.ldy(&code, opeland, &read),
            Instruction::STA => self.sta(opeland, &write),
            Instruction::STX => self.stx(opeland, &write),
            Instruction::STY => self.sty(opeland, &write),
            Instruction::TXA => self.txa(),
            Instruction::TYA => self.tya(),
            Instruction::TXS => self.txs(),
            Instruction::TAY => self.tay(),
            Instruction::TAX => self.tax(),
            Instruction::TSX => self.tsx(),
            Instruction::PHP => self.php(&write),
            Instruction::PLP => self.plp(&read),
            Instruction::PHA => self.pha(&write),
            Instruction::PLA => self.pla(&read),
            Instruction::ADC => self.adc(&code, opeland, &read),
            Instruction::SBC => self.sbc(&code, opeland, &read),
            Instruction::CPX => self.cpx(&code, opeland, &read),
            Instruction::CPY => println!("{}", "TODO:"),
            Instruction::CMP => println!("{}", "TODO:"),
            Instruction::AND => println!("{}", "TODO:"),
            Instruction::EOR => println!("{}", "TODO:"),
            Instruction::ORA => println!("{}", "TODO:"),
            Instruction::BIT => println!("{}", "TODO:"),
            Instruction::ASL => println!("{}", "TODO:"),
            Instruction::LSR => println!("{}", "TODO:"),
            Instruction::ROL => println!("{}", "TODO:"),
            Instruction::ROR => println!("{}", "TODO:"),
            Instruction::INX => println!("{}", "TODO:"),
            Instruction::INY => println!("{}", "TODO:"),
            Instruction::INC => println!("{}", "TODO:"),
            Instruction::DEX => println!("{}", "TODO:"),
            Instruction::DEY => println!("{}", "TODO:"),
            Instruction::DEC => println!("{}", "TODO:"),
            Instruction::CLC => println!("{}", "TODO:"),
            Instruction::CLI => println!("{}", "TODO:"),
            Instruction::CLV => println!("{}", "TODO:"),
            Instruction::SEC => println!("{}", "TODO:"),
            Instruction::SEI => println!("{}", "TODO:"),
            Instruction::NOP => println!("{}", "TODO:"),
            Instruction::BRK => println!("{}", "TODO:"),
            Instruction::JSR => println!("{}", "TODO:"),
            Instruction::JMP => println!("{}", "TODO:"),
            Instruction::RTI => println!("{}", "TODO:"),
            Instruction::RTS => println!("{}", "TODO:"),
            Instruction::BPL => println!("{}", "TODO:"),
            Instruction::BMI => println!("{}", "TODO:"),
            Instruction::BVC => println!("{}", "TODO:"),
            Instruction::BVS => println!("{}", "TODO:"),
            Instruction::BCC => println!("{}", "TODO:"),
            Instruction::BCS => println!("{}", "TODO:"),
            Instruction::BNE => println!("{}", "TODO:"),
            Instruction::BEQ => println!("{}", "TODO:"),
            Instruction::SED => println!("{}", "TODO:"),
            Instruction::CLD => println!("{}", "TODO:"),
            Instruction::LAX => println!("{}", "TODO:"),
            Instruction::SAX => println!("{}", "TODO:"),
            Instruction::DCP => println!("{}", "TODO:"),
            Instruction::ISB => println!("{}", "TODO:"),
            Instruction::SLO => println!("{}", "TODO:"),
            Instruction::RLA => println!("{}", "TODO:"),
            Instruction::SRE => println!("{}", "TODO:"),
            Instruction::RRA => println!("{}", "TODO:"),
        }
        code.cycle
    }

    fn fetch<R>(&mut self, read: R) -> Data
        where R: Fn(Addr) -> Data
    {
        let code = read(self.registers.get_pc());
        self.registers.update_pc();
        code
    }

    fn fetch_word<R>(&mut self, read: R) -> Word
        where R: Fn(Addr) -> Data
    {
        let lower = read(self.registers.get_pc()) as Word;
        self.registers.update_pc();
        let upper = read(self.registers.get_pc()) as Word;
        self.registers.update_pc();
        (upper << 8 | lower) as Word
    }

    fn read_word<R>(&self, read: R, addr: Addr) -> Word
        where R: Fn(Addr) -> Data
    {
        let lower = read(addr) as Word;
        let upper = read(addr + 1) as Word;
        (upper << 8 | lower) as Word
    }

    fn fetch_relative<R>(&mut self, ref read: &R) -> Word
        where R: Fn(Addr) -> Data
    {
        let base = self.fetch(read) as Word;
        if base < 0x80 {
            base + self.registers.get_pc()
        } else {
            base + self.registers.get_pc() - 256
        }
    }

    fn fetch_zeropage_x<R>(&mut self, ref read: &R) -> Word
        where R: Fn(Addr) -> Data
    {
        let addr = self.fetch(read) as Word;
        (addr + self.registers.get(ByteRegister::X) as Word) & 0xFF as Word
    }

    fn fetch_zeropage_y<R>(&mut self, ref read: &R) -> Word
        where R: Fn(Addr) -> Data
    {
        let addr = self.fetch(read) as Word;
        (addr + self.registers.get(ByteRegister::Y) as Word) & 0xFF as Word
    }

    fn fetch_absolute_x<R>(&mut self, ref read: &R) -> Word
        where R: Fn(Addr) -> Data
    {
        let addr = self.fetch_word(read);
        (addr + self.registers.get(ByteRegister::X) as Word) & 0xFFFF
    }

    fn fetch_absolute_y<R>(&mut self, ref read: &R) -> Word
        where R: Fn(Addr) -> Data
    {
        let addr = self.fetch_word(read);
        (addr + self.registers.get(ByteRegister::Y) as Word) & 0xFFFF
    }

    fn fetch_pre_indexed_indirect<R>(&mut self, ref read: &R) -> Word
        where R: Fn(Addr) -> Data
    {
        let addr = ((self.fetch(read) + self.registers.get(ByteRegister::X)) & 0xFF) as Addr;
        let addr = (read(addr) as Addr) + ((read((addr + 1) as Addr & 0xFF) as Addr) << 8);
        addr & 0xFFFF
    }

    fn fetch_post_indexed_indirect<R>(&mut self, ref read: &R) -> Word
        where R: Fn(Addr) -> Data
    {
        let addr = self.fetch(read) as Addr;
        let addr = (read(addr) as Addr) + ((read((addr + 1) & 0xFF) as Addr) << 8);
        addr + (self.registers.get(ByteRegister::Y) as Addr) & 0xFFFF
    }

    fn fetch_indirect_absolute<R>(&mut self, ref read: &R) -> Word
        where R: Fn(Addr) -> Data
    {
        let addr = self.fetch_word(read);
        let upper = read((addr & 0xFF00) | ((((addr & 0xFF) + 1) & 0xFF)) as Addr) as Addr;
        let addr = (read(addr) as Addr) + (upper << 8) as Addr;
        addr & 0xFFFF
    }

    fn fetch_opeland<R>(&mut self, code: &Opecode, ref read: &R) -> Word
        where R: Fn(Addr) -> Data
    {
        match code.mode {
            Addressing::Accumulator => 0x0000,
            Addressing::Implied => 0x0000,
            Addressing::Immediate => self.fetch(read) as Word,
            Addressing::Relative => self.fetch_relative(read),
            Addressing::ZeroPage => self.fetch(read) as Word,
            Addressing::ZeroPageX => self.fetch_zeropage_x(read),
            Addressing::ZeroPageY => self.fetch_zeropage_y(read),
            Addressing::Absolute => self.fetch_word(read),     
            Addressing::AbsoluteX => self.fetch_absolute_x(read),
            Addressing::AbsoluteY => self.fetch_absolute_y(read),
            Addressing::PreIndexedIndirect => self.fetch_pre_indexed_indirect(read),
            Addressing::PostIndexedIndirect => self.fetch_post_indexed_indirect(read),
            Addressing::IndirectAbsolute => self.fetch_indirect_absolute(read),
        }
    }

    fn branch(&mut self, addr: Addr) {
        self.registers.set_pc(addr);
    }

    fn push_status<W>(&mut self, write: W)
        where W: Fn(Addr, Data)
    {
        let status = self.registers.get(ByteRegister::P);
        self.push(status, &write);
    }

    fn push<W>(&mut self, data: Data, write: W)
        where W: Fn(Addr, Data)
    {
        let addr = self.registers.get(ByteRegister::SP) as Addr;
        write((addr | 0x0100), data);
        self.registers.dec_sp();
    }

    fn pop<R>(&mut self, read: R) -> Data
        where R: Fn(Addr) -> Data
    {
        self.registers.inc_sp();
        let addr = 0x0100 | self.registers.get(ByteRegister::SP) as Addr;
        read(addr)
    }

    fn lda<R>(&mut self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let computed = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        self.registers
            .set_acc(computed)
            .update_negative(computed)
            .update_zero(computed);
    }

    fn ldx<R>(&mut self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let computed = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        self.registers
            .set_x(computed)
            .update_negative(computed)
            .update_zero(computed);
    }

    fn ldy<R>(&mut self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let computed = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        self.registers
            .set_y(computed)
            .update_negative(computed)
            .update_zero(computed);
    }

    fn sta<W>(&self, opeland: Word, write: W)
        where W: Fn(Addr, Data)
    {
        write(opeland, self.registers.get(ByteRegister::A));
    }

    fn stx<W>(&self, opeland: Word, write: W)
        where W: Fn(Addr, Data)
    {
        write(opeland, self.registers.get(ByteRegister::X));
    }

    fn sty<W>(&self, opeland: Word, write: W)
        where W: Fn(Addr, Data)
    {
        write(opeland, self.registers.get(ByteRegister::Y));
    }

    fn txa(&mut self) {
        let x = self.registers.get(ByteRegister::X);
        self.registers
            .set_acc(x)
            .update_negative(x)
            .update_zero(x);
    }

    fn tya(&mut self) {
        let y = self.registers.get(ByteRegister::Y);
        self.registers
            .set_acc(y)
            .update_negative(y)
            .update_zero(y);
    }

    fn txs(&mut self) {
        let x = self.registers.get(ByteRegister::X);
        self.registers.set_sp(x);
    }

    fn tay(&mut self) {
        let acc = self.registers.get(ByteRegister::A);
        self.registers
            .set_y(acc)
            .update_negative(acc)
            .update_zero(acc);
    }

    fn tax(&mut self) {
        let acc = self.registers.get(ByteRegister::A);
        self.registers
            .set_x(acc)
            .update_negative(acc)
            .update_zero(acc);
    }

    fn tsx(&mut self) {
        let sp = self.registers.get(ByteRegister::SP);
        self.registers
            .set_x(sp)
            .update_negative(sp)
            .update_zero(sp);
    }

    fn php<W>(&mut self, ref write: W)
        where W: Fn(Addr, Data)
    {
        self.registers.set_break(true);
        self.push_status(&write);
    }

    fn plp<R>(&mut self, ref read: R)
        where R: Fn(Addr) -> Data
    {
        self.registers.set_reserved(true);
        let status = self.pop(&read);
        self.registers.set_p(status);
    }

    fn pha<W>(&mut self, ref write: W)
        where W: Fn(Addr, Data)
    {
        let acc = self.registers.get(ByteRegister::A);
        self.push(acc, &write);
    }

    fn pla<R>(&mut self, ref read: R)
        where R: Fn(Addr) -> Data
    {
        let v = self.pop(&read);
        self.registers
            .set_acc(v)
            .update_negative(v)
            .update_zero(v);
    }

    fn adc<R>(&mut self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        let computed = fetched + self.registers.get(ByteRegister::A) +
                       bool_to_u8(self.registers.get_status(StatusName::carry));
        self.registers
            .update_overflow(fetched, computed)
            .update_negative(computed)
            .update_zero(computed)
            .set_carry(computed > 0xFF as u8)
            .set_acc(computed);
    }

    fn sbc<R>(&mut self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        let computed = self.registers.get(ByteRegister::A) - fetched -
                       bool_to_u8(!self.registers.get_status(StatusName::carry));
        self.registers
            .update_overflow(computed, fetched)
            .update_negative(computed)
            .update_zero(computed)
            .set_carry(computed >= 0 as u8)
            .set_acc(computed);
    }


    fn cpx<R>(&mut self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        let computed = self.registers.get(ByteRegister::X) - fetched;
        self.registers
            .update_negative(computed)
            .update_zero(computed)
            .set_carry(computed >= 0 as u8);
    }


    /*


      case 'AND': {
        const data = mode === 'immediate' ? addrOrData : this.read(addrOrData);
        const operated = data & this.registers.A;
        this.registers.P.negative = !!(operated & 0x80);
        this.registers.P.zero = !operated;
        this.registers.A = operated & 0xFF;
        break;
      }
      case 'ASL': {
        if (mode === 'accumulator') {
          const acc = this.registers.A;
          this.registers.P.carry = !!(acc & 0x80);
          this.registers.A = (acc << 1) & 0xFF;
          this.registers.P.zero = !this.registers.A;
          this.registers.P.negative = !!(this.registers.A & 0x80);
        } else {
          const data = this.read(addrOrData);
          this.registers.P.carry = !!(data & 0x80);
          const shifted = (data << 1) & 0xFF;
          this.write(addrOrData, shifted);
          this.registers.P.zero = !shifted;
          this.registers.P.negative = !!(shifted & 0x80);
        }
        break;
      }
      case 'BIT': {
        const data = this.read(addrOrData);
        this.registers.P.negative = !!(data & 0x80);
        this.registers.P.overflow = !!(data & 0x40);
        this.registers.P.zero = !(this.registers.A & data);
        break;
      }
      case 'CMP': {
        const data = mode === 'immediate' ? addrOrData : this.read(addrOrData);
        const compared = this.registers.A - data;
        this.registers.P.carry = compared >= 0;
        this.registers.P.negative = !!(compared & 0x80);
        this.registers.P.zero = !(compared & 0xff);
        break;
      }
      case 'CPX': {
        const data = mode === 'immediate' ? addrOrData : this.read(addrOrData);
        const compared = this.registers.X - data;
        this.registers.P.carry = compared >= 0;
        this.registers.P.negative = !!(compared & 0x80);
        this.registers.P.zero = !(compared & 0xff);
        break;
      }
      case 'CPY': {
        const data = mode === 'immediate' ? addrOrData : this.read(addrOrData);
        const compared = this.registers.Y - data;
        this.registers.P.carry = compared >= 0;
        this.registers.P.negative = !!(compared & 0x80);
        this.registers.P.zero = !(compared & 0xff);
        break;
      }
      case 'DEC': {
        const data = (this.read(addrOrData) - 1) & 0xFF;
        this.registers.P.negative = !!(data & 0x80);
        this.registers.P.zero = !data;
        this.write(addrOrData, data);
        break;
      }
      case 'DEX': {
        this.registers.X = (this.registers.X - 1) & 0xFF;
        this.registers.P.negative = !!(this.registers.X & 0x80);
        this.registers.P.zero = !this.registers.X;
        break;
      }
      case 'DEY': {
        this.registers.Y = (this.registers.Y - 1) & 0xFF;
        this.registers.P.negative = !!(this.registers.Y & 0x80);
        this.registers.P.zero = !this.registers.Y;
        break;
      }
      case 'EOR': {
        const data = mode === 'immediate' ? addrOrData : this.read(addrOrData);
        const operated = data ^ this.registers.A;
        this.registers.P.negative = !!(operated & 0x80);
        this.registers.P.zero = !operated;
        this.registers.A = operated & 0xFF;
        break;
      }
      case 'INC': {
        const data = (this.read(addrOrData) + 1) & 0xFF;
        this.registers.P.negative = !!(data & 0x80);
        this.registers.P.zero = !data;
        this.write(addrOrData, data);
        break;
      }
      case 'INX': {
        this.registers.X = (this.registers.X + 1) & 0xFF;
        this.registers.P.negative = !!(this.registers.X & 0x80);
        this.registers.P.zero = !this.registers.X;
        break;
      }
      case 'INY': {
        this.registers.Y = (this.registers.Y + 1) & 0xFF;
        this.registers.P.negative = !!(this.registers.Y & 0x80);
        this.registers.P.zero = !this.registers.Y;
        break;
      }
      case 'LSR': {
        if (mode === 'accumulator') {
          const acc = this.registers.A & 0xFF;
          this.registers.P.carry = !!(acc & 0x01);
          this.registers.A = acc >> 1;
          this.registers.P.zero = !this.registers.A;
        } else {
          const data = this.read(addrOrData);
          this.registers.P.carry = !!(data & 0x01);
          this.registers.P.zero = !(data >> 1);
          this.write(addrOrData, data >> 1);
        }
        this.registers.P.negative = false;
        break;
      }
      case 'ORA': {
        const data = mode === 'immediate' ? addrOrData : this.read(addrOrData);
        const operated = data | this.registers.A;
        this.registers.P.negative = !!(operated & 0x80);
        this.registers.P.zero = !operated;
        this.registers.A = operated & 0xFF;
        break;
      }
      case 'ROL': {
        if (mode === 'accumulator') {
          const acc = this.registers.A;
          this.registers.A = (acc << 1) & 0xFF | (this.registers.P.carry ? 0x01 : 0x00);
          this.registers.P.carry = !!(acc & 0x80);
          this.registers.P.zero = !this.registers.A;
          this.registers.P.negative = !!(this.registers.A & 0x80);
        } else {
          const data = this.read(addrOrData);
          const writeData = (data << 1 | (this.registers.P.carry ? 0x01 : 0x00)) & 0xFF;
          this.write(addrOrData, writeData);
          this.registers.P.carry = !!(data & 0x80);
          this.registers.P.zero = !writeData;
          this.registers.P.negative = !!(writeData & 0x80);
        }
        break;
      }
      case 'ROR': {
        if (mode === 'accumulator') {
          const acc = this.registers.A;
          this.registers.A = acc >> 1 | (this.registers.P.carry ? 0x80 : 0x00);
          this.registers.P.carry = !!(acc & 0x01);
          this.registers.P.zero = !this.registers.A;
          this.registers.P.negative = !!(this.registers.A & 0x80);
        } else {
          const data = this.read(addrOrData);
          const writeData = data >> 1 | (this.registers.P.carry ? 0x80 : 0x00);
          this.write(addrOrData, writeData);
          this.registers.P.carry = !!(data & 0x01);
          this.registers.P.zero = !writeData;
          this.registers.P.negative = !!(writeData & 0x80);
        }
        break;
      }

      case 'PHA': {
        this.push(this.registers.A);
        break;
      }



      case 'JMP': {
        this.registers.PC = addrOrData;
        break;
      }
      case 'JSR': {
        const PC = this.registers.PC - 1;
        this.push((PC >> 8) & 0xFF);
        this.push(PC & 0xFF);
        this.registers.PC = addrOrData;
        break;
      }
      case 'RTS': {
        this.popPC();
        this.registers.PC++;
        break;
      }
      case 'RTI': {
        this.popStatus();
        this.popPC();
        this.registers.P.reserved = true;
        break;
      }
      case 'BCC': {
        if (!this.registers.P.carry) this.branch(addrOrData);
        break;
      }
      case 'BCS': {
        if (this.registers.P.carry) this.branch(addrOrData);
        break;
      }
      case 'BEQ': {
        if (this.registers.P.zero) this.branch(addrOrData);
        break;
      }
      case 'BMI': {
        if (this.registers.P.negative) this.branch(addrOrData);
        break;
      }
      case 'BNE': {
        if (!this.registers.P.zero) this.branch(addrOrData);
        break;
      }
      case 'BPL': {
        if (!this.registers.P.negative) this.branch(addrOrData);
        break;
      }
      case 'BVS': {
        if (this.registers.P.overflow) this.branch(addrOrData);
        break;
      }
      case 'BVC': {
        if (!this.registers.P.overflow) this.branch(addrOrData);
        break;
      }
      case 'CLD': {
        this.registers.P.decimal = false;
        break;
      }
      case 'CLC': {
        this.registers.P.carry = false;
        break;
      }
      case 'CLI': {
        this.registers.P.interrupt = false;
        break;
      }
      case 'CLV': {
        this.registers.P.overflow = false;
        break;
      }
      case 'SEC': {
        this.registers.P.carry = true;
        break;
      }
      case 'SEI': {
        this.registers.P.interrupt = true;
        break;
      }
      case 'SED': {
        this.registers.P.decimal = true;
        break;
      }
      case 'BRK': {
        const interrupt = this.registers.P.interrupt;
        this.registers.PC++;
        this.push((this.registers.PC >> 8) & 0xFF);
        this.push(this.registers.PC & 0xFF);
        this.registers.P.break = true;
        this.pushStatus();
        this.registers.P.interrupt = true;
        // Ignore interrupt when already set.
        if (!interrupt) {
          this.registers.PC = this.read(0xFFFE, "Word");
        }
        this.registers.PC--;
        break;
      }
      case 'NOP': {
        break;
      }
      // Unofficial Opecode
      case 'NOPD': {
        this.registers.PC++;
        break;
      }
      case 'NOPI': {
        this.registers.PC += 2;
        break;
      }
      case 'LAX': {
        this.registers.A = this.registers.X = this.read(addrOrData);
        this.registers.P.negative = !!(this.registers.A & 0x80);
        this.registers.P.zero = !this.registers.A;
        break;
      }
      case 'SAX': {
        const operated = this.registers.A & this.registers.X;
        this.write(addrOrData, operated);
        break;
      }
      case 'DCP': {
        const operated = (this.read(addrOrData) - 1) & 0xFF;
        this.registers.P.negative = !!(((this.registers.A - operated) & 0x1FF) & 0x80);
        this.registers.P.zero = !((this.registers.A - operated) & 0x1FF);
        this.write(addrOrData, operated);
        break;
      }
      case 'ISB': {
        const data = (this.read(addrOrData) + 1) & 0xFF;
        const operated = (~data & 0xFF) + this.registers.A + this.registers.P.carry;
        const overflow = (!(((this.registers.A ^ data) & 0x80) != 0) && (((this.registers.A ^ operated) & 0x80)) != 0);
        this.registers.P.overflow = overflow;
        this.registers.P.carry = operated > 0xFF;
        this.registers.P.negative = !!(operated & 0x80);
        this.registers.P.zero = !(operated & 0xFF);
        this.registers.A = operated & 0xFF;
        this.write(addrOrData, data);
        break;
      }
      case 'SLO': {
        let data = this.read(addrOrData);
        this.registers.P.carry = !!(data & 0x80);
        data = (data << 1) & 0xFF;
        this.registers.A |= data;
        this.registers.P.negative = !!(this.registers.A & 0x80);
        this.registers.P.zero = !(this.registers.A & 0xFF);
        this.write(addrOrData, data);
        break;
      }
      case 'RLA': {
        const data = (this.read(addrOrData) << 1) + this.registers.P.carry;
        this.registers.P.carry = !!(data & 0x100);
        this.registers.A = (data & this.registers.A) & 0xFF;
        this.registers.P.negative = !!(this.registers.A & 0x80);
        this.registers.P.zero = !(this.registers.A & 0xFF);
        this.write(addrOrData, data);
        break;
      }
      case 'SRE': {
        let data = this.read(addrOrData);
        this.registers.P.carry = !!(data & 0x01)
        data >>= 1;
        this.registers.A ^= data;
        this.registers.P.negative = !!(this.registers.A & 0x80);
        this.registers.P.zero = !(this.registers.A & 0xFF);
        this.write(addrOrData, data);
        break;
      }
      case 'RRA': {
        let data = this.read(addrOrData);
        const carry = !!(data & 0x01);
        data = (data >> 1) | (this.registers.P.carry ? 0x80 : 0x00);
        const operated = data + this.registers.A + carry;
        const overflow = (!(((this.registers.A ^ data) & 0x80) != 0) && (((this.registers.A ^ operated) & 0x80)) != 0);
        this.registers.P.overflow = overflow;
        this.registers.P.negative = !!(operated & 0x80);
        this.registers.P.zero = !(operated & 0xFF);
        this.registers.A = operated & 0xFF;
        this.registers.P.carry = operated > 0xFF;
        this.write(addrOrData, data);
        break;
      }
      */
}


#[test]
fn lda_immidiate() {
    let mut cpu = Cpu::new();
    cpu.registers.set_pc(0x0000);
    let rom = vec![0x00];
    let code = Opecode {
        name: Instruction::LDA,
        mode: Addressing::Immediate,
        cycle: 1, // dummy
    };
    cpu.lda(&code, 255, |addr: Addr| rom[addr as usize]);
    assert!(cpu.registers.get(ByteRegister::A) == 255);
}

#[test]
fn ldx_immidiate() {
    let mut cpu = Cpu::new();
    cpu.registers.set_pc(0x0000);
    let rom = vec![0x00];
    let code = Opecode {
        name: Instruction::LDX,
        mode: Addressing::Immediate,
        cycle: 1, // dummy
    };
    cpu.ldx(&code, 255, |addr: Addr| rom[addr as usize]);
    assert!(cpu.registers.get(ByteRegister::X) == 255);
}

#[test]
fn sta() {
    let mut cpu = Cpu::new();
    cpu.registers.set_pc(0x0000);
    cpu.registers.set_acc(0xA5);
    let mut mem = 0;
    let write = |addr: Addr, data: Data| {
        assert!(data == 0xA5);
        assert!(addr == 0xFF);
    };
    cpu.sta(0xFF, &write);
}

#[test]
fn stx() {
    let mut cpu = Cpu::new();
    cpu.registers.set_pc(0x0000);
    cpu.registers.set_x(0xA5);
    let mut mem = 0;
    let write = |addr: Addr, data: Data| {
        assert!(data == 0xA5);
        assert!(addr == 0xFF);
    };
    cpu.stx(0xFF, &write);
}

#[test]
fn sty() {
    let mut cpu = Cpu::new();
    cpu.registers.set_pc(0x0000);
    cpu.registers.set_y(0xA5);
    let mut mem = 0;
    let write = |addr: Addr, data: Data| {
        assert!(data == 0xA5);
        assert!(addr == 0xFF);
    };
    cpu.sty(0xFF, &write);
}

#[test]
fn tax() {
    let mut cpu = Cpu::new();
    cpu.registers.set_acc(0xA5);
    cpu.tax();
    assert!(cpu.registers.get(ByteRegister::X) == 0xA5);
}

#[test]
fn tay() {
    let mut cpu = Cpu::new();
    cpu.registers.set_acc(0xA5);
    cpu.tay();
    assert!(cpu.registers.get(ByteRegister::Y) == 0xA5);
}

#[test]
fn txa() {
    let mut cpu = Cpu::new();
    cpu.registers.set_x(0xA5);
    cpu.txa();
    assert!(cpu.registers.get(ByteRegister::A) == 0xA5);
}

#[test]
fn tya() {
    let mut cpu = Cpu::new();
    cpu.registers.set_y(0xA5);
    cpu.tya();
    assert!(cpu.registers.get(ByteRegister::A) == 0xA5);
}

#[test]
fn txs() {
    let mut cpu = Cpu::new();
    cpu.registers.set_x(0xA5);
    cpu.txs();
    assert!(cpu.registers.get(ByteRegister::SP) == 0xA5);
}

#[test]
fn tsx() {
    let mut cpu = Cpu::new();
    cpu.registers.set_sp(0xA5);
    cpu.tsx();
    assert!(cpu.registers.get(ByteRegister::X) == 0xA5);
}

#[test]
fn php() {
    let mut cpu = Cpu::new();
    cpu.registers.set_sp(0xA5);
    let mut mem = 0;
    let write = |addr: Addr, data: Data| {
        assert!(data == 0x34);
        assert!(addr == 0x01A5);
    };
    cpu.php(&write);
}

#[test]
fn plp() {
    let mut cpu = Cpu::new();
    cpu.registers.set_sp(0xA5);
    let read = |addr: Addr| {
        assert_eq!(addr, 0x01A6);
        0xA5 as u8
    };
    cpu.plp(&read);
    assert_eq!(cpu.registers.get(ByteRegister::P), 0xA5);
}

#[test]
fn pha() {
    let mut cpu = Cpu::new();
    cpu.registers.set_sp(0xA5);
    cpu.registers.set_acc(0x5A);
    let mut mem = 0;
    let write = |addr: Addr, data: Data| {
        assert!(data == 0x5A);
        assert!(addr == 0x01A5);
    };
    cpu.pha(&write);
}

#[test]
fn adc_immediate() {
    let mut cpu = Cpu::new();
    cpu.registers.set_pc(0x0000);
    cpu.registers.set_acc(0x05);
    let code = Opecode {
        name: Instruction::ADC,
        mode: Addressing::Immediate,
        cycle: 1, // dummy
    };
    cpu.adc(&code, 0xA5, |addr: Addr| 0 /* dummy */);
    assert!(cpu.registers.get(ByteRegister::A) == 0xAA);
}

#[test]
fn sbc_immediate() {
    let mut cpu = Cpu::new();
    cpu.registers.set_pc(0x0000);
    cpu.registers.set_acc(0x10);
    let code = Opecode {
        name: Instruction::SBC,
        mode: Addressing::Immediate,
        cycle: 1, // dummy
    };
    cpu.sbc(&code, 0x06, |addr: Addr| 0 /* dummy */);
    assert!(cpu.registers.get(ByteRegister::A) == 0x09);
}

#[test]
fn cpx_immediate() {
    let mut cpu = Cpu::new();
    cpu.registers.set_pc(0x0000);
    cpu.registers.set_x(0x05);
    let code = Opecode {
        name: Instruction::CPX,
        mode: Addressing::Immediate,
        cycle: 1, // dummy
    };
    cpu.cpx(&code, 0x04, |addr: Addr| 0 /* dummy */);
    assert!(cpu.registers.get_status(StatusName::carry));
}
