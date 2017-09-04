(async () => {
  const run = Module.cwrap('run', null, ['number', 'number']);
  const canvas = document.querySelector("canvas");
  const ctx = canvas.getContext('2d');

  const width = 256;
  const height = 240;

  canvas.width = width;
  canvas.height = height;

  const image = ctx.createImageData(width, height);

  const res = await fetch('./roms/hello.nes');
  const arrayBuf = await res.arrayBuffer();
  const nes = new Uint8Array(arrayBuf);
  const size = nes.byteLength;
  const ptr = Module._malloc(size);
  const buf = new Uint8Array(Module.HEAPU8.buffer, ptr, size);
  for (let i = 0; i < nes.length - 1; i = i + 1) {
    buf[i] = nes[i];
  }
  window.render = () => console.log("a", buf[0]);
  run(size, buf.byteOffset);
})();




