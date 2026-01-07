pub mod bus;
pub mod cpu;
pub mod debug;
pub mod mem;
pub mod tracer;

use thiserror::Error;

pub use bus::{BusDevice, ReadResult};

#[derive(Debug, Error)]
pub enum EmuError {
    #[error("Normal emulation stop")]
    StopEmulation,
    #[error("Cycle limit reached")]
    CycleLimitReached,
    #[error("Illegal CPU opcode: 0x{0:02X}")]
    IllegalCpuOpcode(u8),
    #[error("Test ROM reported failure with code {0}")]
    TestROMFailure(u8),
}

pub type EmuResult<T> = Result<T, EmuError>;
