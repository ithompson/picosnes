pub mod cpu;
pub mod tracer;

#[derive(Debug, PartialEq, Eq)]
pub enum ReadResult {
    Data(u8),
    OpenBus,
}

pub trait BusDevice {
    fn bus_read(&mut self, addr: u32) -> ReadResult;
    fn bus_write(&mut self, addr: u32, data: u8);
}

#[derive(Debug, PartialEq, Eq)]
pub struct AddrMapping {
    pub init_start: u32,
    pub trg_start: u32,
    pub len: u32,
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
    fn bus_read(&mut self, addr: u32) -> ReadResult {
        if let Some((range, device)) = self.find_device(addr) {
            device.bus_read(range.translate(addr))
        } else {
            ReadResult::OpenBus
        }
    }

    fn bus_write(&mut self, addr: u32, data: u8) {
        if let Some((range, device)) = self.find_device(addr) {
            device.bus_write(range.translate(addr), data);
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
    fn bus_read(&mut self, addr: u32) -> ReadResult {
        let mirrored_addr = addr & self.addr_mask;
        self.device.bus_read(mirrored_addr)
    }

    fn bus_write(&mut self, addr: u32, data: u8) {
        let mirrored_addr = addr & self.addr_mask;
        self.device.bus_write(mirrored_addr, data);
    }
}

pub struct RAMDevice {
    memory: Vec<u8>,
}

impl RAMDevice {
    pub fn new(size: usize) -> Self {
        RAMDevice {
            memory: vec![0; size],
        }
    }
}

impl BusDevice for RAMDevice {
    fn bus_read(&mut self, addr: u32) -> ReadResult {
        let addr = addr as usize;
        if addr < self.memory.len() {
            ReadResult::Data(self.memory[addr])
        } else {
            ReadResult::OpenBus
        }
    }

    fn bus_write(&mut self, addr: u32, data: u8) {
        let addr = addr as usize;
        if addr < self.memory.len() {
            self.memory[addr] = data;
        }
    }
}

pub struct ROMDevice {
    contents: Vec<u8>,
}

impl ROMDevice {
    pub fn new(contents: Vec<u8>) -> Self {
        ROMDevice { contents }
    }
}

impl BusDevice for ROMDevice {
    fn bus_read(&mut self, addr: u32) -> ReadResult {
        let addr = addr as usize;
        if addr < self.contents.len() {
            ReadResult::Data(self.contents[addr])
        } else {
            ReadResult::OpenBus
        }
    }

    fn bus_write(&mut self, _addr: u32, _data: u8) {
        // ROM is read-only, so we ignore writes
    }
}
