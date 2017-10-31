use super::super::cpu_registers::{CpuRegisters, Register};
use super::super::bus::cpu_bus::CpuBus;
use super::super::types::{Data, Addr, Word};

pub fn sta<T: Register>(opeland: Word, registers: &mut T, ref mut bus: &mut CpuBus) {
    bus.write(opeland, registers.get_A());
}

/*
    fn branch(&self, addr: Addr) {
        self.registers.borrow_mut().set_pc(addr);
    }

    fn push_status<W>(&self, write: W)
        where W: Fn(Addr, Data)
    {
        let status = self.registers.borrow().get(ByteRegister::P);
        self.push(status, &write);
    }

    fn push_pc<W>(&self, write: W)
        where W: Fn(Addr, Data)
    {
        let pc = self.registers.borrow().get_pc();
        self.push((pc >> 8) as u8, &write);
        self.push(pc as u8, &write);
    }

    fn push<W>(&self, data: Data, write: W)
        where W: Fn(Addr, Data)
    {
        let addr = self.registers.borrow().get(ByteRegister::SP) as Addr;
        write((addr | 0x0100), data);
        self.registers.borrow_mut().dec_sp();
    }

    fn pop<R>(&self, read: R) -> Data
        where R: Fn(Addr) -> Data
    {
        self.registers.borrow_mut().inc_sp();
        let addr = 0x0100 | self.registers.borrow().get(ByteRegister::SP) as Addr;
        read(addr)
    }

    fn pop_pc<R>(&self, read: R)
        where R: Fn(Addr) -> Data
    {
        let lower = self.pop(&read) as u16;
        let upper = self.pop(&read) as u16;
        self.registers.borrow_mut().set_pc(upper << 8 | lower);
    }

    fn pop_status<R>(&self, read: R)
        where R: Fn(Addr) -> Data
    {
        let status = self.pop(&read) as u8;
        self.registers.borrow_mut().set_p(status);
    }

    fn lda<R>(&self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let computed = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        self.registers
            .borrow_mut()
            .set_acc(computed)
            .update_negative(computed)
            .update_zero(computed);
    }

    fn ldx<R>(&self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let computed = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        self.registers
            .borrow_mut()
            .set_x(computed)
            .update_negative(computed)
            .update_zero(computed);
    }

    fn ldy<R>(&self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let computed = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        self.registers
            .borrow_mut()
            .set_y(computed)
            .update_negative(computed)
            .update_zero(computed);
    }

    fn stx<W>(&self, opeland: Word, write: W)
        where W: Fn(Addr, Data)
    {
        write(opeland, self.registers.borrow().get(ByteRegister::X));
    }

    fn sty<W>(&self, opeland: Word, write: W)
        where W: Fn(Addr, Data)
    {
        write(opeland, self.registers.borrow().get(ByteRegister::Y));
    }

    fn txa(&self) {
        let x = self.registers.borrow().get(ByteRegister::X);
        self.registers
            .borrow_mut()
            .set_acc(x)
            .update_negative(x)
            .update_zero(x);
    }

    fn tya(&self) {
        let y = self.registers.borrow().get(ByteRegister::Y);
        self.registers
            .borrow_mut()
            .set_acc(y)
            .update_negative(y)
            .update_zero(y);
    }

    fn txs(&self) {
        let x = self.registers.borrow().get(ByteRegister::X);
        self.registers.borrow_mut().set_sp(x);
    }

    fn tay(&self) {
        let acc = self.registers.borrow().get(ByteRegister::A);
        self.registers
            .borrow_mut()
            .set_y(acc)
            .update_negative(acc)
            .update_zero(acc);
    }

    fn tax(&self) {
        let acc = self.registers.borrow().get(ByteRegister::A);
        self.registers
            .borrow_mut()
            .set_x(acc)
            .update_negative(acc)
            .update_zero(acc);
    }

    fn tsx(&self) {
        let sp = self.registers.borrow().get(ByteRegister::SP);
        self.registers
            .borrow_mut()
            .set_x(sp)
            .update_negative(sp)
            .update_zero(sp);
    }

    fn php<W>(&self, ref write: W)
        where W: Fn(Addr, Data)
    {
        self.registers.borrow_mut().set_break(true);
        self.push_status(&write);
    }

    fn plp<R>(&self, ref read: R)
        where R: Fn(Addr) -> Data
    {
        self.registers.borrow_mut().set_reserved(true);
        let status = self.pop(&read);
        self.registers.borrow_mut().set_p(status);
    }

    fn pha<W>(&self, ref write: W)
        where W: Fn(Addr, Data)
    {
        let acc = self.registers.borrow().get(ByteRegister::A);
        self.push(acc, &write);
    }

    fn pla<R>(&self, ref read: R)
        where R: Fn(Addr) -> Data
    {
        let v = self.pop(&read);
        self.registers
            .borrow_mut()
            .set_acc(v)
            .update_negative(v)
            .update_zero(v);
    }

    fn adc<R>(&self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        let computed = fetched + self.registers.borrow().get(ByteRegister::A) +
                       bool_to_u8(self.registers.borrow().get_status(StatusName::carry));
        self.registers
            .borrow_mut()
            .update_overflow(fetched, computed)
            .update_negative(computed)
            .update_zero(computed)
            .set_carry(computed > 0xFF as u8)
            .set_acc(computed);
    }

    fn sbc<R>(&self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        let computed = self.registers.borrow().get(ByteRegister::A) - fetched -
                       bool_to_u8(!self.registers.borrow().get_status(StatusName::carry));
        self.registers
            .borrow_mut()
            .update_overflow(computed, fetched)
            .update_negative(computed)
            .update_zero(computed)
            .set_carry(computed >= 0 as u8)
            .set_acc(computed);
    }

    fn cpx<R>(&self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        let computed = self.registers.borrow().get(ByteRegister::X) - fetched;
        self.registers
            .borrow_mut()
            .update_negative(computed)
            .update_zero(computed)
            .set_carry(computed >= 0 as u8);
    }

    fn cpy<R>(&self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        let computed = self.registers.borrow().get(ByteRegister::Y) - fetched;
        self.registers
            .borrow_mut()
            .update_negative(computed)
            .update_zero(computed)
            .set_carry(computed >= 0 as u8);
    }

    fn cmp<R>(&self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        let computed = self.registers.borrow().get(ByteRegister::A) - fetched;
        self.registers
            .borrow_mut()
            .update_negative(computed)
            .update_zero(computed)
            .set_carry(computed >= 0 as u8);
    }

    fn and<R>(&self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        let computed = self.registers.borrow().get(ByteRegister::A) & fetched;
        self.registers
            .borrow_mut()
            .update_negative(computed)
            .update_zero(computed)
            .set_acc(computed);
    }

    fn eor<R>(&self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        let computed = self.registers.borrow().get(ByteRegister::A) ^ fetched;
        self.registers
            .borrow_mut()
            .update_negative(computed)
            .update_zero(computed)
            .set_acc(computed);
    }

    fn ora<R>(&self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        let computed = self.registers.borrow().get(ByteRegister::A) | fetched;
        self.registers
            .borrow_mut()
            .update_negative(computed)
            .update_zero(computed)
            .set_acc(computed);
    }

    fn bit<R>(&self, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let fetched = read(opeland);
        self.registers
            .borrow_mut()
            .update_negative(fetched)
            .update_zero(fetched & self.registers.borrow().get(ByteRegister::A))
            .set_overflow(fetched & 0x40 == 0x40);
    }

    fn asl<R, W>(&self, code: &Opecode, opeland: Word, read: R, write: W)
        where R: Fn(Addr) -> Data,
              W: Fn(Addr, Data)
    {
        match code.mode {
            Addressing::Accumulator => {
                let acc = self.registers.borrow().get(ByteRegister::A);
                let shifted = (acc << 1) as u8;
                self.registers
                    .borrow_mut()
                    .set_carry(acc & 0x80 == 0x80)
                    .update_negative(shifted)
                    .update_zero(shifted)
                    .set_acc(shifted);
            }
            _ => {
                let fetched = read(opeland);
                let shifted = (fetched << 1) as u8;
                self.registers
                    .borrow_mut()
                    .set_carry(fetched & 0x80 == 0x80)
                    .update_negative(shifted)
                    .update_zero(shifted);
                write(opeland, shifted);
            }
        };
    }

    fn lsr<R, W>(&self, code: &Opecode, opeland: Word, read: R, write: W)
        where R: Fn(Addr) -> Data,
              W: Fn(Addr, Data)
    {
        match code.mode {
            Addressing::Accumulator => {
                let acc = self.registers.borrow().get(ByteRegister::A);
                let shifted = (acc >> 1) as u8;
                self.registers
                    .borrow_mut()
                    .set_carry(acc & 0x01 == 0x01)
                    .update_negative(shifted)
                    .update_zero(shifted)
                    .set_acc(shifted);
            }
            _ => {
                let fetched = read(opeland);
                let shifted = (fetched >> 1) as u8;
                self.registers
                    .borrow_mut()
                    .set_carry(fetched & 0x01 == 0x01)
                    .update_negative(shifted)
                    .update_zero(shifted);
                write(opeland, shifted);
            }
        };
    }

    fn ror<R, W>(&self, code: &Opecode, opeland: Word, read: R, write: W)
        where R: Fn(Addr) -> Data,
              W: Fn(Addr, Data)
    {
        let shift = |v: u8| {
            ((v >> 1) |
             if self.registers.borrow().get_status(StatusName::carry) {
                 0x80
             } else {
                 0x00
             }) as u8
        };
        match code.mode {
            Addressing::Accumulator => {
                let acc = self.registers.borrow().get(ByteRegister::A);
                let shifted = shift(acc);
                self.registers
                    .borrow_mut()
                    .set_carry(acc & 0x01 == 0x01)
                    .update_negative(shifted)
                    .update_zero(shifted)
                    .set_acc(shifted);
            }
            _ => {
                let fetched = read(opeland);
                let shifted = shift(fetched);
                self.registers
                    .borrow_mut()
                    .set_carry(fetched & 0x01 == 0x01)
                    .update_negative(shifted)
                    .update_zero(shifted);
                write(opeland, shifted);
            }
        };
    }

    fn rol<R, W>(&self, code: &Opecode, opeland: Word, read: R, write: W)
        where R: Fn(Addr) -> Data,
              W: Fn(Addr, Data)
    {
        let shift = |v: u8| {
            ((v << 1) |
             if self.registers.borrow().get_status(StatusName::carry) {
                 0x01
             } else {
                 0x00
             }) as u8
        };
        match code.mode {
            Addressing::Accumulator => {
                let acc = self.registers.borrow().get(ByteRegister::A);
                let shifted = shift(acc);
                self.registers
                    .borrow_mut()
                    .set_carry(acc & 0x01 == 0x01)
                    .update_negative(shifted)
                    .update_zero(shifted)
                    .set_acc(shifted);
            }
            _ => {
                let fetched = read(opeland);
                let shifted = shift(fetched);
                self.registers
                    .borrow_mut()
                    .set_carry(fetched & 0x01 == 0x01)
                    .update_negative(shifted)
                    .update_zero(shifted);
                write(opeland, shifted);
            }
        };
    }

    fn inx(&self) {
        let x = self.registers.borrow().get(ByteRegister::X) + 1;
        self.registers
            .borrow_mut()
            .set_x(x)
            .update_negative(x)
            .update_zero(x);
    }

    fn iny(&self) {
        let y = self.registers.borrow().get(ByteRegister::Y) + 1;
        self.registers
            .borrow_mut()
            .set_y(y)
            .update_negative(y)
            .update_zero(y);
    }

    fn inc<R, W>(&self, opeland: Word, read: R, write: W)
        where R: Fn(Addr) -> Data,
              W: Fn(Addr, Data)
    {
        let data = read(opeland) + 1 as u8;
        self.registers
            .borrow_mut()
            .update_negative(data)
            .update_zero(data);
        write(opeland, data);
    }

    fn dex(&self) {
        let x = self.registers.borrow().get(ByteRegister::X) - 1;
        self.registers
            .borrow_mut()
            .set_x(x)
            .update_negative(x)
            .update_zero(x);
    }

    fn dey(&self) {
        let y = self.registers.borrow().get(ByteRegister::Y) - 1;
        self.registers
            .borrow_mut()
            .set_y(y)
            .update_negative(y)
            .update_zero(y);
    }

    fn dec<R, W>(&self, opeland: Word, read: R, write: W)
        where R: Fn(Addr) -> Data,
              W: Fn(Addr, Data)
    {
        let data = read(opeland) - 1 as u8;
        self.registers
            .borrow_mut()
            .update_negative(data)
            .update_zero(data);
        write(opeland, data);
    }

    fn clc(&self) {
        self.registers.borrow_mut().set_carry(false);
    }

    fn cli(&self) {
        self.registers.borrow_mut().set_interrupt(false);
    }

    fn clv(&self) {
        self.registers.borrow_mut().set_overflow(false);
    }

    fn sec(&self) {
        self.registers.borrow_mut().set_carry(true);
    }

    fn sei(&self) {
        self.registers.borrow_mut().set_interrupt(true);
    }

    fn brk<R, W>(&self, read: R, write: W)
        where R: Fn(Addr) -> Data,
              W: Fn(Addr, Data)
    {
        let interrupt = self.registers
            .borrow()
            .get_status(StatusName::interrupt);
        self.registers.borrow_mut().inc_pc();
        self.push_pc(&write);
        self.registers.borrow_mut().set_break(true);
        self.push_status(&write);
        self.registers.borrow_mut().set_interrupt(true);
        // Ignore interrupt when already set.
        if !interrupt {
            let fetched = self.read_word(&read, 0xFFFE);
            self.registers.borrow_mut().set_pc(fetched);
        }
        self.registers.borrow_mut().dec_pc();
    }

    fn jsr<W>(&self, opeland: Word, write: W)
        where W: Fn(Addr, Data)
    {
        let pc = self.registers.borrow().get_pc() - 1;
        self.push((pc >> 8) as u8, &write);
        self.push(pc as u8, &write);
        self.registers.borrow_mut().set_pc(opeland);
    }

    fn jmp(&self, opeland: Word) {
        self.registers.borrow_mut().set_pc(opeland);
    }

    fn rti<R>(&self, read: R)
        where R: Fn(Addr) -> Data
    {
        self.pop_status(&read);
        self.pop_pc(&read);
        self.registers.borrow_mut().set_reserved(true);
    }

    fn rts<R>(&self, read: R)
        where R: Fn(Addr) -> Data
    {
        self.pop_pc(&read);
        self.registers.borrow_mut().inc_pc();
    }

    fn bcc(&self, opeland: Word) {
        if !self.registers.borrow().get_status(StatusName::carry) {
            self.branch(opeland)
        }
    }

    fn bcs(&self, opeland: Word) {
        if self.registers.borrow().get_status(StatusName::carry) {
            self.branch(opeland)
        }
    }

    fn beq(&self, opeland: Word) {
        if self.registers.borrow().get_status(StatusName::zero) {
            self.branch(opeland)
        }
    }

    fn bmi(&self, opeland: Word) {
        if self.registers.borrow().get_status(StatusName::negative) {
            self.branch(opeland)
        }
    }

    fn bne(&self, opeland: Word) {
        if !self.registers.borrow().get_status(StatusName::zero) {
            self.branch(opeland)
        }
    }

    fn bpl(&self, opeland: Word) {
        if !self.registers.borrow().get_status(StatusName::negative) {
            self.branch(opeland)
        }
    }

    fn bvs(&self, opeland: Word) {
        if self.registers.borrow().get_status(StatusName::overflow) {
            self.branch(opeland)
        }
    }

    fn bvc(&self, opeland: Word) {
        if !self.registers.borrow().get_status(StatusName::overflow) {
            self.branch(opeland)
        }
    }

    fn cld(&self) {
        self.registers.borrow_mut().set_decimal(true);
    }

    fn sed(&self) {
        self.registers.borrow_mut().set_decimal(true);
    }
    */




