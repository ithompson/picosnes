use super::Cpu6502;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemCycle {
    IncReadPC,
    ReadPC,
    IncReadTmp,
    ReadTmp,
    WriteTmp,
    IncReadStk,
    ReadStk,
    PushStk,
    PopStk,
}

#[derive(Debug, Copy, Clone)]
pub struct CpuAction {
    pub trace_name: &'static str,
    pub action_func: fn(&mut Cpu6502) -> (),
}

pub type CpuCycle = (&'static CpuAction, MemCycle);

macro_rules! actions {
    () => {};
    ($name:ident => || $body:block, $($tail:tt)*) => {
        static $name: CpuAction = CpuAction {
            trace_name: stringify!($name),
            action_func: |_| $body,
        };
        actions!($($tail)*);
    };
    ($name:ident => |$arg:ident| $body:block, $($tail:tt)*) => {
        static $name: CpuAction = CpuAction {
            trace_name: stringify!($name),
            action_func: |$arg| $body,
        };
        actions!($($tail)*);
    };
}

actions! {
    RESET => |cpu| {
        cpu.regs.pc.set(0x200);
    },
    DISPATCH => |cpu| {
        cpu.dispatch(cpu.internal.rd_val)
    },
    NOP => || {},
    INC_PC_HI => |cpu| {
        let mut pc = cpu.regs.pc.get();
        pc = pc.wrapping_add(0x100);
        cpu.regs.pc.set(pc);
    },
    SET_PC_HI => |cpu| {
        let mut pc = cpu.regs.pc.get();
        pc = (pc & 0x00FF) | ((cpu.internal.rd_val as u16) << 8);
        cpu.regs.pc.set(pc);
    },
    SET_PC => |cpu| {
        cpu.regs
            .pc
            .set(((cpu.internal.rd_val as u16) << 8) | (cpu.internal.tmp_lo as u16));
    },
    SET_PC_LO => |cpu| {
        let mut pc = cpu.regs.pc.get();
        pc = (pc & 0xFF00) | (cpu.internal.rd_val as u16);
        cpu.regs.pc.set(pc);
    },
    SET_PC_LO_INC_TMP => |cpu| {
        let mut pc = cpu.regs.pc.get();
        pc = (pc & 0xFF00) | (cpu.internal.rd_val as u16);
        cpu.regs.pc.set(pc);
        cpu.internal.tmp_lo = cpu.internal.tmp_lo.wrapping_add(1);
    },
    OP_A => |cpu| {
        let mut val = cpu.regs.a.get();
        (cpu.op_func)(cpu, &mut val);
        cpu.regs.a.set(val);
    },
    OP_DAT => |cpu| {
        let mut val = cpu.internal.dat;
        (cpu.op_func)(cpu, &mut val);
        cpu.internal.dat = val;
    },
    OP_RD_VAL => |cpu| {
        let mut val = cpu.internal.rd_val;
        (cpu.op_func)(cpu, &mut val);
        cpu.internal.rd_val = val;
    },
    SET_TMP_LO => |cpu| {
        cpu.internal.tmp_lo = cpu.internal.rd_val;
    },
    SET_TMP_ZP => |cpu| {
        cpu.internal.tmp_lo = cpu.internal.rd_val;
        cpu.internal.tmp_hi = 0;
    },
    SET_DAT => |cpu| {
        cpu.internal.dat = cpu.internal.rd_val;
    },
}

pub static RESET_SEQUENCE: [CpuCycle; 1] = [(&RESET, MemCycle::ReadPC)];
pub static DISPATCH_SEQUENCE: [CpuCycle; 1] = [(&DISPATCH, MemCycle::IncReadPC)];

pub static ACC_RMW_SEQUENCE: [CpuCycle; 1] = [(&OP_A, MemCycle::ReadPC)];
pub static ABS_JMP_SEQUENCE: [CpuCycle; 2] = [
    (&SET_TMP_LO, MemCycle::IncReadPC),
    (&SET_PC, MemCycle::ReadPC),
];
pub static ZP_RMW_SEQUENCE: [CpuCycle; 4] = [
    (&SET_TMP_ZP, MemCycle::IncReadTmp),
    (&SET_DAT, MemCycle::WriteTmp),
    (&OP_DAT, MemCycle::WriteTmp),
    (&NOP, MemCycle::ReadPC),
];
