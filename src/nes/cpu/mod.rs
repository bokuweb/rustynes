use nes::bus::cpu_bus::CpuBus;

pub struct Cpu;


impl Cpu {
    pub fn new() -> Cpu {
        Cpu
    }

    pub fn run(&self, mut bus: CpuBus) {
        bus.write(0, 100);
        println!("{}", bus.read(0));
    }
}
