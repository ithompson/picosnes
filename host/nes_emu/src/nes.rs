use crate::components::{
    BusDevice, GenericRouter, MirroringWrapper, RAMDevice, ROMDevice, ReadResult,
    cpu::{BusAccess, Cpu6502},
    tracer::Tracer,
};
use crate::nes_file::NesFile;

pub struct NESSystem<'t> {
    cpu: Cpu6502<'t>,
    cpu_bus: GenericRouter,
    tracer: &'t Tracer,
}

impl<'t> NESSystem<'t> {
    pub fn new(tracer: &'t Tracer, rom: NesFile) -> Self {
        let mut system = NESSystem {
            cpu: Cpu6502::new(tracer),
            cpu_bus: GenericRouter::new(),
            tracer,
        };
        // Internal RAM: 0x0000 - 0x1FFF, mirroring every 0x0800 bytes
        let internal_ram = RAMDevice::new(0x800);
        let mirrorred_internal_ram = MirroringWrapper::new(internal_ram, 11);
        system
            .cpu_bus
            .add_device(0x0000, 0x0000, 0x2000, Box::new(mirrorred_internal_ram));

        // PPU: 0x2000 - 0x3FFF, mirroring every 0x0008 bytes
        let fake_ppu = RAMDevice::new(0x8);
        let mirrored_ppu = MirroringWrapper::new(fake_ppu, 3);
        system
            .cpu_bus
            .add_device(0x2000, 0x0, 0x2000, Box::new(mirrored_ppu));

        // APU and IO: 0x4000 - 0x4017
        let fake_apu = RAMDevice::new(0x18);
        system
            .cpu_bus
            .add_device(0x4000, 0x0, 0x18, Box::new(fake_apu));

        let prg_ram = Box::new(RAMDevice::new(rom.prg_ram_size));
        system.cpu_bus.add_device(0x6000, 0x0, 0x1FFF, prg_ram);

        // FIXME: temporary assumptions that work for the test ROMs
        // Mapper 1, PRG-ROM small enough that no bank switching is needed
        assert!(rom.mapper.id == 0);
        assert!(rom.prg_rom.len() == 0x8000);
        let prg_rom = Box::new(ROMDevice::new(rom.prg_rom));
        system.cpu_bus.add_device(0x8000, 0x0000, 0x7FFF, prg_rom);

        system
    }

    pub fn run(&mut self) {
        // Main emulation loop
        let mut data_bus = 0;
        loop {
            let bus_op = self.cpu.tick(data_bus);
            match bus_op {
                BusAccess::Read(addr) => {
                    match self.cpu_bus.bus_read(addr as u32) {
                        ReadResult::Data(value) => data_bus = value,
                        ReadResult::OpenBus => {}
                    }

                    self.tracer
                        .trace_mem_read(self.cpu.trace_component_id(), addr, data_bus);
                }
                BusAccess::Write(addr, value) => {
                    // Handle write operation
                    data_bus = value;
                    self.cpu_bus.bus_write(addr as u32, value);

                    self.tracer
                        .trace_mem_write(self.cpu.trace_component_id(), addr, value);
                }
            }
        }
    }
}
