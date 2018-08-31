import Oscillator from './src/nes/browser/oscillator.js';
import Noise from './src/nes/browser/noise.js';

let buf = null

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
  if (buf != null)
    buf[buf.length - 1] |= convertKeyCode(e.keyCode);
}

const onKeyup = (e) => {
  if (buf != null)
    buf[buf.length - 1] &= ~convertKeyCode(e.keyCode);
}

const setupKeyHandler = () => {
  if (typeof window !== 'undefined') {
    document.addEventListener('keydown', onKeydown);
    document.addEventListener('keyup', onKeyup);
  }
};

setupKeyHandler();

const startArrayBuf = (arrayBuf) => {
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

  const nes = new Uint8Array(arrayBuf);
  // Add key code area to tail.
  const size = nes.byteLength + 1;
  const ptr = Module._malloc(size);
  buf = new Uint8Array(Module.HEAPU8.buffer, ptr, size);
  buf.set(nes);

  console.log('start nes');
  run(size, buf.byteOffset);
};

export const start = async (rom = './roms/falling.nes') => {
  const res = await fetch(rom);
  const arrayBuf = await res.arrayBuffer();
  startArrayBuf(arrayBuf);
};

export const startFile = async (file) => {
  const loadFile = (file) => {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = (e) => resolve(e.target.result);
      reader.onerror = (e) => reject(reader.error);
      reader.readAsArrayBuffer(file);
    });
  };

  const arrayBuf = await loadFile(file)
  startArrayBuf(arrayBuf);
};
