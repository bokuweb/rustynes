
/*
/* @flow */

import Oscillator from './oscillator';
import { CPU_CLOCK } from '../constants/cpu';
import { counterTable, globalGain } from '../constants/apu';

import type { Byte } from '../types/common';

export default class Square {

  oscillator: Oscillator;
  sweepUnitCounter: number;
  lengthCounter: number;
  isLengthCounterEnable: boolean;
  sweepUnitDivider: number;
  frequency: number;
  sweepShiftAmount: number;
  isSweepEnabled: boolean;
  sweepMode: boolean;
  dividerForFrequency: number;
  envelopeLoopEnable: boolean;
  envelopeGeneratorCounter: number;
  envelopeRate: number;
  envelopeVolume: number;
  envelopeEnable: boolean;

  constructor() {
    this.reset();
    this.oscillator = new Oscillator();
    this.oscillator.setVolume(1)
    this.sweepUnitCounter = 0;
    this.envelopeGeneratorCounter = 0;
    this.envelopeRate = 0x0F;
    this.envelopeEnable = false;
  }

  get volume(): number {
    const vol = this.envelopeEnable ? this.envelopeVolume : this.envelopeRate;
    return vol / (0x0F / globalGain);
  }

  reset() {
    this.lengthCounter = 0;
    this.isLengthCounterEnable = false;
  }

  updateEnvelope() {
    if ((--this.envelopeGeneratorCounter) <= 0) {
      this.envelopeGeneratorCounter = this.envelopeRate;
      if (this.envelopeVolume > 0) {
        this.envelopeVolume--;
      } else {
        this.envelopeVolume = this.envelopeLoopEnable ? 0x0F : 0x00;
      }
    }
    this.oscillator.setVolume(this.volume);
  }

  // Length counter
  // When clocked by the frame counter, the length counter is decremented except when:
  // The length counter is 0, or The halt flag is set
  updateSweepAndLengthCounter() {
    if (this.isLengthCounterEnable && this.lengthCounter > 0) {
      this.lengthCounter--;
      if (this.lengthCounter === 0) {
        this.oscillator.stop();
      }
    }

    this.sweepUnitCounter++;
    if (!(this.sweepUnitCounter % this.sweepUnitDivider)) {
      // INFO: 
      // sweep mode 0 : newFreq = currentFreq - (currentFreq >> N)
      // sweep mode 1 : newFreq = currentFreq + (currentFreq >> N)
      if (this.isSweepEnabled) {
        const sign = this.sweepMode ? 1 : -1;
        this.frequency = this.frequency + ((this.frequency >> this.sweepShiftAmount) * sign);
        if (this.frequency > 4095) {
          this.frequency = 4095;
          this.oscillator.stop();
        } else if (this.frequency < 16) {
          this.frequency = 16;
          this.oscillator.stop();
        }
        this.oscillator.changeFrequency(this.frequency);
      }
    }
  }

  getPulseWidth(duty: number): number {
    switch (duty) {
      case 0x00: return 0.125;
      case 0x01: return 0.25;
      case 0x02: return 0.5;
      case 0x03: return 0.75;
      default: return 0;
    }
  }

  write(addr: Byte, data: Byte) {
    if (addr === 0x00) {
      this.envelopeEnable = !((data & 0x10) !== 0);
      this.envelopeRate = data & 0xF + 1;
      this.envelopeLoopEnable = ((data & 0x20) !== 0);
      const duty = (data >> 6) & 0x3;
      this.isLengthCounterEnable = !(data & 0x20);
      this.oscillator.setVolume(this.volume);
      this.oscillator.setPulseWidth(this.getPulseWidth(duty));

    }
    else if (addr === 0x01) {
      // Sweep
      this.isSweepEnabled = !!(data & 0x80);
      this.sweepUnitDivider = ((data >> 4) & 0x07) + 1;
      this.sweepMode = !!(data & 0x08);
      this.sweepShiftAmount = data & 0x07;
    }
    else if (addr === 0x02) {
      this.dividerForFrequency &= 0x700;
      this.dividerForFrequency |= data;
    }
    else if (addr === 0x03) {
      // Programmable timer, length counter
      this.dividerForFrequency &= 0xFF;
      this.dividerForFrequency |= ((data & 0x7) << 8);
      if (this.isLengthCounterEnable) {
        this.lengthCounter = counterTable[data & 0xF8];
      }
      this.frequency = CPU_CLOCK / ((this.dividerForFrequency + 1) * 32);
      this.sweepUnitCounter = 0;
      // envelope
      this.envelopeGeneratorCounter = this.envelopeRate;
      this.envelopeVolume = 0x0F;
      this.start();
    }
  }

  start() {
    this.oscillator.start();
    this.oscillator.setFrequency(this.frequency);
  }

  close() {
    this.oscillator.close();
  }  
}
*/