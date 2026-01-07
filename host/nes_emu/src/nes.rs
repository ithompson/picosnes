use crate::components::{
    BusDevice, EmuError, EmuResult, ReadResult,
    bus::{GenericRouter, MirroringWrapper},
    cpu::{ArchRegs, BusAccess, Cpu6502},
    debug::TestROMMonitor,
    mem::{RAMDevice, ROMDevice},
    tracer::Tracer,
};
use crate::nes_file::NesFile;

pub struct NESSystem<'t> {
    cpu: Cpu6502<'t>,
    cpu_bus: GenericRouter,
    data_bus_state: u8,
    tracer: &'t Tracer,
    tick_count: u64,
}

impl<'t> NESSystem<'t> {
    pub fn new(tracer: &'t Tracer, rom: NesFile) -> Self {
        let mut system = NESSystem {
            cpu: Cpu6502::new(tracer),
            cpu_bus: GenericRouter::new(),
            data_bus_state: 0,
            tracer,
            tick_count: 0,
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

        //let prg_ram = Box::new(RAMDevice::new(rom.prg_ram_size));
        let prg_ram = Box::new(TestROMMonitor::new(RAMDevice::new(0x2000), 0x0));
        system.cpu_bus.add_device(0x6000, 0x0, 0x1FFF, prg_ram);

        // FIXME: temporary assumptions that work for the test ROMs
        // Mapper 1, PRG-ROM small enough that no bank switching is needed
        assert!(rom.mapper.id == 0);
        assert!(rom.prg_rom.len() == 0x8000);
        let prg_rom = Box::new(ROMDevice::new(rom.prg_rom));
        system.cpu_bus.add_device(0x8000, 0x0000, 0x7FFF, prg_rom);

        system
    }

    pub fn start_simulation(&mut self) -> EmuResult<()> {
        self.tick_count = 0;
        self.cpu_bus.start_of_simulation()
    }

    pub fn end_simulation(&mut self) {
        self.cpu_bus.end_of_simulation();
    }

    fn run_tick(&mut self) -> EmuResult<()> {
        match self.cpu.tick(self.data_bus_state)? {
            BusAccess::Read(addr) => {
                match self.cpu_bus.bus_read(addr as u32)? {
                    ReadResult::Data(value) => self.data_bus_state = value,
                    ReadResult::OpenBus => {}
                }

                self.tracer.trace_event(
                    self.cpu.mem_trace_element(),
                    format_args!("      RD 0x{:04X} => 0x{:02X}", addr, self.data_bus_state),
                );
            }
            BusAccess::Write(addr, value) => {
                // Handle write operation
                self.data_bus_state = value;
                self.cpu_bus.bus_write(addr as u32, value)?;

                self.tracer.trace_event(
                    self.cpu.mem_trace_element(),
                    format_args!("      WR 0x{:04X} => 0x{:02X}", addr, self.data_bus_state),
                );
            }
        }
        self.tick_count += 1;
        Ok(())
    }

    pub fn get_regs(&self) -> &ArchRegs<'t> {
        self.cpu.get_regs()
    }

    pub fn get_tick_count(&self) -> u64 {
        self.tick_count
    }

    pub fn run(&mut self, tick_limit: Option<u64>) -> EmuResult<()> {
        loop {
            if let Some(limit) = tick_limit {
                if self.tick_count >= limit {
                    return Err(EmuError::CycleLimitReached);
                }
            }
            self.run_tick()?;
        }
    }
}
