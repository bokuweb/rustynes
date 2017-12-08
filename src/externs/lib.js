mergeInto(LibraryManager.library, {
    canvas_render: function (ptr, len) {
        var buf = new Uint8Array(Module.HEAPU8.buffer, ptr, len);
        for (var i = 0; i < buf.length; i += 1) {
            window.__nes.image.data[i] = buf[i];
        }
        window.__nes.ctx.putImageData(window.__nes.image, 0, 0);
    }
});