#[test]
fn lda_immidiate() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    let rom = vec![0x00];
    let code = Opecode {
        name: Instruction::LDA,
        mode: Addressing::Immediate,
        cycle: 1, // mock
    };
    cpu.lda(&code, 255, |addr: Addr| rom[addr as usize]);
    assert!(cpu.registers.borrow().get(ByteRegister::A) == 255);
}

#[test]
fn ldx_immidiate() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    let rom = vec![0x00];
    let code = Opecode {
        name: Instruction::LDX,
        mode: Addressing::Immediate,
        cycle: 1, // mock
    };
    cpu.ldx(&code, 255, |addr: Addr| rom[addr as usize]);
    assert!(cpu.registers.borrow().get(ByteRegister::X) == 255);
}

#[test]
fn sta() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_acc(0xA5);
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
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_x(0xA5);
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
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_y(0xA5);
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
    cpu.registers.borrow_mut().set_acc(0xA5);
    cpu.tax();
    assert!(cpu.registers.borrow().get(ByteRegister::X) == 0xA5);
}

#[test]
fn tay() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_acc(0xA5);
    cpu.tay();
    assert!(cpu.registers.borrow().get(ByteRegister::Y) == 0xA5);
}

#[test]
fn txa() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_x(0xA5);
    cpu.txa();
    assert!(cpu.registers.borrow().get(ByteRegister::A) == 0xA5);
}

