mergeInto(LibraryManager.library, {
  canvas_render: function (ptr, len) {
    Module.NES.buf = new Uint8Array(Module.HEAPU8.buffer, ptr, len);
    Module.NES.image.data.set(Module.NES.buf);
    Module.NES.ctx.putImageData(Module.NES.image, 0, 0);
  },
  // test1: function (a, b, c) {
  //   Module.NES.test1(a, b, c);
  // },
  start_oscillator: function (index) {
    console.log('start oscillator', index);
    Module.NES.oscs[index].start();
  },
  stop_oscillator: function (index) {
    console.log('stop oscillator', index);
    Module.NES.oscs[index].stop();
  },
  close_oscillator: function (index) {
    console.log('close oscillator', index);
    Module.NES.oscs[index].close();
  },
  set_oscillator_frequency: function (index, freq) {
    console.log('set oscillator frequency', index, freq);
    Module.NES.oscs[index].setFrequency(freq);
  },
  change_oscillator_frequency: function (index, freq) {
    console.log('change oscillator frequency', index, freq);
    Module.NES.oscs[index].changeFrequency(freq);
  },
  set_oscillator_volume: function (index, volume) {
    console.log('set oscillator volume', index, volume);
    Module.NES.oscs[index].setVolume(volume);
  },
  set_oscillator_pulse_width: function (index, width) {
    console.log('set oscillator pulse width', index, width);
    Module.NES.oscs[index].setPulseWidth(width);
  }
});
