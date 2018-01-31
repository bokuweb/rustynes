mod square;
mod triangle;
mod constants;

use nes::types::{Data, Addr, Word};
use self::square::Square;
use self::triangle::Triangle;
use self::constants::*;


#[derive(Debug)]
pub struct Apu {
    squares: (Square, Square),
    triangle: Triangle,
    cycle: u16,
    step: usize,
    sequencer_mode: bool,
}

impl Apu {
    pub fn run(&mut self, cycle: u16) {
        self.cycle += cycle;
        if (self.cycle >= DIVIDE_COUNT_FOR_240HZ) {
            // invoked by 240hz
            self.cycle -= DIVIDE_COUNT_FOR_240HZ;
            if self.sequencer_mode {
                self.update_by_sequence_mode1();
            } else {
                self.update_by_sequence_mode0();
            }
        }
    }

    pub fn read(&mut self, addr: Addr) -> Data {
        // println!("apu read {:x}", addr);
        // if (addr === 0x15) {
        //   self.interrupts.deassertIrq();
        // }
        0
    }

    pub fn write(&mut self, addr: Addr, data: Data) {
        // println!("apu write {:x} {:x}", addr, data);
        match addr {
            0x00...0x03 => {
                self.squares.0.write(addr, data);
            }
            0x04...0x07 => {
                self.squares.1.write(addr - 0x04, data);
            }   
            0x08...0x0c => {
                // triangle
                self.triangle.write(addr - 0x08, data);
            }         
            _ => (),
        }
    }

    pub fn new() -> Self {
        Apu {
            squares: (Square::new(0), Square::new(1)),
            triangle: Triangle::new(2),
            cycle: 0,
            step: 0,
            sequencer_mode: false,
        }
    }

    fn update_by_sequence_mode0(&mut self) {
        self.update_envelope();
        if self.step % 2 == 1 {
            self.update_counters();
        }
        self.step += 1;
        if self.step == 4 {
            // if self.enable_irq {
            //self.interrupts.assert_irq();
            // }
            self.step = 0;
        }
    }

    fn update_by_sequence_mode1(&mut self) {
        if self.step % 2 == 0 {
            self.update_counters();
        }
        self.step += 1;
        if (self.step == 5) {
            self.step = 0;
        } else {
            self.update_envelope();
        }
    }

    fn update_counters(&mut self) {
        self.squares.0.update_counters();
        self.squares.1.update_counters();
        // self.triangle.updateCounter();
        // self.noise.updateCounter();
    }

    fn update_envelope(&mut self) {
        self.squares.0.update_envelope();
        self.squares.1.update_envelope();
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
    self.interrupts = interrupts;
    // APU Registers
    // (0x4000 〜 0x4017)
    self.registers = new Uint8Array(0x18);
    self.cycle = 0;
    self.step = 0;
    self.square = [new Square(), new Square()];
    self.triangle = new Triangle();
    self.noise = new Noise();
    self.enableIrq = false;
  }






  write(addr: Byte, data: Byte) {
    /* eslint-disable */
    // console.log('apu write', addr, data);
    // TODO: FIx perf
    if (addr <= 0x03) {
      // square wave control register
      self.square[0].write(addr, data);
    } else if (addr <= 0x07) {
      // square wave control register
      self.square[1].write(addr - 0x04, data);
    } else if (addr <= 0x0B) {
      // triangle
      self.triangle.write(addr - 0x08, data);
    } else if (addr <= 0x0F) {
      // noise
      self.noise.write(addr - 0x0C, data);
    } else if (addr === 0x17) {
      self.sequencerMode = data & 0x80 ? 1 : 0;
      self.registers[addr] = data;
      self.enableIrq = !!(data & 0x40);
    }
  }

  close() {
    self.noise.close();
    self.square[0].close();
    self.square[1].close();
    self.triangle.close();
  }
}
*/