#[test]
fn tya() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_y(0xA5);
    cpu.tya();
    assert!(cpu.registers.borrow().get(ByteRegister::A) == 0xA5);
}

#[test]
fn txs() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_x(0xA5);
    cpu.txs();
    assert!(cpu.registers.borrow().get(ByteRegister::SP) == 0xA5);
}

#[test]
fn tsx() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_sp(0xA5);
    cpu.tsx();
    assert!(cpu.registers.borrow().get(ByteRegister::X) == 0xA5);
}

#[test]
fn php() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_sp(0xA5);
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
    cpu.registers.borrow_mut().set_sp(0xA5);
    let read = |addr: Addr| {
        assert_eq!(addr, 0x01A6);
        0xA5 as u8
    };
    cpu.plp(&read);
    assert_eq!(cpu.registers.borrow().get(ByteRegister::P), 0xA5);
}

#[test]
fn pha() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_sp(0xA5);
    cpu.registers.borrow_mut().set_acc(0x5A);
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
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_acc(0x05);
    let code = Opecode {
        name: Instruction::ADC,
        mode: Addressing::Immediate,
        cycle: 1, // mock
    };
    cpu.adc(&code, 0xA5, |addr: Addr| 0 /* mock */);
    assert!(cpu.registers.borrow().get(ByteRegister::A) == 0xAA);
}

