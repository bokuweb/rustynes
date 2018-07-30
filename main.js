import Oscillator from './src/nes/browser/oscillator.js';
import Noise from './src/nes/browser/noise.js';

const start = async (rom = './roms/falling.nes') => {
  const run = Module.cwrap('run', null, ['number', 'number']);
  const canvas = document.querySelector("canvas");
  const ctx = canvas.getContext('2d');
  if (Module.NES) {
    Module.NES.oscs.forEach(o => o.close());
    Module.NES.noise.close();
  }
  Module.NES = {
    ctx,
    canvas,
    image: ctx.createImageData(256, 240),
    oscs: [new Oscillator(), new Oscillator(), new Oscillator('triangle')],
    noise: new Noise(),
  }
  canvas.width = 256;
  canvas.height = 240;

  const res = await fetch(rom);
  const arrayBuf = await res.arrayBuffer();
  const nes = new Uint8Array(arrayBuf);
  // Add key code area to tail.
  const size = nes.byteLength + 1;
  const ptr = Module._malloc(size);
  const buf = new Uint8Array(Module.HEAPU8.buffer, ptr, size);
  for (let i = 0; i < nes.length - 1; i = i + 1) {
    buf[i] = nes[i];
  }

  const convertKeyCode = (keyCode) => {
    switch (keyCode) {
      case 88: return 0x01; // X  A 
      case 90: return 0x02; // Z  B
      case 65: return 0x04; // A  SELECT
      case 83: return 0x08; // S  START
      case 38: return 0x10; // ↑  ↑  
      case 40: return 0x20; // ↓  ↓
      case 37: return 0x40; // ←  ←
      case 39: return 0x80; // →  →
    }
  };

  const onKeydown = (e) => {
    buf[size - 1] |= convertKeyCode(e.keyCode);
  }

  const onKeyup = (e) => {
    buf[size - 1] &= ~convertKeyCode(event.keyCode);
  }

  const setupKeyHandler = () => {
    if (typeof window !== 'undefined') {
      document.addEventListener('keydown', onKeydown);
      document.addEventListener('keyup', onKeyup);
    }
  };

  setupKeyHandler();
  console.log('start nes');
  run(size, buf.byteOffset);
};

export default start;

