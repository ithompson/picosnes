use super::{BusDevice, EmuResult, ReadResult};

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
    fn bus_read(&mut self, addr: u32) -> EmuResult<ReadResult> {
        let addr = addr as usize;
        if addr < self.memory.len() {
            Ok(ReadResult::Data(self.memory[addr]))
        } else {
            Ok(ReadResult::OpenBus)
        }
    }

    fn bus_write(&mut self, addr: u32, data: u8) -> EmuResult<()> {
        let addr = addr as usize;
        if addr < self.memory.len() {
            self.memory[addr] = data;
        }
        Ok(())
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
    fn bus_read(&mut self, addr: u32) -> EmuResult<ReadResult> {
        let addr = addr as usize;
        if addr < self.contents.len() {
            Ok(ReadResult::Data(self.contents[addr]))
        } else {
            Ok(ReadResult::OpenBus)
        }
    }

    fn bus_write(&mut self, _addr: u32, _data: u8) -> EmuResult<()> {
        // ROM is read-only, so we ignore writes
        Ok(())
    }
}
