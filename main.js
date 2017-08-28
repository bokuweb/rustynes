const runNES = Module.cwrap('run', null, ['number', 'number']);
const canvas = document.querySelector("canvas");
const ctx = canvas.getContext('2d');

const width = 256;
const height = 240;

canvas.width = width;
canvas.height = height;

const image = ctx.createImageData(width, height);

fetch('./roms/hello.nes')
    .then((res) => res.arrayBuffer())
    .then((arrBuf) => {
        const nes = new Uint8Array(arrBuf);
        const size = nes.byteLength;
        const ptr = Module._malloc(size);
        // console.log('nes', nes)
        const buf = new Uint8Array(Module.HEAPU8.buffer, ptr, size);
        for (let i = 0; i < nes.length - 1; i = i + 1) {
            buf[i] = nes[i];
        }
        runNES(size, buf.byteOffset);
    });



