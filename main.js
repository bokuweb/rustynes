const update = Module.cwrap('update', null, ['number', 'number', 'number']);
const canvas = document.querySelector("canvas");
const ctx = canvas.getContext('2d');
const width = window.innerWidth;
const height = window.innerHeight;
canvas.width = width;
canvas.height = height;

const image = ctx.createImageData(width, height);
const column = ~~(width);
const bufsize = ~~(height) * column;
const bufptr = Module._malloc(bufsize);

Module._memset(bufptr, 0, bufsize);
let buf = new Uint8Array(Module.HEAPU8.buffer, bufptr, bufsize);

console.log(update(bufsize, buf.byteOffset, column));
