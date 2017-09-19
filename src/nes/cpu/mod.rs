mod opecode;

use std::collections::HashMap;
use nes::bus::cpu_bus::CpuBus;
// use self::opecode;

struct Status {
    negative: bool,
    overflow: bool,
    reserved: bool,
    break_mode: bool,
    decimal_mode: bool,
    interrupt: bool,
    zero: bool,
    carry: bool,
}

pub struct Cpu {
    A: u8,
    X: u8,
    Y: u8,
    PC: u16,
    P: Status,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            A: 0,
            X: 0,
            Y: 0,
            PC: 0x8000,
            P: Status {
                negative: true,
                overflow: true,
                reserved: true,
                break_mode: true,
                decimal_mode: true,
                interrupt: true,
                zero: true,
                carry: true,
            },
        }
    }

    pub fn run(&self, mut bus: &mut CpuBus) -> usize {

        let ref m = opecode::opecode::MAP;
        let a = 0xA5;
        println!("{:?}", *m.get(&a).unwrap());

        // let mut map = HashMap::new();
        // map.insert(1u8, 12);
        // let n = map.get(&1).unwrap();
        // println!("{}", n);
        //bus.write(0, 100);
        // println!("{}", bus.read(0));
        // let n = self.opecode.get(&1).unwrap();
        // println!("{:?}", n);
        20
    }
}
