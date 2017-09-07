use nes::bus::cpu_bus::CpuBus;

pub struct Cpu {
    pub bus: CpuBus,
}

impl Cpu {
    pub fn new(bus: CpuBus) -> Cpu {
        Cpu { bus }
    }

    pub fn run(&self) {
        println!("{}", self.bus.read(0));
    }
}
