extern crate libc;

mod nes;
mod parser;
mod externs;

use nes::Nes;

fn main() {}

extern "C" fn test() {
    externs::eval("render();");
}

#[no_mangle]
pub extern "C" fn run(len: usize, ptr: *mut u8) -> u8 {
    let buf: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    externs::set_main_loop(test);
    let nes = parser::parse(buf);
    // Nes::new(nes);
    10
}
