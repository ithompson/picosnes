mod op_impls;

#[cfg(test)]
mod test_helpers;

use super::{ArchPSR, ArchRegs};

pub type OpFunc = fn(&mut ArchRegs, &mut u8) -> ();
pub use op_impls::*;
