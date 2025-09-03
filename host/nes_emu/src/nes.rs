use crate::components::{
    BusDevice, GenericRouter, RAMDevice, ReadResult,
    cpu::{BusAccess, Cpu6502},
    tracer::Tracer,
};

pub struct NESSystem<'t> {
    cpu: Cpu6502<'t>,
    cpu_bus: GenericRouter,
    tracer: &'t Tracer,
}

impl<'t> NESSystem<'t> {
    pub fn new(tracer: &'t Tracer) -> Self {
        let mut system = NESSystem {
            cpu: Cpu6502::new(tracer),
            cpu_bus: GenericRouter::new(),
            tracer,
        };
        let sysmem = Box::new(RAMDevice::new(0x4000)); // 16KB of system RAM
        system.cpu_bus.add_device(0x0000, 0x0000, 0x4000, sysmem);
        let bootmem = Box::new(RAMDevice::new(0x10)); // 16 bytes of boot RAM
        system.cpu_bus.add_device(0xFFF0, 0x0000, 0x0010, bootmem);

        // Pre-populate a tiny program
        system.cpu_bus.bus_write(0x200, 0xE6); // INC $zp
        system.cpu_bus.bus_write(0x201, 0x00); // zp:0x00
        system.cpu_bus.bus_write(0x202, 0x4C); // JMP $abs
        system.cpu_bus.bus_write(0x203, 0x00); // abs:lo
        system.cpu_bus.bus_write(0x204, 0x02); // abs:hi
        // Reset vector
        system.cpu_bus.bus_write(0xFFFC, 0x00); // Reset vector lo
        system.cpu_bus.bus_write(0xFFFD, 0x02); // Reset vector hi

        system
    }

    pub fn run(&mut self) {
        // Main emulation loop
        let mut data_bus = 0;
        loop {
            let bus_op = self.cpu.tick(data_bus);
            match bus_op {
                BusAccess::Read(addr) => {
                    match self.cpu_bus.bus_read(addr) {
                        ReadResult::Data(value) => data_bus = value,
                        ReadResult::OpenBus => {}
                    }

                    self.tracer
                        .trace_mem_read(self.cpu.trace_component_id(), addr, data_bus);
                }
                BusAccess::Write(addr, value) => {
                    // Handle write operation
                    data_bus = value;
                    self.cpu_bus.bus_write(addr, value);

                    self.tracer
                        .trace_mem_write(self.cpu.trace_component_id(), addr, value);
                }
            }
        }
    }
}
