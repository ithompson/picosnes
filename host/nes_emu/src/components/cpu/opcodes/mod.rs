mod opcode_table;

use super::ops;
use super::sequences::{self, CpuCycle};

#[derive(Debug)]
pub struct Opcode {
    pub code: u8,
    pub name: &'static str,
    pub sequence: &'static [CpuCycle],
    pub op_func: ops::OpFunc,
}

macro_rules! opcode {
    ($table:ident, $code:expr, $name:expr, $sequence:ident, $op_func:ident) => {
        $table[$code as usize] = Some(Opcode {
            code: $code,
            name: $name,
            sequence: sequences::$sequence,
            op_func: ops::$op_func,
        });
    };
}
use opcode;

pub use opcode_table::OPCODE_TABLE;