#[test]
fn sbc_immediate() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_acc(0x10);
    let code = Opecode {
        name: Instruction::SBC,
        mode: Addressing::Immediate,
        cycle: 1, // mock
    };
    cpu.sbc(&code, 0x06, |addr: Addr| 0 /* mock */);
    assert!(cpu.registers.borrow().get(ByteRegister::A) == 0x09);
}

#[test]
fn cpx_immediate() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_x(0x05);
    let code = Opecode {
        name: Instruction::CPX,
        mode: Addressing::Immediate,
        cycle: 1, // mock
    };
    cpu.cpx(&code, 0x04, |addr: Addr| 0 /* mock */);
    assert!(cpu.registers.borrow().get_status(StatusName::carry));
}

#[test]
fn cpy_immediate() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_y(0x05);
    let code = Opecode {
        name: Instruction::CPY,
        mode: Addressing::Immediate,
        cycle: 1, // mock
    };
    cpu.cpy(&code, 0x04, |addr: Addr| 0 /* mock */);
    assert!(cpu.registers.borrow().get_status(StatusName::carry));
}

#[test]
fn cmp_immediate() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_acc(0x05);
    let code = Opecode {
        name: Instruction::CMP,
        mode: Addressing::Immediate,
        cycle: 1, // mock
    };
    cpu.cmp(&code, 0x04, |addr: Addr| 0 /* mock */);
    assert!(cpu.registers.borrow().get_status(StatusName::carry));
}

