use super::constants::*;
use nes::types::{Data, Addr};

#[derive(Debug)]
pub struct Triangle {
    index: usize,
    is_length_counter_enable: bool,
    length_counter: usize,
    linear_counter: usize,
    divider_for_frequency: usize,
    frequency: usize,
    enable: bool,
    playing: bool,
}

extern "C" {
    fn start_oscillator(index: usize);
    fn stop_oscillator(index: usize);
    fn close_oscillator(index: usize);
    fn set_oscillator_frequency(index: usize, freq: usize);
    fn change_oscillator_frequency(index: usize, freq: usize);
    fn set_oscillator_volume(index: usize, volume: f32);
    fn set_oscillator_pulse_width(index: usize, width: f32);
}

impl Triangle {
    pub fn new(index: usize) -> Self {
        Triangle {
            index,
            is_length_counter_enable: false,
            length_counter: 0,
            linear_counter: 0,
            divider_for_frequency: 1,
            frequency: 0,
            enable: false,
            playing: false,
        }
    }

    fn get_volume(&self) -> f32 {
        16.0 / (16.0 / GROBAL_GAIN)
    }

    fn stop_oscillator(&mut self) {
        self.length_counter = 0;
        self.linear_counter = 0;
        unsafe {
            stop_oscillator(self.index);
            set_oscillator_volume(self.index, 0.0);
        };
    }

    pub fn enable(&mut self) {
        println!("enable");
        self.enable = true;
        self.start();
    }

    pub fn disable(&mut self) {
        println!("disable");
        self.enable = false;
        self.stop();
    }

    pub fn stop(&mut self) {
        if self.playing {
            self.playing = false;
            self.stop_oscillator();
        }
    }

    // Length counter
    // When clocked by the frame counter, the length counter is decremented except when:
    // The length counter is 0, or The halt flag is set
    pub fn update_counter(&mut self) {
        if self.is_length_counter_enable && self.length_counter > 0 {
            self.length_counter -= 1;
        }
        if self.linear_counter > 0 {
            self.linear_counter -= 1;
        }
        if !self.is_length_counter_enable {
            return;
        }
        if self.length_counter == 0 || self.linear_counter == 0 {
            self.stop();
        }
    }

    pub fn start(&mut self) {
        if !self.playing {
            self.playing = true;
            unsafe {
                start_oscillator(self.index);
                set_oscillator_frequency(self.index, self.frequency);
            };
        } else {
            unsafe {
                change_oscillator_frequency(self.index, self.frequency);
            }
        }
    }

    pub fn has_count_end(&self) -> bool {
        self.length_counter == 0
    }

    fn set_volume(&self) {
        unsafe { set_oscillator_volume(self.index, self.get_volume()) }
    }

    pub fn write(&mut self, addr: Addr, data: Data) {
        match addr {
            0x00 => {
                self.is_length_counter_enable = data & 0x80 == 0;
                self.linear_counter = data as usize & 0x7F;
            }
            0x02 => {
                self.divider_for_frequency &= 0x700;
                self.divider_for_frequency |= data as usize;
            }    
            0x03 => {
                // Programmable timer, length counter
                self.divider_for_frequency &= 0xFF;
                self.divider_for_frequency |= (data as usize & 0x7) << 8;
                if self.is_length_counter_enable {
                    self.length_counter = COUNTER_TABLE[(data & 0xF8) as usize >> 3] as usize / 2;
                }
                self.frequency = (CPU_CLOCK / ((self.divider_for_frequency + 1) * 32)) as usize;
                self.set_volume();
                if self.enable {
                    self.start();
                }
            }                        
            _ => (),
        }
    }
}
