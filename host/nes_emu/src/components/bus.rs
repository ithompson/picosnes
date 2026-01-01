use super::EmuResult;

#[derive(Debug, PartialEq, Eq)]
pub enum ReadResult {
    Data(u8),
    OpenBus,
}

pub trait BusDevice {
    fn bus_read(&mut self, addr: u32) -> EmuResult<ReadResult>;
    fn bus_write(&mut self, addr: u32, data: u8) -> EmuResult<()>;

    fn start_of_simulation(&mut self) -> EmuResult<()> {
        Ok(())
    }

    fn end_of_simulation(&mut self) {}
}

#[derive(Debug, PartialEq, Eq)]
struct AddrMapping {
    init_start: u32,
    trg_start: u32,
    len: u32,
}

impl AddrMapping {
    pub fn matches(&self, addr: u32) -> bool {
        addr >= self.init_start && addr.wrapping_sub(self.init_start) < self.len
    }

    pub fn translate(&self, addr: u32) -> u32 {
        addr.wrapping_sub(self.init_start)
            .wrapping_add(self.trg_start)
    }
}

pub struct GenericRouter {
    devices: Vec<(AddrMapping, Box<dyn BusDevice>)>,
}

impl GenericRouter {
    pub fn new() -> Self {
        GenericRouter {
            devices: Vec::new(),
        }
    }

    pub fn add_device(
        &mut self,
        init_start: u32,
        trg_start: u32,
        len: u32,
        device: Box<dyn BusDevice>,
    ) {
        self.devices.push((
            AddrMapping {
                init_start,
                trg_start,
                len,
            },
            device,
        ));
    }

    fn find_device(&mut self, addr: u32) -> Option<&mut (AddrMapping, Box<dyn BusDevice>)> {
        self.devices
            .iter_mut()
            .find(|(range, _)| range.matches(addr))
    }
}

impl BusDevice for GenericRouter {
    fn bus_read(&mut self, addr: u32) -> EmuResult<ReadResult> {
        if let Some((range, device)) = self.find_device(addr) {
            device.bus_read(range.translate(addr))
        } else {
            Ok(ReadResult::OpenBus)
        }
    }

    fn bus_write(&mut self, addr: u32, data: u8) -> EmuResult<()> {
        if let Some((range, device)) = self.find_device(addr) {
            device.bus_write(range.translate(addr), data)
        } else {
            Ok(())
        }
    }

    fn start_of_simulation(&mut self) -> EmuResult<()> {
        for (_, device) in self.devices.iter_mut() {
            device.start_of_simulation()?;
        }
        Ok(())
    }

    fn end_of_simulation(&mut self) {
        for (_, device) in self.devices.iter_mut() {
            device.end_of_simulation();
        }
    }
}

pub struct MirroringWrapper<T: BusDevice> {
    device: T,
    addr_mask: u32,
}

impl<T: BusDevice> MirroringWrapper<T> {
    pub fn new(device: T, addr_bits: usize) -> Self {
        MirroringWrapper {
            device,
            addr_mask: (1 << addr_bits) - 1,
        }
    }
}

impl<T: BusDevice> BusDevice for MirroringWrapper<T> {
    fn bus_read(&mut self, addr: u32) -> EmuResult<ReadResult> {
        let mirrored_addr = addr & self.addr_mask;
        self.device.bus_read(mirrored_addr)
    }

    fn bus_write(&mut self, addr: u32, data: u8) -> EmuResult<()> {
        let mirrored_addr = addr & self.addr_mask;
        self.device.bus_write(mirrored_addr, data)
    }

    fn start_of_simulation(&mut self) -> EmuResult<()> {
        self.device.start_of_simulation()
    }

    fn end_of_simulation(&mut self) {
        self.device.end_of_simulation();
    }
}
