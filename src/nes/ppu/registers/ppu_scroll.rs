
use super::super::super::types::Data;
use super::super::super::Ram;
use super::super::palette::*;

#[derive(Debug)]
enum Enable {
    X,
    Y,
}

#[derive(Debug)]
pub struct PpuScroll {
    x: Data,
    y: Data,
    enable: Enable,
}

impl PpuScroll {
    pub fn new() -> Self {
        PpuScroll {
            x: 0,
            y: 0,
            enable: Enable::X,
        }
    }

    pub fn enable_x(&mut self) {
        self.enable = Enable::X;
    }    

    pub fn get_x(&self) -> Data {
        self.x
    }

    pub fn get_y(&self) -> Data {
        self.y
    }    

    pub fn write(&mut self, data: Data) {
        match self.enable {
            Enable::X => {
                self.enable = Enable::Y;
                self.x = data;
            },
            Enable::Y => {
                self.enable = Enable::X;
                self.y = data;
            }
        }
    }
}