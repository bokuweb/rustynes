mod opecode;

use std::collections::HashMap;
use nes::bus::cpu_bus::CpuBus;

pub struct Cpu {
    opecode: HashMap<String, u8>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { 
            opecode: opecode::build_opecode_map().clone()
             }
    }

    pub fn run(&self, mut bus: &mut CpuBus) -> usize {

        // let mut map = HashMap::new();
        // map.insert(1 as u8, 12);
        // let n = map.get(&1).unwrap();
        // println!("{}", n);
        //bus.write(0, 100);
        // println!("{}", bus.read(0));
        let n = self.opecode.get(&String::from("0xA9")).unwrap();
        // println!("{:?}", n);
        20
    }
}
