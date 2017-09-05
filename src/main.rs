extern crate libc;

mod nes;
mod parser;
mod externs;

use nes::Nes;

fn main() {}

#[no_mangle]
pub extern "C" fn run(len: usize, ptr: *mut u8) -> u8 {
    let buf: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    let nes = parser::parse(buf);
    let mut nes = Nes::new();
    // let a = 10;
    println!("{}", nes.a);
    externs::set_main_loop_callback(|| {
        println!("{}", nes.add());
    });
    10
}
