mod opcodes;
mod ops;
mod sequences;

use std::fmt;

use sequences::{CpuCycle, MemCycle};

use super::tracer::{TraceComponentId, TraceableReg, TraceableValue, Tracer};
use opcodes::OPCODE_TABLE;
use ops::OpFunc;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
struct ArchPSR {
    n: bool,
    z: bool,
    c: bool,
    v: bool,
    i: bool,
    d: bool,
}

impl ArchPSR {
    const C_BIT: u8 = 0;
    const Z_BIT: u8 = 1;
    const I_BIT: u8 = 2;
    const D_BIT: u8 = 3;
    const B_BIT: u8 = 4;
    const V_BIT: u8 = 6;
    const N_BIT: u8 = 7;

    const C_MASK: u8 = 1 << Self::C_BIT;
    const Z_MASK: u8 = 1 << Self::Z_BIT;
    const I_MASK: u8 = 1 << Self::I_BIT;
    const D_MASK: u8 = 1 << Self::D_BIT;
    const B_MASK: u8 = 1 << Self::B_BIT;
    const V_MASK: u8 = 1 << Self::V_BIT;
    const N_MASK: u8 = 1 << Self::N_BIT;

    fn from_stk_u8(value: u8) -> Self {
        Self {
            n: value & Self::N_MASK != 0,
            z: value & Self::Z_MASK != 0,
            c: value & Self::C_MASK != 0,
            v: value & Self::V_MASK != 0,
            i: value & Self::I_MASK != 0,
            d: value & Self::D_MASK != 0,
        }
    }

    fn as_stk_u8(&self, b: bool) -> u8 {
        let mut value = 1 << 5; // Unused bit 5 is always 1
        if self.n {
            value |= Self::N_MASK;
        }
        if self.z {
            value |= Self::Z_MASK;
        }
        if self.c {
            value |= Self::C_MASK;
        }
        if self.v {
            value |= Self::V_MASK;
        }
        if self.i {
            value |= Self::I_MASK;
        }
        if self.d {
            value |= Self::D_MASK;
        }
        if b {
            value |= Self::B_MASK;
        }
        value
    }

    fn with_nz(self, n: bool, z: bool) -> Self {
        Self { n, z, ..self }
    }

    fn with_nz_from_value(self, value: u8) -> Self {
        self.with_nz(value & 0x80 != 0, value == 0)
    }

    fn with_nzc(self, n: bool, z: bool, c: bool) -> Self {
        Self { n, z, c, ..self }
    }

    fn with_nzc_from_value(self, value: u8, c: bool) -> Self {
        self.with_nzc(value & 0x80 != 0, value == 0, c)
    }

    fn with_nzv(self, n: bool, z: bool, v: bool) -> Self {
        Self { n, z, v, ..self }
    }

    fn with_nzv_from_value(self, value: u8, v: bool) -> Self {
        self.with_nzv(value & 0x80 != 0, value == 0, v)
    }

    fn with_nzcv(self, n: bool, z: bool, c: bool, v: bool) -> Self {
        Self { n, z, c, v, ..self }
    }

    fn with_nzcv_from_value(self, value: u8, c: bool, v: bool) -> Self {
        self.with_nzcv(value & 0x80 != 0, value == 0, c, v)
    }

    fn with_c(self, c: bool) -> Self {
        Self { c, ..self }
    }

    fn with_d(self, d: bool) -> Self {
        Self { d, ..self }
    }

    fn with_i(self, i: bool) -> Self {
        Self { i, ..self }
    }

    fn with_v(self, v: bool) -> Self {
        Self { v, ..self }
    }
}

impl TraceableValue for ArchPSR {
    fn fmt_trace(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}1.{}{}{}{}",
            if self.n { 'N' } else { '.' },
            if self.v { 'V' } else { '.' },
            if self.d { 'D' } else { '.' },
            if self.i { 'I' } else { '.' },
            if self.z { 'Z' } else { '.' },
            if self.c { 'C' } else { '.' },
        )
    }
}

#[derive(Debug)]
struct ArchRegs<'t> {
    a: TraceableReg<'t, u8>,
    x: TraceableReg<'t, u8>,
    y: TraceableReg<'t, u8>,
    p: TraceableReg<'t, ArchPSR>,
    s: TraceableReg<'t, u8>,
    pc: TraceableReg<'t, u16>,
}

