extern crate rustynes;

use std::fs;
use rustynes::nes;
use rustynes::nes::Context;

fn main() {
    match fs::read("roms/falling.nes") {
        Result::Ok(mut rom) => {
            let mut ctx = Context::new(&mut rom);
            nes::reset(&mut ctx);
        },
        Result::Err(err) => {
            panic!(err);
        }
    }
}
