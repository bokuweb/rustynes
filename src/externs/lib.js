mergeInto(LibraryManager.library, {
    canvas_render: function (ptr, len) {
        var buf = new Uint8Array(Module.HEAPU8.buffer, ptr, len);
        // console.log(buf);
        var canvas = document.querySelector('canvas');
        var ctx = canvas.getContext('2d');
        var image = ctx.createImageData(256, 240);
        for (var i = 0; i < buf.length; i += 1) {
            image.data[i] = buf[i];
        }
        ctx.putImageData(image, 0, 0);
    }
});