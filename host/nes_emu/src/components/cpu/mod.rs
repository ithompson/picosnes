mod opcodes;
mod ops;
mod sequences;

use std::fmt;

use sequences::{CpuCycle, MemCycle};

use super::tracer::{TraceElementId, TraceableReg, TraceableValue, Tracer};
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
    fn new(tracer: &'t Tracer, trace_parent: TraceElementId) -> Self {
        Self {
            a: TraceableReg::new("A", tracer, trace_parent),
            x: TraceableReg::new("X", tracer, trace_parent),
            y: TraceableReg::new("Y", tracer, trace_parent),
            p: TraceableReg::new("P", tracer, trace_parent),
            s: TraceableReg::new("S", tracer, trace_parent),
            pc: TraceableReg::new("PC", tracer, trace_parent),
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
    mem_trace_element: TraceElementId,
    seq_trace_element: TraceElementId,
    instr_trace_element: TraceElementId,

    nmi_pending: bool,
    irq_signaled: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BusAccess {
    Read(u16),
    Write(u16, u8),
}

impl<'a> Cpu6502<'a> {
    pub fn new(tracer: &'a Tracer) -> Self {
        let root_trace_element = tracer.register_element("cpu", None);
        let mem_trace_element = tracer.register_element("mem", Some(root_trace_element));
        let regs_trace_element = tracer.register_element("regs", Some(root_trace_element));
        let seq_trace_element = tracer.register_element("seq", Some(root_trace_element));
        let instr_trace_element = tracer.register_element("instr", Some(root_trace_element));

        Cpu6502 {
            regs: ArchRegs::new(tracer, regs_trace_element),
            internal: Default::default(),
            op_func: ops::nop,
            sequence: sequences::RESET_SEQUENCE,
            tracer,
            mem_trace_element,
            seq_trace_element,
            instr_trace_element,

            nmi_pending: false,
            irq_signaled: false,
        }
    }

    pub fn mem_trace_element(&self) -> TraceElementId {
        self.mem_trace_element
    }

    pub fn trigger_nmi(&mut self) {
        self.nmi_pending = true;
    }

    pub fn set_irq_signaled(&mut self, active: bool) {
        self.irq_signaled = active;
    }

    pub fn reset(&mut self) {
        self.sequence = sequences::RESET_SEQUENCE;
        self.nmi_pending = false;
    }

    pub fn tick(&mut self, data_bus: u8) -> BusAccess {
        self.internal.rd_val = data_bus;

        if self.sequence.is_empty() {
            self.sequence = sequences::DISPATCH_SEQUENCE;
        }

        let (action, mem_cycle) = self.sequence.first().unwrap();
        self.sequence = &self.sequence[1..];

        self.tracer.trace_event(
            self.seq_trace_element,
            format_args!("    {}", action.trace_name),
        );
        (action.action_func)(self);

        match mem_cycle {
            MemCycle::IncReadPC => {
                self.regs.pc.update(|pc| pc.wrapping_add(1));
                BusAccess::Read(self.regs.pc.get())
            }
            MemCycle::ReadPC => BusAccess::Read(self.regs.pc.get()),
            MemCycle::IncReadTmp => {
                self.regs.pc.update(|pc| pc.wrapping_add(1));
                BusAccess::Read(self.internal.tmp_lo as u16 | ((self.internal.tmp_hi as u16) << 8))
            }
            MemCycle::ReadTmp => {
                BusAccess::Read(self.internal.tmp_lo as u16 | ((self.internal.tmp_hi as u16) << 8))
            }
            MemCycle::IncWriteTmp => {
                self.regs.pc.update(|pc| pc.wrapping_add(1));
                BusAccess::Write(
                    self.internal.tmp_lo as u16 | ((self.internal.tmp_hi as u16) << 8),
                    self.internal.dat,
                )
            }
            MemCycle::WriteTmp => BusAccess::Write(
                self.internal.tmp_lo as u16 | ((self.internal.tmp_hi as u16) << 8),
                self.internal.dat,
            ),
            MemCycle::IncReadStk => {
                self.regs.pc.update(|pc| pc.wrapping_add(1));
                BusAccess::Read(0x0100 | (self.regs.s.get() as u16))
            }
            MemCycle::ReadStk => BusAccess::Read(0x0100 | (self.regs.s.get() as u16)),
            MemCycle::IncPushStk => {
                self.regs.pc.update(|pc| pc.wrapping_add(1));
                let sp = self.regs.s.get();
                self.regs.s.set(sp.wrapping_sub(1));
                BusAccess::Write(0x0100 | (sp as u16), self.internal.dat)
            }
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
        // FIXME: Strictly speaking this is the wrong way to do IRQ/NMI handling
        // A proper implementation would be to force a BRK opcode, and then the
        // BRK sequence would have the IRQ/NMI checks on the cycle that pushes P
        // to the stack. If IRQ/NMI is detected, the action on that cycle would
        // swap to the IRQ/NMI sequence
        if self.nmi_pending {
            self.nmi_pending = false;
            self.sequence = sequences::NMI_SEQUENCE;
        } else if self.irq_signaled && !self.regs.p.get().i {
            self.sequence = sequences::IRQ_SEQUENCE;
        } else if let Some(opdesc) = &OPCODE_TABLE[opcode as usize] {
            self.tracer.trace_event(
                self.instr_trace_element,
                format_args!(
                    "0x{:04X} 0x{:02X} {}",
                    self.regs.pc.get(),
                    opdesc.code,
                    opdesc.name
                ),
            );
            self.sequence = opdesc.sequence;
            self.op_func = opdesc.op_func;
        } else {
            panic!("Invalid opcode: {:02X}", opcode);
        }
    }

    fn skip_next_cycle(&mut self) {
        self.sequence = &self.sequence[1..];
    }

    fn end_instruction(&mut self) {
        self.sequence = &[];
    }
}
