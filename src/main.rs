// extern crate libc;

mod externs;
// mod bridge;
// extern crate emscripten_sys;

mod nes;

use nes::Nes;

fn main() {}

#[no_mangle]
pub extern "C" fn update(len: usize, ptr: *mut bool, col: usize) -> u8 {
    let row = len / col;
    let buf: &mut [bool] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    // fetch();
    // externs::fetch("console.log('Hello, eval!')");
    // let a = externs::now();
    let a = "roms/hello.nes";
    let b = externs::fetch(a);
    println!("{:?}", b);
    Nes::new()
    // buf.clone_from_slice(game.as_slice())
}
