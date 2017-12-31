
(async () => {
  // const main = Module.cwrap('run');
  const run = Module.cwrap('run', null, ['number', 'number']);
  const canvas = document.querySelector("canvas");
  const ctx = canvas.getContext('2d');
  Module.NES = {
    ctx,
    canvas,
    image: ctx.createImageData(256, 240),
  }
  canvas.width = 256;
  canvas.height = 240;


  const res = await fetch('./roms/giko011.nes');
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

  const setupKeyHandler = () => {
    if (typeof window !== 'undefined') {
      document.addEventListener('keydown', (event) => {
        buf[size - 1] |= convertKeyCode(event.keyCode);
      });

      document.addEventListener('keyup', (event) => {
        buf[size - 1] &= ~convertKeyCode(event.keyCode);
      });
    }
  };
  setupKeyHandler();
  run(size, buf.byteOffset);
  // main();
})().catch(e => {
  if (e == 'SimulateInfiniteLoop') {
    Module['noExitRuntime'] = true;
    return;
  }
});



