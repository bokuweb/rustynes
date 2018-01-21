mergeInto(LibraryManager.library, {
  canvas_render: function (ptr, len) {
    Module.NES.buf = new Uint8Array(Module.HEAPU8.buffer, ptr, len);
    Module.NES.image.data.set(Module.NES.buf);
    Module.NES.ctx.putImageData(Module.NES.image, 0, 0);
  },
  test1: function (a, b, c) {
    Module.NES.test1(a, b, c);
  },
  test2: function () {
    Module.NES.test2();
  }
});