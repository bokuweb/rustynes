export default class Noise {

  constructor() {
    try {
      const AudioContext = window.AudioContext || window.webkitAudioContext
      this.context = new AudioContext();
    } catch (e) {
      throw new Error('Web Audio isn\'t supported in this browser!');
    }
    this.createSource();
    this.playing = false;
  }

  createSource() {
    const node = this.context.createBufferSource();
    const buffer = this.context.createBuffer(1, this.context.sampleRate, this.context.sampleRate);
    const data = buffer.getChannelData(0);
    for (let i = 0; i < this.context.sampleRate; i++) {
      data[i] = Math.random();
    }
    node.buffer = buffer;
    node.loop = true;
    this.gain = this.context.createGain();
    this.gain.gain.value = 0.01;
    node.connect(this.gain);
    this.bandpass = this.context.createBiquadFilter();
    this.gain.connect(this.bandpass);
    this.bandpass.connect(this.context.destination);
    this.source = node;
    this.setVolume(0);
  }

  setVolume(volume) {
    volume = Math.max(0, Math.min(1, volume));
    this.gain.gain.value = volume;
  }

  setFrequency(frequency) {
    this.bandpass.frequency.value = frequency > 22050 ? 22050 : frequency;
  }

  start() {
    if (!this.playing) {
      this.playing = true;
      this.source.start(0);
    }
  }

  stop() {
    if (this.playing) {
      this.playing = false;
      this.source.stop();
      this.createSource();
    }
  }

  close() {
    this.context.close();
  }
}