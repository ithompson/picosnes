use super::{ArchPSR, Cpu6502};

pub type OpFunc = fn(&mut Cpu6502, &mut u8) -> ();

macro_rules! unimplemented_op {
    ($name:ident) => {
        pub fn $name(_cpu: &mut Cpu6502, _val: &mut u8) {
            unimplemented!("Unimplemented CPU operation: {}", stringify!($name));
        }
    };
}

pub fn nop(_cpu: &mut Cpu6502, _val: &mut u8) {}

pub fn inc(cpu: &mut Cpu6502, val: &mut u8) {
    *val = val.wrapping_add(1);
    cpu.regs.p.update(|p| p.with_nz_from_value(*val));
}

unimplemented_op!(inx);
unimplemented_op!(iny);
unimplemented_op!(dec);
unimplemented_op!(dex);
unimplemented_op!(dey);

unimplemented_op!(adc);
unimplemented_op!(sbc);
unimplemented_op!(and);
unimplemented_op!(ora);
unimplemented_op!(eor);

pub fn asl(cpu: &mut Cpu6502, val: &mut u8) {
    let carry = (*val & 0x80) != 0;
    *val <<= 1;
    cpu.regs.p.update(|p| p.with_nzc_from_value(*val, carry));
}
unimplemented_op!(lsr);
unimplemented_op!(rol);
unimplemented_op!(ror);

unimplemented_op!(bit);
unimplemented_op!(cmp);
unimplemented_op!(cpx);
unimplemented_op!(cpy);

pub fn pha(cpu: &mut Cpu6502, val: &mut u8) {
    *val = cpu.regs.a.get();
}
pub fn php(cpu: &mut Cpu6502, val: &mut u8) {
    *val = cpu.regs.p.get().as_stk_u8(false);
}
pub fn pla(cpu: &mut Cpu6502, val: &mut u8) {
    cpu.regs.a.set(*val);
    cpu.regs.p.update(|p| p.with_nz_from_value(*val));
}
pub fn plp(cpu: &mut Cpu6502, val: &mut u8) {
    cpu.regs.p.set(ArchPSR::from_stk_u8(*val));
}

pub fn sec(cpu: &mut Cpu6502, _val: &mut u8) {
    cpu.regs.p.update(|p| p.with_c(true));
}
pub fn sed(cpu: &mut Cpu6502, _val: &mut u8) {
    cpu.regs.p.update(|p| p.with_d(true));
}
pub fn sei(cpu: &mut Cpu6502, _val: &mut u8) {
    cpu.regs.p.update(|p| p.with_i(true));
}
pub fn clc(cpu: &mut Cpu6502, _val: &mut u8) {
    cpu.regs.p.update(|p| p.with_c(false));
}
pub fn cld(cpu: &mut Cpu6502, _val: &mut u8) {
    cpu.regs.p.update(|p| p.with_d(false));
}
pub fn cli(cpu: &mut Cpu6502, _val: &mut u8) {
    cpu.regs.p.update(|p| p.with_i(false));
}
pub fn clv(cpu: &mut Cpu6502, _val: &mut u8) {
    cpu.regs.p.update(|p| p.with_v(false));
}

pub fn lda(cpu: &mut Cpu6502, val: &mut u8) {
    cpu.regs.a.set(*val);
    cpu.regs.p.update(|p| p.with_nz_from_value(*val));
}
pub fn ldx(cpu: &mut Cpu6502, val: &mut u8) {
    cpu.regs.x.set(*val);
    cpu.regs.p.update(|p| p.with_nz_from_value(*val));
}
pub fn ldy(cpu: &mut Cpu6502, val: &mut u8) {
    cpu.regs.y.set(*val);
    cpu.regs.p.update(|p| p.with_nz_from_value(*val));
}

pub fn sta(cpu: &mut Cpu6502, val: &mut u8) {
    *val = cpu.regs.a.get();
}
pub fn stx(cpu: &mut Cpu6502, val: &mut u8) {
    *val = cpu.regs.x.get();
}
pub fn sty(cpu: &mut Cpu6502, val: &mut u8) {
    *val = cpu.regs.y.get();
}

pub fn txa(cpu: &mut Cpu6502, _val: &mut u8) {
    let a = cpu.regs.x.get();
    cpu.regs.a.set(a);
    cpu.regs.p.update(|p| p.with_nz_from_value(a));
}
pub fn tya(cpu: &mut Cpu6502, _val: &mut u8) {
    let a = cpu.regs.y.get();
    cpu.regs.a.set(a);
    cpu.regs.p.update(|p| p.with_nz_from_value(a));
}
pub fn txs(cpu: &mut Cpu6502, _val: &mut u8) {
    let a = cpu.regs.x.get();
    cpu.regs.s.set(a);
}
pub fn tay(cpu: &mut Cpu6502, _val: &mut u8) {
    let a = cpu.regs.a.get();
    cpu.regs.y.set(a);
    cpu.regs.p.update(|p| p.with_nz_from_value(a));
}
pub fn tax(cpu: &mut Cpu6502, _val: &mut u8) {
    let a = cpu.regs.a.get();
    cpu.regs.x.set(a);
    cpu.regs.p.update(|p| p.with_nz_from_value(a));
}
pub fn tsx(cpu: &mut Cpu6502, _val: &mut u8) {
    let a = cpu.regs.s.get();
    cpu.regs.x.set(a);
    cpu.regs.p.update(|p| p.with_nz_from_value(a));
}
