pub mod cpu_6502;
pub mod tracer;

use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub enum ReadResult {
    Data(u8),
    OpenBus,
}

pub trait BusDevice {
    fn bus_read(&mut self, addr: u16) -> ReadResult;
    fn bus_write(&mut self, addr: u16, data: u8);
}

#[derive(Debug, PartialEq, Eq)]
pub struct AddrMapping {
    pub init_start: u16,
    pub trg_start: u16,
    pub len: u16,
}

impl AddrMapping {
    pub fn matches(&self, addr: u16) -> bool {
        addr >= self.init_start && addr.wrapping_sub(self.init_start) < self.len
    }

    pub fn translate(&self, addr: u16) -> u16 {
        addr.wrapping_sub(self.init_start)
            .wrapping_add(self.trg_start)
    }
}

pub struct GenericRouter<'a> {
    devices: Vec<(AddrMapping, &'a RefCell<dyn BusDevice>)>,
}

impl<'a> GenericRouter<'a> {
    pub fn new() -> Self {
        GenericRouter {
            devices: Vec::new(),
        }
    }

    pub fn add_device(
        &mut self,
        init_start: u16,
        trg_start: u16,
        len: u16,
        device: &'a RefCell<dyn BusDevice>,
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

    fn find_device(&self, addr: u16) -> Option<&(AddrMapping, &'a RefCell<dyn BusDevice>)> {
        self.devices.iter().find(|(range, _)| range.matches(addr))
    }
}

impl BusDevice for GenericRouter<'_> {
    fn bus_read(&mut self, addr: u16) -> ReadResult {
        if let Some((range, device)) = self.find_device(addr) {
            device.borrow_mut().bus_read(range.translate(addr))
        } else {
            ReadResult::OpenBus
        }
    }

    fn bus_write(&mut self, addr: u16, data: u8) {
        self.find_device(addr)
            .map(|(range, device)| device.borrow_mut().bus_write(range.translate(addr), data));
    }
}

pub struct RAMDevice {
    base_addr: u16,
    memory: Vec<u8>,
}

impl RAMDevice {
    pub fn new(base_addr: u16, size: usize) -> Self {
        RAMDevice {
            base_addr,
            memory: vec![0; size],
        }
    }
}

impl BusDevice for RAMDevice {
    fn bus_read(&mut self, addr: u16) -> ReadResult {
        let offset = addr.wrapping_sub(self.base_addr) as usize;
        if offset < self.memory.len() {
            ReadResult::Data(self.memory[offset])
        } else {
            ReadResult::OpenBus
        }
    }

    fn bus_write(&mut self, addr: u16, data: u8) {
        let offset = addr.wrapping_sub(self.base_addr) as usize;
        if offset < self.memory.len() {
            self.memory[offset] = data;
        }
    }
}

struct ROMDevice {
    base_addr: u16,
    contents: Vec<u8>,
}

impl ROMDevice {
    pub fn new(base_addr: u16, contents: Vec<u8>) -> Self {
        ROMDevice {
            base_addr,
            contents,
        }
    }
}

impl BusDevice for ROMDevice {
    fn bus_read(&mut self, addr: u16) -> ReadResult {
        let offset = addr.wrapping_sub(self.base_addr) as usize;
        if offset < self.contents.len() {
            ReadResult::Data(self.contents[offset])
        } else {
            ReadResult::OpenBus
        }
    }

    fn bus_write(&mut self, _addr: u16, _data: u8) {
        // ROM is read-only, so we ignore writes
    }
}
