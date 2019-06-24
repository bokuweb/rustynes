extern crate rustynes;

use std::fs;
use rustynes::nes;
use rustynes::nes::Context;

#[no_mangle]
fn canvas_render(_ptr: *const u8, len: usize) {
    println!("canvas_render, len={}", len);
}

#[no_mangle]
fn start_oscillator(_index: usize) {}
#[no_mangle]
fn stop_oscillator(_index: usize) {}
//#[no_mangle]
// fn close_oscillator(index: usize) {}
#[no_mangle]
fn set_oscillator_frequency(_index: usize, _freq: usize) {}
#[no_mangle]
fn change_oscillator_frequency(_index: usize, _freq: usize) {}
#[no_mangle]
fn set_oscillator_volume(_index: usize, _volume: f32) {}
#[no_mangle]
fn set_oscillator_pulse_width(_index: usize, _width: f32) {}

#[no_mangle]
fn set_noise_frequency(_freq: f32) {}
#[no_mangle]
fn set_noise_volume(_volume: f32) {}
#[no_mangle]
fn stop_noise() {}
#[no_mangle]
fn start_noise() {}
//#[no_mangle]
//fn close_noise();

fn main() {
    match fs::read("roms/falling.nes") {
        Result::Ok(mut rom) => {
            let mut ctx = Context::new(&mut rom);
            nes::reset(&mut ctx);

            for _i in 0..10 {
                let key_state = 0;
                nes::run(&mut ctx, key_state);
            }
        },
        Result::Err(err) => {
            panic!(err);
        }
    }
}
