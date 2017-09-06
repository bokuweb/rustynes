extern crate libc;

mod nes;
mod externs;

use std::cell::RefCell;
use std::ptr::null_mut;

use nes::Nes;

fn main() {}

extern "C" fn main_loop() {
    NES.with(|n| {
        let nes = *n.borrow_mut() as *mut Nes;
        unsafe {
            let a = (*nes).run();
            let js = format!("console.log({value})", value = a);
            externs::eval(&js);
        }
    });
}

thread_local!(static NES: RefCell<*mut Nes> = RefCell::new(null_mut()));

#[no_mangle]
pub extern "C" fn run(len: usize, ptr: *mut u8) {
    let buf: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    let nes = Nes::new(buf);
    NES.with(|n| { *n.borrow_mut() = &nes as *const _ as *mut Nes; });
    externs::set_main_loop(main_loop);
}
