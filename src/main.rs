extern crate libc;

mod nes;
mod externs;

use nes::Nes;

fn main() {}

#[no_mangle]
pub extern "C" fn run(len: usize, ptr: *mut u8) -> u8 {
    let buf: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    let mut nes = Nes::new(buf);
    externs::set_main_loop_callback(|| {
        println!("{}", nes.run());
        // externs::eval("render()");
    });
    10
}
