#[macro_use]
extern crate lazy_static;
extern crate libc;

mod nes;
mod externs;

use std::cell::RefCell;
use std::ptr::null_mut;
use std::rc::Rc;

use nes::Nes;

fn main() {
    // let buf: &mut [u8] = vec![120, 120, 120, 120, 120];
    // let mut nes = Box::new(Nes::new(buf));

    // println!("{:?}", nes.program_rom);
    // NES.with(|n| { *n.borrow_mut() = &Nes::new(buf) as *const _ as *mut libc::c_void; });
    // let mut nes = Rc::new(RefCell::new(Nes::new(buf)));
    let mut nes = Nes::new();
    // nes.borrow_mut().reset();
    nes.reset();
    let mut main_loop = || {
        //NES.with(|n| {
        //let mut nes = *n.borrow_mut() as *mut Nes;
        // let a = vec![10, 20, 40, 50];

        // externs::eval(&format!("console.log({:?});", a));
        println!("{:?}", nes.cpu);
        // nes.borrow_mut().run();
        nes.run();
        // let js = ["const canvas = document.querySelector('canvas');",
        //           "const ctx = canvas.getContext('2d');",
        //           "const image = ctx.createImageData(256, 240);",
        //           "for (let i = 0; i < 256 * 240; i += 1) {",
        //           "const color = 0;",
        //           "image.data[i * 4] = color;",
        //           "image.data[i * 4 + 1] = color;",
        //           "image.data[i * 4 + 2] = color;",
        //           "image.data[i * 4 + 3] = 0xFF;",
        //           "}",
        //           "ctx.putImageData(image, 0, 0);"]
        //         .join("");
        // externs::eval(&js);
    };
    externs::set_main_loop_callback(main_loop);
}

// thread_local!(static NES: RefCell<*mut libc::c_void> = RefCell::new(null_mut()));

// extern "C" fn main_loop() {
//     NES.with(|n| {
//         let mut nes = *n.borrow_mut() as *mut Nes;
//         // let a = vec![10, 20, 40, 50];
//
//         // externs::eval(&format!("console.log({:?});", a));
//         unsafe {
//             println!("{:?}", (*nes).cpu);
//             let a = (*nes).run();
//             let js = ["const canvas = document.querySelector('canvas');",
//                       "const ctx = canvas.getContext('2d');",
//                       "const image = ctx.createImageData(256, 240);",
//                       "for (let i = 0; i < 256 * 240; i += 1) {",
//                       "const color = 0;",
//                       "image.data[i * 4] = color;",
//                       "image.data[i * 4 + 1] = color;",
//                       "image.data[i * 4 + 2] = color;",
//                       "image.data[i * 4 + 3] = 0xFF;",
//                       "}",
//                       "ctx.putImageData(image, 0, 0);"]
//                     .join("");
//             externs::eval(&js);
//         }
//     });
// }

/*
#[no_mangle]
pub fn run(len: usize, ptr: *mut u8) {
    let buf: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    // let mut nes = Box::new(Nes::new(buf));

    // println!("{:?}", nes.program_rom);
    // NES.with(|n| { *n.borrow_mut() = &Nes::new(buf) as *const _ as *mut libc::c_void; });
    // let mut nes = Rc::new(RefCell::new(Nes::new(buf)));
    let mut nes = Nes::new(buf);
    // nes.borrow_mut().reset();
    nes.reset();
    let mut main_loop = || {
        //NES.with(|n| {
        //let mut nes = *n.borrow_mut() as *mut Nes;
        // let a = vec![10, 20, 40, 50];

        // externs::eval(&format!("console.log({:?});", a));
        println!("{:?}", nes.cpu);
        // nes.borrow_mut().run();
        nes.run();
        // let js = ["const canvas = document.querySelector('canvas');",
        //           "const ctx = canvas.getContext('2d');",
        //           "const image = ctx.createImageData(256, 240);",
        //           "for (let i = 0; i < 256 * 240; i += 1) {",
        //           "const color = 0;",
        //           "image.data[i * 4] = color;",
        //           "image.data[i * 4 + 1] = color;",
        //           "image.data[i * 4 + 2] = color;",
        //           "image.data[i * 4 + 3] = 0xFF;",
        //           "}",
        //           "ctx.putImageData(image, 0, 0);"]
        //         .join("");
        // externs::eval(&js);
    };
    externs::set_main_loop_callback(main_loop);
}
*/