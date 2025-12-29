mod actions;
mod sequence_tables;

pub use sequence_tables::*;

use super::{Cpu6502, CpuError};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemCycle {
    IncReadPC,
    ReadPC,
    IncReadTmp,
    ReadTmp,
    IncWriteTmp,
    WriteTmp,
    IncReadStk,
    ReadStk,
    IncPushStk,
    PushStk,
    PopStk,
}

#[derive(Debug, Copy, Clone)]
pub struct CpuAction {
    pub trace_name: &'static str,
    pub action_func: fn(&mut Cpu6502) -> Result<(), CpuError>,
}

pub type CpuCycle = (&'static CpuAction, MemCycle);

macro_rules! action_defs {
    () => {};
    ($name:ident => || $body:block, $($tail:tt)*) => {
        pub static $name: CpuAction = CpuAction {
            trace_name: stringify!($name),
            action_func: |_| $body,
        };
        action_defs!($($tail)*);
    };
    ($name:ident => |$arg:ident| $body:block, $($tail:tt)*) => {
        pub static $name: CpuAction = CpuAction {
            trace_name: stringify!($name),
            action_func: |$arg| $body,
        };
        action_defs!($($tail)*);
    };
}
use action_defs;

macro_rules! seq {
    ($name:ident => [$(($action:ident, $mem_cycle:ident)),* $(,)?]) => {
        pub static $name: &[CpuCycle] = &[$((&actions::$action, MemCycle::$mem_cycle)),*];
    };
}
use seq;