#[test]
fn and_immediate() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_acc(0xA5);
    let code = Opecode {
        name: Instruction::AND,
        mode: Addressing::Immediate,
        cycle: 1, // mock
    };
    cpu.and(&code, 0x05, |addr: Addr| 0 /* mock */);
    assert_eq!(cpu.registers.borrow().get(ByteRegister::A), 0x05);
}

#[test]
fn eor_immediate() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_acc(0xA5);
    let code = Opecode {
        name: Instruction::EOR,
        mode: Addressing::Immediate,
        cycle: 1, // mock
    };
    cpu.eor(&code, 0x05, |addr: Addr| 0 /* mock */);
    assert_eq!(cpu.registers.borrow().get(ByteRegister::A), 0xA0);
}

#[test]
fn ora_immediate() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_acc(0xA0);
    let code = Opecode {
        name: Instruction::ORA,
        mode: Addressing::Immediate,
        cycle: 1, // mock
    };
    cpu.ora(&code, 0x05, |addr: Addr| 0 /* mock */);
    assert_eq!(cpu.registers.borrow().get(ByteRegister::A), 0xA5);
}

#[test]
fn asl_accumlator() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_acc(0x55);
    let code = Opecode {
        name: Instruction::ASL,
        mode: Addressing::Accumulator,
        cycle: 1, // mock
    };
    cpu.asl(&code,
            0x00,
            |addr: Addr| 0,
            |addr: Addr, data: Data| () /* mock */);
    assert_eq!(cpu.registers.borrow().get(ByteRegister::A), 0xAA);
}

