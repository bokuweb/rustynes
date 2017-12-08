mergeInto(LibraryManager.library, {
    canvas_render: function (ptr, len) {
        var buf = new Uint8Array(Module.HEAPU8.buffer, ptr, len);
        for (var i = 0; i < buf.length; i += 1) {
            Module.NES.image.data[i] = buf[i];
        }
        Module.NES.ctx.putImageData(Module.NES.image, 0, 0);
    }
});