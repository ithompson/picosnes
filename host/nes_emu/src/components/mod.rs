pub mod bus;
pub mod cpu;
pub mod debug;
pub mod mem;
pub mod tracer;

use thiserror::Error;

pub use bus::{BusDevice, ReadResult};

#[derive(Debug, Error)]
pub enum EmuError {
    #[error("CPU error: {0}")]
    Cpu(#[from] cpu::CpuError),
    #[error("Normal emulation stop")]
    StopEmulation,
    #[error("Test ROM reported failure with code {0}")]
    TestROMFailure(u8),
}

pub type EmuResult<T> = Result<T, EmuError>;
