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
    SET_PC => |cpu| {
        cpu.regs
            .pc
            .set(((cpu.internal.rd_val as u16) << 8) | (cpu.internal.tmp_lo as u16));
    },
    OP_NOARG => |cpu| {
        let mut val = 0;
        (cpu.op_func)(cpu, &mut val);
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
    SET_DAT_PC_LO => |cpu| {
        cpu.internal.dat = (cpu.regs.pc.get() & 0x00FF) as u8;
    },
    SET_DAT_PC_HI => |cpu| {
        cpu.internal.dat = (cpu.regs.pc.get() >> 8) as u8;
    },
    ADD_X_TO_TMP => |cpu| {
        cpu.internal.tmp_lo = cpu.internal.tmp_lo.wrapping_add(cpu.regs.x.get());
    },
    ADD_X_TO_TMP_AND_OP_DAT => |cpu| {
        cpu.internal.tmp_lo = cpu.internal.tmp_lo.wrapping_add(cpu.regs.x.get());
        let mut val = 0;
        (cpu.op_func)(cpu, &mut val);
        cpu.internal.dat = val;
    },
}

macro_rules! seq {
    ($name:ident => [$(($action:ident, $mem_cycle:ident)),* $(,)?]) => {
        pub static $name: &[CpuCycle] = &[$((&$action, MemCycle::$mem_cycle)),*];
    };
}

macro_rules! unimplemented_seq {
    ($name:ident) => {
        pub static $name: &[CpuCycle] = &[(
            &CpuAction {
                trace_name: stringify!($name),
                action_func: |_| unimplemented!("Unimplemented op sequence {}", stringify!($name)),
            },
            MemCycle::ReadPC,
        )];
    };
}

seq!(RESET_SEQUENCE => [(RESET, ReadPC)]);
seq!(DISPATCH_SEQUENCE => [(DISPATCH, IncReadPC)]);

seq!(ABS_JMP_SEQUENCE => [
    (SET_TMP_LO, IncReadPC),
    (SET_PC, ReadPC),
]);
seq!(ABS_JSR_SEQUENCE => [
    (SET_TMP_LO, IncReadStk),
    (SET_DAT_PC_HI, PushStk),
    (SET_DAT_PC_LO, PushStk),
    (NOP, ReadPC),
    (SET_PC, ReadPC),
]);
unimplemented_seq!(ABS_READ_SEQUENCE);
unimplemented_seq!(ABS_RMW_SEQUENCE);
unimplemented_seq!(ABS_WRITE_SEQUENCE);

unimplemented_seq!(ABSIND_JMP_SEQUENCE);

unimplemented_seq!(ABSX_READ_SEQUENCE);
unimplemented_seq!(ABSX_RMW_SEQUENCE);
unimplemented_seq!(ABSX_WRITE_SEQUENCE);

unimplemented_seq!(ABSY_READ_SEQUENCE);
unimplemented_seq!(ABSY_WRITE_SEQUENCE);

seq!(ACC_RMW_SEQUENCE => [
    (OP_A, ReadPC)
]);

unimplemented_seq!(IMM_READ_SEQUENCE);

unimplemented_seq!(IMP_BRK_SEQUENCE);
unimplemented_seq!(IMP_RTI_SEQUENCE);
unimplemented_seq!(IMP_RTS_SEQUENCE);
seq!(IMP_NOMEM_SEQUENCE => [
    (OP_NOARG, ReadPC)
]);
seq!(IMP_PUSH_SEQUENCE => [
    (OP_DAT, PushStk),
    (NOP, ReadPC),
]);
seq!(IMP_POP_SEQUENCE => [
    (NOP, ReadStk),
    (NOP, PopStk),
    (OP_RD_VAL, ReadPC),
]);

unimplemented_seq!(INDX_READ_SEQUENCE);
unimplemented_seq!(INDX_WRITE_SEQUENCE);

unimplemented_seq!(INDY_READ_SEQUENCE);
unimplemented_seq!(INDY_WRITE_SEQUENCE);

unimplemented_seq!(REL_BRANCH_SEQUENCE);

unimplemented_seq!(ZP_READ_SEQUENCE);
seq!(ZP_RMW_SEQUENCE => [
    (SET_TMP_ZP, IncReadTmp),
    (SET_DAT, WriteTmp),
    (OP_DAT, WriteTmp),
    (NOP, ReadPC),
]);
unimplemented_seq!(ZP_WRITE_SEQUENCE);

seq!(ZPX_READ_SEQUENCE => [
    (SET_TMP_ZP, IncReadTmp),
    (ADD_X_TO_TMP, ReadTmp),
    (OP_RD_VAL, ReadPC),
]);
seq!(ZPX_RMW_SEQUENCE => [
    (SET_TMP_ZP, IncReadTmp),
    (ADD_X_TO_TMP, ReadTmp),
    (SET_DAT, WriteTmp),
    (OP_DAT, WriteTmp),
    (NOP, ReadPC),
]);
seq!(ZPX_WRITE_SEQUENCE => [
    (SET_TMP_ZP, IncReadTmp),
    (ADD_X_TO_TMP_AND_OP_DAT, WriteTmp),
    (NOP, ReadPC),
]);

unimplemented_seq!(ZPY_READ_SEQUENCE);
unimplemented_seq!(ZPY_WRITE_SEQUENCE);
