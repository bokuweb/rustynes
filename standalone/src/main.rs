extern crate rustynes;
extern crate sdl2;

use sdl2::{Sdl};
use sdl2::event::{Event};
use sdl2::keyboard::{Keycode};
use sdl2::pixels::{Color};
use sdl2::render::{WindowCanvas};
use sdl2::rect::{Point};

use std::time::{Duration, SystemTime};

use std::env;
use std::fs;
use rustynes::nes;
use rustynes::nes::Context;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 224;

const PAD_A: u8      = 0x01;
const PAD_B: u8      = 0x02;
const PAD_SELECT: u8 = 0x04;
const PAD_START: u8  = 0x08;
const PAD_U: u8      = 0x10;
const PAD_D: u8      = 0x20;
const PAD_L: u8      = 0x40;
const PAD_R: u8      = 0x80;

fn keycode_to_pad(key: Keycode) -> u8 {
    match key {
        Keycode::X => PAD_A,
        Keycode::Z => PAD_B,
        Keycode::A => PAD_SELECT,
        Keycode::S => PAD_START,
        Keycode::Up => PAD_U,
        Keycode::Down => PAD_D,
        Keycode::Left => PAD_L,
        Keycode::Right => PAD_R,
        _ => 0,
    }
}

pub struct App {
    sdl_context: Sdl,
    canvas: WindowCanvas,

    ctx: Option<Context>,
}

impl App {
    pub fn new() -> App {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("rustynes", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        App {
            sdl_context,
            canvas,
            ctx: None,
        }
    }

    pub fn set_rom(&mut self, mut rom: Vec<u8>) {
        let mut ctx = Context::new(&mut rom);
        nes::reset(&mut ctx);
        self.ctx = Some(ctx);
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut pad = 0;
        let mut prev_time = SystemTime::now();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    Event::KeyDown { keycode: Some(key), .. } => {
                        pad |= keycode_to_pad(key);
                    },
                    Event::KeyUp { keycode: Some(key), .. } => {
                        pad &= !keycode_to_pad(key);
                    },
                    _ => {}
                }
            }

            self.update(pad);
            self.render();
            self.canvas.present();

            let elapsed_time = SystemTime::now().duration_since(prev_time).expect("Time went backwards").as_nanos();
            let wait = if elapsed_time < 1_000_000_000u128 / 60 { 1_000_000_000u32 / 60 - (elapsed_time as u32) } else { 0 };
            ::std::thread::sleep(Duration::new(0, wait));

            prev_time = SystemTime::now();
        }
    }

    fn update(&mut self, pad: u8) {
        let optctx = &mut self.ctx;
        match optctx {
            Some(ctx) => {
                nes::run(ctx, pad);
            },
            None => (),
        }
    }

    fn render(&mut self) {
        match &mut self.ctx {
            Some(ctx) => {
                let buf = nes::get_render_buf(ctx);
                for i in 0..HEIGHT {
                    for j in 0..WIDTH {
                        let base = ((i * WIDTH + j) * 4) as usize;
                        let r = buf[base + 0];
                        let g = buf[base + 1];
                        let b = buf[base + 2];
                        self.canvas.set_draw_color(Color::RGB(r, g, b));
                        let _ = self.canvas.draw_point(Point::new(j as i32, i as i32));
                    }
                }
            },
            None => (),
        }
    }
}

#[no_mangle]
fn canvas_render(_ptr: *const u8, _len: usize) {
    //println!("canvas_render, len={}", len);
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
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("<.nes file> required");
        std::process::exit(1);
    }

    let mut app = App::new();

    let filename = &args[1];
    match fs::read(filename) {
        Result::Ok(rom) => {
            app.set_rom(rom);
            app.run();
        },
        Result::Err(err) => {
            eprintln!("Cannot open .nes file: {}", filename);
            panic!(err);
        }
    }
}
