mod square;

use nes::types::{Data, Addr, Word};
use self::square::Square;

#[derive(Debug)]
pub struct Apu {
    squares: (Square, Square),
}


pub trait IApu {
    fn read(&self, addr: Addr) -> Data;

    fn write(&mut self, addr: Addr, data: Data);
}

extern "C" {
    fn test1(a: f32, b: u8, c: u8);
    fn test2();
}

impl Apu {
    pub fn new() -> Self {
        Apu { squares: (Square::new(0), Square::new(1)) }
    }

    pub fn read(&self, addr: Addr) -> Data {
        println!("apu read {:x}", addr);
        // if (addr === 0x15) {
        //   this.interrupts.deassertIrq();
        // }
        0
    }

    pub fn write(&mut self, addr: Addr, data: Data) {
        println!("apu write {:x} {:x}", addr, data);
        match addr {
            0x00...0x03 => {
                // unsafe {
                //     test1(10.111, 20, 30);
                // }
                // square wave control register
                self.square.0.write(addr, data);
            }
            _ => (),
        }
    }
}

/*
/* @flow */

import type { Byte } from '../types/common';
import Square from './square';
import Noise from './noise';
import Triangle from './triangle';
import Interrupts from '../interrupts';
import { DIVIDE_COUNT_FOR_240HZ } from '../constants/apu';

export default class Apu {

  registers: Uint8Array;
  cycle: number;
  step: number;
  envelopesCounter: number;
  square: Square[];
  triangle: Triangle;
  noise: Noise;
  sequencerMode: number;
  enableIrq: boolean;
  interrupts: Interrupts;

  constructor(interrupts: Interrupts) {
    this.interrupts = interrupts;
    // APU Registers
    // (0x4000 〜 0x4017)
    this.registers = new Uint8Array(0x18);
    this.cycle = 0;
    this.step = 0;
    this.square = [new Square(), new Square()];
    this.triangle = new Triangle();
    this.noise = new Noise();
    this.enableIrq = false;
  }

  run(cycle: number) {
    this.cycle += cycle;
    if (this.cycle >= DIVIDE_COUNT_FOR_240HZ) {
      // invoked by 240hz
      this.cycle -= DIVIDE_COUNT_FOR_240HZ;
      if (this.sequencerMode) {
        this.updateBySequenceMode1();
      } else {
        this.updateBySequenceMode0();
      }
    }
  }

  updateBySequenceMode0() {
    this.updateEnvelope();
    if (this.step % 2 === 1) {
      this.updateSweepAndLengthCounter();
    }
    this.step++;
    if (this.step === 4) {
      if (this.enableIrq) {
        this.interrupts.assertIrq();
      }
      this.step = 0;
    }
  }

  updateBySequenceMode1() {
    if (this.step % 2 === 0) {
      this.updateSweepAndLengthCounter();
    }
    this.step++;
    if (this.step === 5) {
      this.step = 0;
    } else {
      this.updateEnvelope();
    }
  }

  updateSweepAndLengthCounter() {
    this.square.forEach((s: Square): void => s.updateSweepAndLengthCounter());
    this.triangle.updateCounter();
    this.noise.updateCounter();
  }

  updateEnvelope() {
    this.square.forEach((s: Square): void => s.updateEnvelope());
    this.noise.updateEnvelope();
  }

  write(addr: Byte, data: Byte) {
    /* eslint-disable */
    // console.log('apu write', addr, data);
    // TODO: FIx perf
    if (addr <= 0x03) {
      // square wave control register
      this.square[0].write(addr, data);
    } else if (addr <= 0x07) {
      // square wave control register
      this.square[1].write(addr - 0x04, data);
    } else if (addr <= 0x0B) {
      // triangle
      this.triangle.write(addr - 0x08, data);
    } else if (addr <= 0x0F) {
      // noise
      this.noise.write(addr - 0x0C, data);
    } else if (addr === 0x17) {
      this.sequencerMode = data & 0x80 ? 1 : 0;
      this.registers[addr] = data;
      this.enableIrq = !!(data & 0x40);
    }
  }

  read(addr: Byte): Byte {
    // TODO: Implement other registers
    if (addr === 0x15) {
      this.interrupts.deassertIrq();
    }
    return 0;
  }

  close() {
    this.noise.close();
    this.square[0].close();
    this.square[1].close();
    this.triangle.close();
  }
}
*/