#[test]
fn lsr_accumlator() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_acc(0x55);
    let code = Opecode {
        name: Instruction::LSR,
        mode: Addressing::Accumulator,
        cycle: 1, // mock
    };
    cpu.lsr(&code,
            0x00,
            |addr: Addr| 0,
            |addr: Addr, data: Data| () /* mock */);
    assert_eq!(cpu.registers.borrow().get(ByteRegister::A), 0x2A);
}

#[test]
fn ror_accumlator_with_carry() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    cpu.registers.borrow_mut().set_acc(0x55);
    cpu.registers.borrow_mut().set_carry(true);
    let code = Opecode {
        name: Instruction::ROR,
        mode: Addressing::Accumulator,
        cycle: 1, // mock
    };
    cpu.ror(&code,
            0x00,
            |addr: Addr| 0,
            |addr: Addr, data: Data| () /* mock */);
    assert_eq!(cpu.registers.borrow().get(ByteRegister::A), 0xAA);
}

#[test]
fn ror_accumlator_without_carry() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000).set_acc(0x55);
    let code = Opecode {
        name: Instruction::ROR,
        mode: Addressing::Accumulator,
        cycle: 1, // mock
    };
    cpu.ror(&code,
            0x00,
            |addr: Addr| 0,
            |addr: Addr, data: Data| () /* mock */);
    assert_eq!(cpu.registers.borrow().get(ByteRegister::A), 0x2A);
}


#[test]
fn rol_accumlator_with_carry() {
    let mut cpu = Cpu::new();
    cpu.registers
        .borrow_mut()
        .set_pc(0x0000)
        .set_acc(0x55)
        .set_carry(true);
    let code = Opecode {
        name: Instruction::ROL,
        mode: Addressing::Accumulator,
        cycle: 1, // mock
    };
    cpu.rol(&code,
            0x00,
            |addr: Addr| 0,
            |addr: Addr, data: Data| () /* mock */);
    assert_eq!(cpu.registers.borrow().get(ByteRegister::A), 0xAB);
}

#[test]
fn inx() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000).set_x(0xA5);
    let code = Opecode {
        name: Instruction::INX,
        mode: Addressing::Implied,
        cycle: 1, // mock
    };
    cpu.inx();
    assert_eq!(cpu.registers.borrow().get(ByteRegister::X), 0xA6);
}

#[test]
fn iny() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000).set_y(0xA5);
    let code = Opecode {
        name: Instruction::INY,
        mode: Addressing::Implied,
        cycle: 1, // mock
    };
    cpu.iny();
    assert_eq!(cpu.registers.borrow().get(ByteRegister::Y), 0xA6);
}

#[test]
fn inc() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    let code = Opecode {
        name: Instruction::INC,
        mode: Addressing::Implied,
        cycle: 1, // mock
    };
    cpu.inc(0x55, |addr: Addr| 0xA5, |addr: Addr, data: Data| {
        assert_eq!(addr, 0x55);
        assert_eq!(data, 0xA6);
    });
}

#[test]
fn dex() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000).set_x(0xA5);
    let code = Opecode {
        name: Instruction::DEX,
        mode: Addressing::Implied,
        cycle: 1, // mock
    };
    cpu.dex();
    assert_eq!(cpu.registers.borrow().get(ByteRegister::X), 0xA4);
}

#[test]
fn dey() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000).set_y(0xA5);
    let code = Opecode {
        name: Instruction::DEY,
        mode: Addressing::Implied,
        cycle: 1, // mock
    };
    cpu.dey();
    assert_eq!(cpu.registers.borrow().get(ByteRegister::Y), 0xA4);
}

#[test]
fn dec() {
    let mut cpu = Cpu::new();
    cpu.registers.borrow_mut().set_pc(0x0000);
    let code = Opecode {
        name: Instruction::DEC,
        mode: Addressing::Implied,
        cycle: 1, // mock
    };
    cpu.dec(0x55, |addr: Addr| 0xA5, |addr: Addr, data: Data| {
        assert_eq!(addr, 0x55);
        assert_eq!(data, 0xA4);
    });
}
