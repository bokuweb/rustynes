extern crate libc;

mod nes;
mod parser;
mod externs;

use nes::Nes;

fn main() {}

extern "C" fn test() {
    // println!("{}", "tick");
    externs::eval("render();");
}

#[no_mangle]
pub extern "C" fn init(len: usize, ptr: *mut u8) -> u8 {
    println!("{}", "NES initiarized...");
    10
}

#[no_mangle]
pub extern "C" fn run(len: usize, ptr: *mut u8) -> u8 {
    externs::eval("console.info('NES started....')");
    let buf: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    externs::set_main_loop(test);
    let nes = parser::parse(buf);
    // Nes::new(nes);
    10
}
