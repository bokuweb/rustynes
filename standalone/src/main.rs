extern crate rustynes;
extern crate sdl2;

use sdl2::{Sdl};
use sdl2::event::{Event};
use sdl2::keyboard::{Keycode};
use sdl2::pixels::{Color};
use sdl2::render::{WindowCanvas};
use sdl2::rect::{Point};

use std::time::{Duration};

use std::fs;
use rustynes::nes;
use rustynes::nes::Context;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 224;

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
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }

            self.update();
            self.render();
            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn update(&mut self) {
        let optctx = &mut self.ctx;
        match optctx {
            Some(ctx) => {
                let key_state = 0;
                nes::run(ctx, key_state);
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
    let mut app = App::new();

    match fs::read("roms/falling.nes") {
        Result::Ok(rom) => {
            app.set_rom(rom);
            app.run();
        },
        Result::Err(err) => {
            panic!(err);
        }
    }
}
