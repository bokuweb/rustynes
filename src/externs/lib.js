mergeInto(LibraryManager.library, {
  canvas_render: function (ptr, len) {
    var buf = new Uint8Array(Module.HEAPU8.buffer, ptr, len);
    Module.NES.image.data.set(buf);
    Module.NES.ctx.putImageData(Module.NES.image, 0, 0);
  }
});