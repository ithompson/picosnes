use core::fmt;
use std::cell::RefCell;

pub trait TraceableValue {
    fn fmt_trace(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

pub struct TraceDisplay<'a, T: TraceableValue + ?Sized>(pub &'a T);
impl<'a, T: TraceableValue + ?Sized> fmt::Display for TraceDisplay<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt_trace(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraceComponentId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraceRegId {
    component: TraceComponentId,
    reg: usize,
}

#[derive(Debug)]
struct TraceReg {
    name: String,
}

#[derive(Debug)]
struct TraceComponent {
    name: String,
    regs: Vec<TraceReg>,
}

#[derive(Debug)]
struct TraceData {
    components: Vec<TraceComponent>,
}

#[derive(Debug)]
pub struct Tracer {
    data: RefCell<TraceData>,
}

impl Tracer {
    pub fn new() -> Self {
        Tracer {
            data: RefCell::new(TraceData {
                components: Vec::new(),
            }),
        }
    }

    pub fn add_component(&self, name: &str) -> TraceComponentId {
        let mut data = self.data.borrow_mut();
        let component = TraceComponent {
            name: name.to_string(),
            regs: Vec::new(),
        };
        data.components.push(component);
        TraceComponentId(data.components.len() - 1)
    }

    pub fn add_register(&self, name: &str, component: TraceComponentId) -> TraceRegId {
        let component_data = &mut self.data.borrow_mut().components[component.0];
        let reg = TraceReg {
            name: name.to_string(),
        };
        component_data.regs.push(reg);
        TraceRegId {
            component,
            reg: component_data.regs.len() - 1,
        }
    }

    fn reg_enabled_for_trace(&self, reg: TraceRegId) -> bool {
        if !self.component_enabled_for_trace(reg.component) {
            return false;
        }
        let component = &self.data.borrow().components[reg.component.0];
        component.regs.get(reg.reg).is_some()
    }

    fn component_enabled_for_trace(&self, component: TraceComponentId) -> bool {
        let component_data = &self.data.borrow().components[component.0];
        !component_data.regs.is_empty()
    }

    pub fn trace_reg_write<T: TraceableValue>(&self, reg: TraceRegId, value: T) {
        if !self.reg_enabled_for_trace(reg) {
            return;
        }

        let component_data = &self.data.borrow().components[reg.component.0];
        let reg_data = &component_data.regs[reg.reg];
        println!(
            "[TRACE][{}][REG] {} = {}",
            component_data.name,
            reg_data.name,
            TraceDisplay(&value)
        );
    }

    pub fn trace_mem_read<T: TraceableValue>(
        &self,
        initiator: TraceComponentId,
        addr: u16,
        value: T,
    ) {
        if !self.component_enabled_for_trace(initiator) {
            return;
        }
        let component_data = &self.data.borrow().components[initiator.0];
        println!(
            "[TRACE][{}][MEM] RD 0x{:04X} => {}",
            component_data.name,
            addr,
            TraceDisplay(&value)
        );
    }
    pub fn trace_mem_write<T: TraceableValue>(
        &self,
        initiator: TraceComponentId,
        addr: u16,
        value: T,
    ) {
        if !self.component_enabled_for_trace(initiator) {
            return;
        }
        let component_data = &self.data.borrow().components[initiator.0];
        println!(
            "[TRACE][{}][MEM] WR 0x{:04X} => {}",
            component_data.name,
            addr,
            TraceDisplay(&value)
        );
    }
    pub fn trace_instr(&self, initiator: TraceComponentId, addr: u16, opcode: u8, text: &str) {
        if !self.component_enabled_for_trace(initiator) {
            return;
        }
        let component_data = &self.data.borrow().components[initiator.0];
        println!(
            "[TRACE][{}][INST] 0x{:04X} 0x{:02X} {}",
            component_data.name, addr, opcode, text
        );
    }
    pub fn trace_seq_action(&self, initiator: TraceComponentId, action: &str) {
        if !self.component_enabled_for_trace(initiator) {
            return;
        }
        let component_data = &self.data.borrow().components[initiator.0];
        println!("[TRACE][{}][SEQ] {}", component_data.name, action);
    }
}

#[derive(Debug)]
pub struct TraceableReg<'t, T: Copy + Default + TraceableValue> {
    value: T,
    tracer: &'t Tracer,
    trace_id: TraceRegId,
}

impl<'t, T: Copy + Default + TraceableValue> TraceableReg<'t, T> {
    pub fn new(name: &str, tracer: &'t Tracer, component_id: TraceComponentId) -> Self {
        let trace_id = tracer.add_register(name, component_id);
        Self {
            value: T::default(),
            tracer,
            trace_id,
        }
    }

    pub fn get(&self) -> T {
        self.value
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
        self.tracer.trace_reg_write(self.trace_id, value);
    }

    pub fn update(&mut self, f: impl FnOnce(T) -> T) {
        let new_value = f(self.value);
        self.set(new_value);
    }
}

impl TraceableValue for u8 {
    fn fmt_trace(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:02X}", self)
    }
}
impl TraceableValue for u16 {
    fn fmt_trace(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:04X}", self)
    }
}
