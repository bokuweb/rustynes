mod nes;

use nes::Nes;

fn main() {}

#[no_mangle]
pub extern "C" fn run(len: usize, ptr: *mut u8) -> u8 {
    // let row = len / col;
    let buf: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    println!("{:?}", buf[0]);
    println!("{}", buf.len());
    // fetch();
    // externs::fetch("console.log('Hello, eval!')");
    // let a = externs::now();
    // let a = "roms/hello.nes";
    // let b = externs::fetch(a);
    // println!("{:?}", b);
    Nes::new()
    // buf.clone_from_slice(game.as_slice())
}
