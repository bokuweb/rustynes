
import pulse from './pulse.js';

export default class Oscillator {

  constructor(type) {
    try {
      const AudioContext = window.AudioContext || window.webkitAudioContext
      this.context = new AudioContext();
    } catch (e) {
      throw new Error('Web Audio isn\'t supported in this browser!');
    }
    this.type = type || 'square';
    this.oscillator = this.createOscillator({ kind: this.type });

    this.waves = {
      '0.125': this.context.createPeriodicWave(pulse['0.125'].real, pulse['0.125'].imag),
      '0.25': this.context.createPeriodicWave(pulse['0.25'].real, pulse['0.25'].imag),
      '0.5': this.context.createPeriodicWave(pulse['0.5'].real, pulse['0.5'].imag),
      '0.75': this.context.createPeriodicWave(pulse['0.75'].real, pulse['0.75'].imag),
    };

    this.setVolume(0);
    this.setPulseWidth(0.5);
    this.playing = false;
  }

  start() {
    if (this.playing) {
      this.stop();
    }
    this.playing = true;
    this.oscillator.start(0);
  }

  stop() {
    if (this.playing) {
      this.setVolume(0);
      this.playing = false;
      this.oscillator.stop(this.context.currentTime);
      this.oscillator = this.createOscillator();
      this.setPulseWidth(0.5);
    }
  }

  close() {
    this.context.close();
  }

  createOscillator(options = {}) {
    const oscillator = this.context.createOscillator();
    if (options.kind) oscillator.type = options.kind;
    if (options.frequency) oscillator.frequency.value = options.frequency;
    if (options.harmonics) {
      const waveform = this.context.createPeriodicWave(
        new Float32Array(options.harmonics.real),
        new Float32Array(options.harmonics.imag)
      )
      oscillator.setPeriodicWave(waveform);
    }

    this.gain = this.context.createGain();
    this.gain.gain.value = 0.01;
    oscillator.connect(this.gain);
    this.gain.connect(this.context.destination);
    return oscillator;
  }

  setPulseWidth(pulseWidth) {
    this.oscillator.setPeriodicWave(this.waves[`${pulseWidth}`]);
  }

  setFrequency(frequency) {
    this.oscillator.frequency.value = frequency;
  }

  changeFrequency(frequency) {
    this.oscillator.frequency.setValueAtTime(frequency, this.context.currentTime)
  }

  setVolume(volume) {
    volume = Math.max(0, Math.min(1, volume));
    this.gain.gain.value = volume;
  }
}