use super::{ArchPSR, Cpu6502};

pub type OpFunc = fn(&mut Cpu6502, &mut u8) -> ();

pub fn nop(_cpu: &mut Cpu6502, _val: &mut u8) {
    // No operation
}

pub fn adc(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A += {reg}
    // @flags: NZC = ALU
    unimplemented!("Mnemonic ADC");
}
pub fn and(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A &= {reg}
    // @flags: NZ = ALU
    unimplemented!("Mnemonic AND");
}
pub fn asl(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} <<= 1
    // @flags: NZC = ALU
    unimplemented!("Mnemonic ASL");
}
pub fn bit(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A & {reg}
    // @flags: Z = ALU, N = M7, V = M6
    unimplemented!("Mnemonic BIT");
}
pub fn clc(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: P.C = 0
    unimplemented!("Mnemonic CLC");
}
pub fn cld(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: P.D = 0
    unimplemented!("Mnemonic CLD");
}
pub fn cli(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: P.I = 0
    unimplemented!("Mnemonic CLI");
}
pub fn clv(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: P.V = 0
    unimplemented!("Mnemonic CLV");
}
pub fn cmp(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A - {reg}
    // @flags: NZC = ALU
    unimplemented!("Mnemonic CMP");
}
pub fn cpx(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: X - {reg}
    // @flags: NZC = ALU
    unimplemented!("Mnemonic CPX");
}
pub fn cpy(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: Y - {reg}
    // @flags: NZC = ALU
    unimplemented!("Mnemonic CPY");
}
pub fn dec(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} -= 1
    // @flags: NZ = ALU
    unimplemented!("Mnemonic DEC");
}
pub fn dex(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: X -= 1
    // @flags: NZ = ALU
    unimplemented!("Mnemonic DEX");
}
pub fn dey(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: Y -= 1
    // @flags: NZ = ALU
    unimplemented!("Mnemonic DEY");
}
pub fn eor(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A ^= {reg}
    // @flags: NZ = ALU
    unimplemented!("Mnemonic EOR");
}
pub fn inc(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} += 1
    // @flags: NZ = ALU
    unimplemented!("Mnemonic INC");
}
pub fn inx(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: X += 1
    // @flags: NZ = ALU
    unimplemented!("Mnemonic INX");
}
pub fn iny(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: Y += 1
    // @flags: NZ = ALU
    unimplemented!("Mnemonic INY");
}
pub fn lda(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A = {reg}
    // @flags: NZ = ALU
    unimplemented!("Mnemonic LDA");
}
pub fn ldx(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: X = {reg}
    // @flags: NZ = ALU
    unimplemented!("Mnemonic LDX");
}
pub fn ldy(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: Y = {reg}
    // @flags: NZ = ALU
    unimplemented!("Mnemonic LDY");
}
pub fn lsr(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} >>= 1
    // @flags: N = 0, ZC = ALU
    unimplemented!("Mnemonic LSR");
}
pub fn ora(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A |= {reg}
    // @flags: NZ = ALU
    unimplemented!("Mnemonic ORA");
}
pub fn pha(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} = A
    unimplemented!("Mnemonic PHA");
}
pub fn php(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} = P
    unimplemented!("Mnemonic PHP");
}
pub fn pla(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A = {reg}
    // @flags: NZ = ALU
    unimplemented!("Mnemonic PLA");
}
pub fn plp(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} = P
    unimplemented!("Mnemonic PLP");
}
pub fn rol(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} ROL= 1
    // @flags: NZ = ALU, C = {reg}7
    unimplemented!("Mnemonic ROL");
}
pub fn ror(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} ROR= 1
    // @flags: NZ = ALU, C = {reg}0
    unimplemented!("Mnemonic ROR");
}
pub fn sbc(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A -= {reg}
    // @flags: NZCV = ALU
    unimplemented!("Mnemonic SBC");
}
pub fn sec(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: P.C = 1
    unimplemented!("Mnemonic SEC");
}
pub fn sed(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: P.D = 1
    unimplemented!("Mnemonic SED");
}
pub fn sei(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: P.I = 1
    unimplemented!("Mnemonic SEI");
}
pub fn sta(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} = A
    unimplemented!("Mnemonic STA");
}
pub fn stx(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} = X
    unimplemented!("Mnemonic STX");
}
pub fn sty(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: {reg} = Y
    unimplemented!("Mnemonic STY");
}
pub fn tax(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: X = A
    // @flags: NZ = ALU
    unimplemented!("Mnemonic TAX");
}
pub fn tay(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: Y = A
    // @flags: NZ = ALU
    unimplemented!("Mnemonic TAY");
}
pub fn tsx(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: X = S
    // @flags: NZ = ALU
    unimplemented!("Mnemonic TSX");
}
pub fn txa(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A = X
    // @flags: NZ = ALU
    unimplemented!("Mnemonic TXA");
}
pub fn txs(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: S = X
    unimplemented!("Mnemonic TXS");
}
pub fn tya(cpu: &mut Cpu6502, val: &mut u8) {
    // @pseudocode: A = Y
    // @flags: NZ = ALU
    unimplemented!("Mnemonic TYA");
}
