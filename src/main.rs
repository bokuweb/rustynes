#[macro_use]
extern crate lazy_static;
extern crate libc;

mod nes;
mod externs;

use std::cell::RefCell;
use std::ptr::null_mut;

use nes::Nes;

fn main() {}

thread_local!(static NES: RefCell<*mut Nes> = RefCell::new(null_mut()));

extern "C" fn main_loop() {
    NES.with(|n| {
        let nes = *n.borrow_mut() as *mut Nes;
        // let a = vec![10, 20, 40, 50];
        // externs::eval(&format!("console.log({:?});", a));
        unsafe {
            let a = (*nes).run();
            let js = [
                "const canvas = document.querySelector('canvas');",
                "const ctx = canvas.getContext('2d');",
                "const image = ctx.createImageData(256, 240);",
                "for (let i = 0; i < 256 * 240; i += 1) {",
                "const color = 0;",
                "image.data[i * 4] = color;",
                "image.data[i * 4 + 1] = color;",
                "image.data[i * 4 + 2] = color;",
                "image.data[i * 4 + 3] = 0xFF;",
                "}",
                "ctx.putImageData(image, 0, 0);",
            ].join("");
            externs::eval(&js);
        }
    });
}

#[no_mangle]
pub extern "C" fn run(len: usize, ptr: *mut u8) {
    let buf: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    let nes = Nes::new(buf);
    NES.with(|n| { *n.borrow_mut() = &nes as *const _ as *mut Nes; });
    externs::set_main_loop(main_loop);
}
