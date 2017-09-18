mod opecode;

use std::collections::HashMap;
use nes::bus::cpu_bus::CpuBus;
// use self::opecode;

pub struct Cpu {
    // opecode: HashMap<u8, u8>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            //opecode: opecode::build_opecode_map().clone()
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
