use super::constants::*;
use nes::types::{Data, Addr, Word};

#[derive(Debug)]
pub struct Triangle {
    index: usize,
    is_length_counter_enable: bool,
    length_counter: usize,
    linear_counter: usize,
    divider_for_frequency: usize,
    frequency: usize,
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
        }
    }

    fn get_volume(&self) -> f32 {
        8.0 / (16.0 / GROBAL_GAIN)
    }

    fn stop_oscillator(&mut self) {
        self.length_counter = 0;
        unsafe {
            stop_oscillator(self.index);
            set_oscillator_volume(self.index, 0.0);
        };
    }

    pub fn stop(&mut self) {
        self.stop_oscillator();
    }

    // Length counter
    // When clocked by the frame counter, the length counter is decremented except when:
    // The length counter is 0, or The halt flag is set
    pub fn update_counter(&mut self) {
        println!("ln counter {} {}", self.length_counter, self.linear_counter);
        if self.is_length_counter_enable && self.length_counter > 0 {
            self.length_counter -= 1;
        }
        if self.linear_counter > 0 {
            self.linear_counter -= 1;
        }
        if (self.is_length_counter_enable && self.length_counter == 0) || self.linear_counter == 0 {
            println!("stop");
            self.stop();
        }
    }

    pub fn start(&self) {
        unsafe {
            start_oscillator(self.index);
            set_oscillator_frequency(self.index, self.frequency);
        };
    }

    pub fn close(&self) {
        unsafe { close_oscillator(self.index) };
    }

    pub fn has_count_end(&self) -> bool {
        self.length_counter == 0
    }

    fn reset(&mut self) {
        self.length_counter = 0;
        self.is_length_counter_enable = false;
        self.set_volume();
    }

    fn set_volume(&self) {
        unsafe { set_oscillator_volume(self.index, self.get_volume()) }
    }

    pub fn write(&mut self, addr: Addr, data: Data) {
        println!("wr triangle {:x} {:x}", addr, data);
        match addr {
            0x00 => {
                self.is_length_counter_enable = data & 0x80 == 0;
                self.linear_counter = data as usize & 0x7F;
                // self.set_volume();
            }
            0x02 => {
                self.divider_for_frequency &= 0x700;
                self.divider_for_frequency |= data as usize;
            }    
            0x03 => {
                // Programmable timer, length counter
                self.divider_for_frequency &= 0xFF;
                self.divider_for_frequency |= ((data as usize & 0x7) << 8);
                if self.is_length_counter_enable {
                    self.length_counter = COUNTER_TABLE[(data & 0xF8) as usize >> 3] as usize;
                }
                self.frequency = (CPU_CLOCK / ((self.divider_for_frequency + 1) * 32)) as usize;
                self.set_volume();
                if self.linear_counter != 0 {
                    self.start();
                }
            }                        
            _ => (),
        }
    }
}
