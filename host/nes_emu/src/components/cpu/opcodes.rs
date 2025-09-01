use super::ops;
use super::sequences as seq;

#[derive(Debug)]
pub struct Opcode {
    pub code: u8,
    pub name: &'static str,
    pub op_func: ops::OpFunc,
    pub sequence: &'static [seq::CpuCycle],
}

pub static OPCODE_TABLE: [Option<Opcode>; 256] = {
    let mut a = [const { None }; 256];

    a[0x0A] = Some(Opcode {
        code: 0x0A,
        name: "ASL A",
        op_func: ops::asl,
        sequence: &seq::ACC_RMW_SEQUENCE,
    });

    a[0x4C] = Some(Opcode {
        code: 0x4C,
        name: "JMP $addr",
        op_func: ops::nop,
        sequence: &seq::ABS_JMP_SEQUENCE,
    });

    a[0xE6] = Some(Opcode {
        code: 0xE6,
        name: "INC $zp",
        op_func: ops::inc,
        sequence: &seq::ZP_RMW_SEQUENCE,
    });

    a
};
