#[macro_use]
extern crate lazy_static;
extern crate libc;

mod nes;
mod externs;

use nes::Context;

fn main() {}

#[no_mangle]
pub fn run(len: usize, ptr: *mut u8) {
    let buf: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    let mut ctx = Context::new(buf);
    nes::reset(&mut ctx);
    let main_loop = || { nes::run(&mut ctx); };
    externs::set_main_loop_callback(main_loop);
}
