extern crate libc;

mod nes;
mod parser;
mod externs;

use nes::Nes;

fn main() {}

extern "C" fn test() {
    // println!("{}", "tick");
}

#[no_mangle]
pub extern "C" fn run(len: usize, ptr: *mut u8) -> u8 {
    externs::eval("console.info('NES started....')");
    let buf: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    externs::set_main_loop(test);
    parser::parse(buf);
    Nes::new()
}