impl<'t> ArchRegs<'t> {
    fn new(tracer: &'t Tracer, component_id: TraceComponentId) -> Self {
        Self {
            a: TraceableReg::new("A", tracer, component_id),
            x: TraceableReg::new("X", tracer, component_id),
            y: TraceableReg::new("Y", tracer, component_id),
            p: TraceableReg::new("P", tracer, component_id),
            s: TraceableReg::new("S", tracer, component_id),
            pc: TraceableReg::new("PC", tracer, component_id),
        }
    }
}

#[derive(Debug, Default)]
struct InternalRegs {
    tmp_lo: u8,
    tmp_hi: u8,
    dat: u8,
    rd_val: u8,
}

pub struct Cpu6502<'a> {
    regs: ArchRegs<'a>,
    internal: InternalRegs,
    sequence: &'static [CpuCycle],
    op_func: OpFunc,

    tracer: &'a Tracer,
    trace_component: TraceComponentId,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BusAccess {
    Read(u16),
    Write(u16, u8),
}

impl<'a> Cpu6502<'a> {
    pub fn new(tracer: &'a Tracer) -> Self {
        let trace_component = tracer.add_component("CPU");
        Cpu6502 {
            regs: ArchRegs::new(tracer, trace_component),
            internal: Default::default(),
            op_func: |_, _| {},
            sequence: sequences::RESET_SEQUENCE,
            tracer,
            trace_component,
        }
    }

    pub fn trace_component_id(&self) -> TraceComponentId {
        self.trace_component
    }

    pub fn set_nmi(&mut self, active: bool) {
        // Placeholder implementation
    }

    pub fn set_irq(&mut self, active: bool) {
        // Placeholder implementation
    }

    pub fn reset(&mut self) {
        self.sequence = sequences::RESET_SEQUENCE;
    }

    pub fn tick(&mut self, data_bus: u8) -> BusAccess {
        self.internal.rd_val = data_bus;

        if self.sequence.is_empty() {
            self.sequence = sequences::DISPATCH_SEQUENCE;
        }

        let (action, mem_cycle) = self.sequence.first().unwrap();
        self.sequence = &self.sequence[1..];

        self.tracer
            .trace_seq_action(self.trace_component, action.trace_name);
        (action.action_func)(self);

        match mem_cycle {
            MemCycle::IncReadPC => {
                self.regs.pc.set(self.regs.pc.get().wrapping_add(1));
                BusAccess::Read(self.regs.pc.get())
            }
            MemCycle::ReadPC => BusAccess::Read(self.regs.pc.get()),
            MemCycle::IncReadTmp => {
                self.regs.pc.set(self.regs.pc.get().wrapping_add(1));
                BusAccess::Read(self.internal.tmp_lo as u16 | ((self.internal.tmp_hi as u16) << 8))
            }
            MemCycle::ReadTmp => {
                BusAccess::Read(self.internal.tmp_lo as u16 | ((self.internal.tmp_hi as u16) << 8))
            }
            MemCycle::WriteTmp => BusAccess::Write(
                self.internal.tmp_lo as u16 | ((self.internal.tmp_hi as u16) << 8),
                self.internal.dat,
            ),
            MemCycle::IncReadStk => {
                self.regs.pc.set(self.regs.pc.get().wrapping_add(1));
                BusAccess::Read(0x0100 | (self.regs.s.get() as u16))
            }
            MemCycle::ReadStk => BusAccess::Read(0x0100 | (self.regs.s.get() as u16)),
            MemCycle::PushStk => {
                let sp = self.regs.s.get();
                self.regs.s.set(sp.wrapping_sub(1));
                BusAccess::Write(0x0100 | (sp as u16), self.internal.dat)
            }
            MemCycle::PopStk => {
                let sp = self.regs.s.get().wrapping_add(1);
                self.regs.s.set(sp);
                BusAccess::Read(0x0100 | (sp as u16))
            }
        }
    }

    fn dispatch(&mut self, opcode: u8) {
        if let Some(opdesc) = &OPCODE_TABLE[opcode as usize] {
            self.tracer.trace_instr(
                self.trace_component,
                self.regs.pc.get(),
                opdesc.code,
                opdesc.name,
            );
            self.sequence = opdesc.sequence;
            self.op_func = opdesc.op_func;
        } else {
            panic!("Invalid opcode: {:02X}", opcode);
        }
    }
}
