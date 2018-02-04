mergeInto(LibraryManager.library, {
  canvas_render: function (ptr, len) {
    Module.NES.buf = new Uint8Array(Module.HEAPU8.buffer, ptr, len);
    Module.NES.image.data.set(Module.NES.buf);
    Module.NES.ctx.putImageData(Module.NES.image, 0, 0);
  },
  start_oscillator: function (index) {
    Module.NES.oscs[index].start();
  },
  stop_oscillator: function (index) {
    Module.NES.oscs[index].stop();
  },
  close_oscillator: function (index) {
    Module.NES.oscs[index].close();
  },
  set_oscillator_frequency: function (index, freq) {
    console.log(index, freq)
    Module.NES.oscs[index].setFrequency(freq);
  },
  change_oscillator_frequency: function (index, freq) {
    Module.NES.oscs[index].changeFrequency(freq);
  },
  set_oscillator_volume: function (index, volume) {
    Module.NES.oscs[index].setVolume(volume);
  },
  set_oscillator_pulse_width: function (index, width) {
    Module.NES.oscs[index].setPulseWidth(width);
  },
  set_noise_frequency: function (freq) {
    Module.NES.noise.setFrequency(freq);
  },
  set_noise_volume: function (volume) {
    Module.NES.noise.setVolume(volume);
  },
  close_noise: function () {
    Module.NES.noise.close();
  },
  stop_noise: function () {
    Module.NES.noise.stop();
  },
  start_noise: function () {
    Module.NES.noise.start();
  }
});
