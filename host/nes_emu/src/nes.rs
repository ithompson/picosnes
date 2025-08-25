use crate::components::{
    BusDevice, GenericRouter, ReadResult,
    cpu_6502::{BusAccess, Cpu6502},
    tracer::Tracer,
};

pub struct NESSystem<'t> {
    cpu: Cpu6502<'t>,
    cpu_bus: GenericRouter<'t>,
    tracer: &'t Tracer,
}

impl<'t> NESSystem<'t> {
    pub fn new(tracer: &'t Tracer) -> Self {
        NESSystem {
            cpu: Cpu6502::new(tracer),
            cpu_bus: GenericRouter::new(),
            tracer,
        }
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
