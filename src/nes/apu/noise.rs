use super::constants::{NOISE_TIMER_PERIOD_TABLE, GROBAL_GAIN, CPU_CLOCK, COUNTER_TABLE};
use nes::types::{Data, Addr};

#[derive(Debug)]
pub struct Noise {
    envelope_generator_counter: usize,
    envelope_rate: usize,
    envelope_volume: usize,
    envelope_enable: bool,
    is_length_counter_enable: bool,
    length_counter: usize,

    divider_for_frequency: usize,
    frequency: usize,
}

extern "C" {
    fn set_noise_frequency(freq: f32);
    fn set_noise_volume(volume: f32);
    fn stop_noise();
    fn start_noise();
    // fn close_noise();
}

impl Noise {
    pub fn new() -> Self {
        Noise {
            envelope_generator_counter: 0,
            envelope_rate: 0x0F,
            envelope_volume: 0x0F,
            envelope_enable: false,

            is_length_counter_enable: false,
            length_counter: 0,
            divider_for_frequency: 1,
            frequency: 0,
        }
    }

    fn get_volume(&self) -> f32 {
        let vol = if self.envelope_enable {
            self.envelope_volume
        } else {
            self.envelope_rate
        };
        vol as f32 / (16.0 / GROBAL_GAIN)
    }

    pub fn update_envelope(&mut self) {
        self.envelope_generator_counter -= 1;
        if self.envelope_generator_counter <= 0 {
            self.envelope_generator_counter = self.envelope_rate;
            if self.envelope_volume > 0 {
                self.envelope_volume -= 1;
            } else {
                self.stop();
                self.envelope_volume = 0x0F;
            }
        }
        self.set_volume();
    }

    pub fn start(&self) {
        unsafe { start_noise() };
    }

    pub fn stop(&self) {
        unsafe { stop_noise() };
    }

    // Length counter
    // When clocked by the frame counter, the length counter is decremented except when:
    // The length counter is 0, or The halt flag is set
    pub fn update_counter(&mut self) {
        if self.is_length_counter_enable && self.length_counter > 0 {
            self.length_counter -= 1;
        }
        if self.is_length_counter_enable && self.length_counter == 0 {
            self.stop();
        }
    }

    // pub fn has_count_end(&self) -> bool {
    //     self.length_counter == 0
    // }

    // fn reset(&mut self) {
    //     self.length_counter = 0;
    //     self.is_length_counter_enable = false;
    //     self.set_volume();
    // }

    fn set_volume(&self) {
        unsafe { set_noise_volume(self.get_volume()) }
    }

    fn set_frequency(&self, data: Data) {
        unsafe {
            set_noise_frequency(CPU_CLOCK as f32 /
                                NOISE_TIMER_PERIOD_TABLE[data as usize & 0xF] as f32 /
                                2f32)
        }
    }

    pub fn write(&mut self, addr: Addr, data: Data) {
        // println!("noise write {:x} {:x}", addr, data);
        match addr {
            0x00 => {
                self.envelope_enable = (data & 0x10) == 0;
                self.envelope_rate = data as usize & 0xF;
                self.is_length_counter_enable = data & 0x20 == 0x00;
                self.set_volume();
            }
            0x02 => {
                // this.isShortPeriod = !!(data & 0x80);
                self.set_frequency(data);
            }    
            0x03 => {
                if self.is_length_counter_enable {
                    self.length_counter = COUNTER_TABLE[(data as usize & 0xF8) >> 3] as usize;
                }
                self.envelope_generator_counter = self.envelope_rate;
                self.envelope_volume = 0x0F;
                self.set_volume();
                self.start();
            }                        
            _ => (),
        }
    }
}
