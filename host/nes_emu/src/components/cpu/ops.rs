use super::Cpu6502;

pub type OpFunc = fn(&mut Cpu6502, &mut u8) -> ();

pub fn nop(_cpu: &mut Cpu6502, _val: &mut u8) {}

pub fn inc(cpu: &mut Cpu6502, val: &mut u8) {
    *val = val.wrapping_add(1);
    let p = cpu.regs.p.get();
    cpu.regs.p.set(p.with_nz_from_value(*val));
}

pub fn asl(cpu: &mut Cpu6502, val: &mut u8) {
    let carry = (*val & 0x80) != 0;
    *val <<= 1;
    let p = cpu.regs.p.get();
    cpu.regs.p.set(p.with_c(carry).with_nz_from_value(*val));
}
