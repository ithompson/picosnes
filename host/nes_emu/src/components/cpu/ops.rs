use super::{ArchPSR, Cpu6502};

pub type OpFunc = fn(&mut Cpu6502, &mut u8) -> ();

pub fn nop(_cpu: &mut Cpu6502, _val: &mut u8) {
    // No operation
}

pub fn adc(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A += {reg}
    // @flags: NZC = ALU
    todo!("Mnemonic ADC");
}
pub fn and(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A &= {reg}
    // @flags: NZ = ALU
    todo!("Mnemonic AND");
}
pub fn asl(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} <<= 1
    // @flags: NZC = ALU
    todo!("Mnemonic ASL");
}
pub fn bit(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A & {reg}
    // @flags: Z = ALU, N = M7, V = M6
    todo!("Mnemonic BIT");
}
pub fn clc(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: P.C = 0
    todo!("Mnemonic CLC");
}
pub fn cld(cpu: &mut Cpu6502, _val: &mut u8) {
    // @pseudocode: P.D = 0
    cpu.regs.p.update(|p| p.with_d(false));
}
pub fn cli(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: P.I = 0
    todo!("Mnemonic CLI");
}
pub fn clv(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: P.V = 0
    todo!("Mnemonic CLV");
}
pub fn cmp(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A - {reg}
    // @flags: NZC = ALU
    todo!("Mnemonic CMP");
}
pub fn cpx(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: X - {reg}
    // @flags: NZC = ALU
    todo!("Mnemonic CPX");
}
pub fn cpy(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: Y - {reg}
    // @flags: NZC = ALU
    todo!("Mnemonic CPY");
}
pub fn dec(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} -= 1
    // @flags: NZ = ALU
    todo!("Mnemonic DEC");
}
pub fn dex(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: X -= 1
    // @flags: NZ = ALU
    todo!("Mnemonic DEX");
}
pub fn dey(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: Y -= 1
    // @flags: NZ = ALU
    todo!("Mnemonic DEY");
}
pub fn eor(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A ^= {reg}
    // @flags: NZ = ALU
    todo!("Mnemonic EOR");
}
pub fn inc(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} += 1
    // @flags: NZ = ALU
    *val = val.wrapping_add(1);
    cpu.regs.p.update(|p| p.with_nz_from_value(*val));
}
pub fn inx(cpu: &mut Cpu6502, _val: &mut u8) {
    // @pseudocode: X += 1
    // @flags: NZ = ALU
    cpu.regs.x.update(|x| x.wrapping_add(1));
    cpu.regs
        .p
        .update(|p| p.with_nz_from_value(cpu.regs.x.get()));
}
pub fn iny(cpu: &mut Cpu6502, _val: &mut u8) {
    // @pseudocode: Y += 1
    // @flags: NZ = ALU
    cpu.regs.y.update(|y| y.wrapping_add(1));
    cpu.regs
        .p
        .update(|p| p.with_nz_from_value(cpu.regs.y.get()));
}
pub fn lda(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A = {reg}
    // @flags: NZ = ALU
    cpu.regs.a.set(*val);
    cpu.regs.p.update(|p| p.with_nz_from_value(*val));
}
pub fn ldx(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: X = {reg}
    // @flags: NZ = ALU
    cpu.regs.x.set(*val);
    cpu.regs.p.update(|p| p.with_nz_from_value(*val));
}
pub fn ldy(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: Y = {reg}
    // @flags: NZ = ALU
    cpu.regs.y.set(*val);
    cpu.regs.p.update(|p| p.with_nz_from_value(*val));
}
pub fn lsr(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} >>= 1
    // @flags: N = 0, ZC = ALU
    todo!("Mnemonic LSR");
}
pub fn ora(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A |= {reg}
    // @flags: NZ = ALU
    todo!("Mnemonic ORA");
}
pub fn pha(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} = A
    todo!("Mnemonic PHA");
}
pub fn php(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} = P
    todo!("Mnemonic PHP");
}
pub fn pla(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A = {reg}
    // @flags: NZ = ALU
    todo!("Mnemonic PLA");
}
pub fn plp(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} = P
    todo!("Mnemonic PLP");
}
pub fn rol(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} ROL= 1
    // @flags: NZ = ALU, C = {reg}7
    todo!("Mnemonic ROL");
}
pub fn ror(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} ROR= 1
    // @flags: NZ = ALU, C = {reg}0
    todo!("Mnemonic ROR");
}
pub fn sbc(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A -= {reg}
    // @flags: NZCV = ALU
    todo!("Mnemonic SBC");
}
pub fn sec(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: P.C = 1
    todo!("Mnemonic SEC");
}
pub fn sed(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: P.D = 1
    todo!("Mnemonic SED");
}
pub fn sei(cpu: &mut Cpu6502, _val: &mut u8) {
    // @pseudocode: P.I = 1
    cpu.regs.p.update(|p| p.with_i(true));
}
pub fn sta(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} = A
    *val = cpu.regs.a.get();
}
pub fn stx(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} = X
    *val = cpu.regs.x.get();
}
pub fn sty(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} = Y
    *val = cpu.regs.y.get();
}
pub fn tax(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: X = A
    // @flags: NZ = ALU
    todo!("Mnemonic TAX");
}
pub fn tay(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: Y = A
    // @flags: NZ = ALU
    todo!("Mnemonic TAY");
}
pub fn tsx(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: X = S
    // @flags: NZ = ALU
    todo!("Mnemonic TSX");
}
pub fn txa(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A = X
    // @flags: NZ = ALU
    todo!("Mnemonic TXA");
}
pub fn txs(cpu: &mut Cpu6502, _val: &mut u8) {
    // @pseudocode: S = X
    cpu.regs.s.set(cpu.regs.x.get());
}
pub fn tya(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A = Y
    // @flags: NZ = ALU
    todo!("Mnemonic TYA");
}
