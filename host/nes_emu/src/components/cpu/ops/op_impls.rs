use super::{ArchPSR, ArchRegs};

pub fn nop(_regs: &mut ArchRegs, _val: &mut u8) {
    // @pseudocode: nop
}

fn alu_addsub(regs: &mut ArchRegs, val: u8) {
    let a = regs.a.get();
    let carry_in = if regs.p.get().c { 1 } else { 0 };
    let wide_result = (a as u16).wrapping_add(val as u16).wrapping_add(carry_in);
    let result = (wide_result & 0xFF) as u8;
    let carry_out = wide_result > 0xFF;
    let overflow = (result ^ a) & (result ^ val) >= 0x80;

    regs.a.set(result);
    regs.p
        .update(|p| p.with_nzcv_from_value(result, carry_out, overflow));
}

pub fn adc(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: A += {reg} + C
    // @flags: NZCV = ALU
    alu_addsub(regs, *val);
}
pub fn and(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: A &= {reg}
    // @flags: NZ = ALU
    regs.a.update(|a| a & *val);
    regs.p.update(|p| p.with_nz_from_value(regs.a.get()));
}
pub fn asl(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} <<= 1
    // @flags: NZC = ALU
    let carry = *val & 0x80;
    *val <<= 1;
    regs.p.update(|p| p.with_nzc_from_value(*val, carry != 0));
}
pub fn bit(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: A & {reg}
    // @flags: Z = ALU, N = M7, V = M6
    let m7 = *val & 0x80 != 0;
    let m6 = *val & 0x40 != 0;
    let z = regs.a.get() & *val == 0;
    regs.p.update(|p| p.with_nzv(m7, z, m6));
}
pub fn clc(regs: &mut ArchRegs, _val: &mut u8) {
    // @pseudocode: P.C = 0
    regs.p.update(|p| p.with_c(false));
}
pub fn cld(regs: &mut ArchRegs, _val: &mut u8) {
    // @pseudocode: P.D = 0
    regs.p.update(|p| p.with_d(false));
}
pub fn cli(regs: &mut ArchRegs, _val: &mut u8) {
    // @pseudocode: P.I = 0
    regs.p.update(|p| p.with_i(false));
}
pub fn clv(regs: &mut ArchRegs, _val: &mut u8) {
    // @pseudocode: P.V = 0
    regs.p.update(|p| p.with_v(false));
}
pub fn cmp(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: A - {reg}
    // @flags: NZC = ALU
    let (result, carry) = regs.a.get().overflowing_sub(*val);
    regs.p.update(|p| p.with_nzc_from_value(result, !carry));
}
pub fn cpx(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: X - {reg}
    // @flags: NZC = ALU
    let (result, carry) = regs.x.get().overflowing_sub(*val);
    regs.p.update(|p| p.with_nzc_from_value(result, !carry));
}
pub fn cpy(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: Y - {reg}
    // @flags: NZC = ALU
    let (result, carry) = regs.y.get().overflowing_sub(*val);
    regs.p.update(|p| p.with_nzc_from_value(result, !carry));
}
pub fn dec(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} -= 1
    // @flags: NZ = ALU
    *val = val.wrapping_sub(1);
    regs.p.update(|p| p.with_nz_from_value(*val));
}
pub fn dex(regs: &mut ArchRegs, _val: &mut u8) {
    // @pseudocode: X -= 1
    // @flags: NZ = ALU
    regs.x.update(|x| x.wrapping_sub(1));
    regs.p.update(|p| p.with_nz_from_value(regs.x.get()));
}
pub fn dey(regs: &mut ArchRegs, _val: &mut u8) {
    // @pseudocode: Y -= 1
    // @flags: NZ = ALU
    regs.y.update(|y| y.wrapping_sub(1));
    regs.p.update(|p| p.with_nz_from_value(regs.y.get()));
}
pub fn eor(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: A ^= {reg}
    // @flags: NZ = ALU
    regs.a.update(|a| a ^ *val);
    regs.p.update(|p| p.with_nz_from_value(regs.a.get()));
}
pub fn inc(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} += 1
    // @flags: NZ = ALU
    *val = val.wrapping_add(1);
    regs.p.update(|p| p.with_nz_from_value(*val));
}
pub fn inx(regs: &mut ArchRegs, _val: &mut u8) {
    // @pseudocode: X += 1
    // @flags: NZ = ALU
    regs.x.update(|x| x.wrapping_add(1));
    regs.p.update(|p| p.with_nz_from_value(regs.x.get()));
}
pub fn iny(regs: &mut ArchRegs, _val: &mut u8) {
    // @pseudocode: Y += 1
    // @flags: NZ = ALU
    regs.y.update(|y| y.wrapping_add(1));
    regs.p.update(|p| p.with_nz_from_value(regs.y.get()));
}
pub fn lda(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: A = {reg}
    // @flags: NZ = ALU
    regs.a.set(*val);
    regs.p.update(|p| p.with_nz_from_value(*val));
}
pub fn ldx(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: X = {reg}
    // @flags: NZ = ALU
    regs.x.set(*val);
    regs.p.update(|p| p.with_nz_from_value(*val));
}
pub fn ldy(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: Y = {reg}
    // @flags: NZ = ALU
    regs.y.set(*val);
    regs.p.update(|p| p.with_nz_from_value(*val));
}
pub fn lsr(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} >>= 1
    // @flags: N = 0, ZC = ALU
    let carry = *val & 1;
    *val >>= 1;
    regs.p.update(|p| p.with_nzc_from_value(*val, carry != 0));
}
pub fn ora(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: A |= {reg}
    // @flags: NZ = ALU
    regs.a.update(|a| a | *val);
    regs.p.update(|p| p.with_nz_from_value(regs.a.get()));
}
pub fn pha(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} = A
    *val = regs.a.get();
}
pub fn php(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} = P
    *val = regs.p.get().as_stk_u8(false);
}
pub fn pla(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: A = {reg}
    // @flags: NZ = ALU
    regs.a.set(*val);
    regs.p.update(|p| p.with_nz_from_value(*val));
}
pub fn plp(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} = P
    regs.p.set(ArchPSR::from_stk_u8(*val));
}
pub fn rol(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} ROL= 1
    // @flags: NZ = ALU, C = {reg}7
    let carry = *val & 0x80;
    *val = (*val << 1) | (if regs.p.c { 1 } else { 0 });
    regs.p.update(|p| p.with_nzc_from_value(*val, carry != 0));
}
pub fn ror(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} ROR= 1
    // @flags: NZ = ALU, C = {reg}0
    let carry = *val & 1;
    *val = (if regs.p.c { 0x80 } else { 0x00 }) | (*val >> 1);
    regs.p.update(|p| p.with_nzc_from_value(*val, carry != 0));
}
pub fn sbc(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: A += ~{reg} + C
    // @flags: NZCV = ALU
    alu_addsub(regs, !*val);
}
pub fn sec(regs: &mut ArchRegs, _val: &mut u8) {
    // @pseudocode: P.C = 1
    regs.p.update(|p| p.with_c(true));
}
pub fn sed(regs: &mut ArchRegs, _val: &mut u8) {
    // @pseudocode: P.D = 1
    regs.p.update(|p| p.with_d(true));
}
pub fn sei(regs: &mut ArchRegs, _val: &mut u8) {
    // @pseudocode: P.I = 1
    regs.p.update(|p| p.with_i(true));
}
pub fn sta(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} = A
    *val = regs.a.get();
}
pub fn stx(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} = X
    *val = regs.x.get();
}
pub fn sty(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} = Y
    *val = regs.y.get();
}
pub fn tax(regs: &mut ArchRegs, _val: &mut u8) {
    // @pseudocode: X = A
    // @flags: NZ = ALU
    regs.x.set(regs.a.get());
    regs.p.update(|p| p.with_nz_from_value(regs.x.get()));
}
pub fn tay(regs: &mut ArchRegs, _val: &mut u8) {
    // @pseudocode: Y = A
    // @flags: NZ = ALU
    regs.y.set(regs.a.get());
    regs.p.update(|p| p.with_nz_from_value(regs.y.get()));
}
pub fn tsx(regs: &mut ArchRegs, _val: &mut u8) {
    // @pseudocode: X = S
    // @flags: NZ = ALU
    regs.x.set(regs.s.get());
    regs.p.update(|p| p.with_nz_from_value(regs.x.get()));
}
pub fn txa(regs: &mut ArchRegs, _val: &mut u8) {
    // @pseudocode: A = X
    // @flags: NZ = ALU
    regs.a.set(regs.x.get());
    regs.p.update(|p| p.with_nz_from_value(regs.a.get()));
}
pub fn txs(regs: &mut ArchRegs, _val: &mut u8) {
    // @pseudocode: S = X
    regs.s.set(regs.x.get());
}
pub fn tya(regs: &mut ArchRegs, _val: &mut u8) {
    // @pseudocode: A = Y
    // @flags: NZ = ALU
    regs.a.set(regs.y.get());
    regs.p.update(|p| p.with_nz_from_value(regs.a.get()));
}

pub fn bcc(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} = P.C == 0
    *val = if !regs.p.get().c { 1 } else { 0 };
}

pub fn bcs(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} = P.C == 1
    *val = if regs.p.get().c { 1 } else { 0 };
}

pub fn beq(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} = P.Z == 1
    *val = if regs.p.get().z { 1 } else { 0 };
}

pub fn bmi(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} = P.N == 1
    *val = if regs.p.get().n { 1 } else { 0 };
}

pub fn bne(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} = P.Z == 0
    *val = if !regs.p.get().z { 1 } else { 0 };
}

pub fn bpl(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} = P.N == 0
    *val = if !regs.p.get().n { 1 } else { 0 };
}

pub fn bvc(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} = P.V == 0
    *val = if !regs.p.get().v { 1 } else { 0 };
}

pub fn bvs(regs: &mut ArchRegs, val: &mut u8) {
    // @pseudocode: {reg} = P.V == 1
    *val = if regs.p.get().v { 1 } else { 0 };
}

#[cfg(test)]
mod tests {
    // The handling of "logic" definitions in op tests generates
    // spurious redundant semicolon warnings. Fixing the macro to
    // avoid them breaks rust-analyzer's macro parsing, so just
    // waive the warning in this test code.
    #![allow(redundant_semicolons)]

    use super::super::test_helpers::define_op_test;
    use super::*;

    // Access ops
    define_op_test!(test_lda(regs, val) { op: lda, update_regs: { a: val, p: regs.p.with_nz_from_value(val) } });
    define_op_test!(test_sta(regs) { op: sta, expected_val: *regs.a });
    define_op_test!(test_ldx(regs, val) { op: ldx, update_regs: { x: val, p: regs.p.with_nz_from_value(val) } });
    define_op_test!(test_stx(regs) { op: stx, expected_val: *regs.x });
    define_op_test!(test_ldy(regs, val) { op: ldy, update_regs: { y: val, p: regs.p.with_nz_from_value(val) } });
    define_op_test!(test_sty(regs) { op: sty, expected_val: *regs.y });

    // Transfer ops
    define_op_test!(test_tax(regs) { op: tax, update_regs: { x: *regs.a, p: regs.p.with_nz_from_value(*regs.a) } });
    define_op_test!(test_txa(regs) { op: txa, update_regs: { a: *regs.x, p: regs.p.with_nz_from_value(*regs.x) } });
    define_op_test!(test_tay(regs) { op: tay, update_regs: { y: *regs.a, p: regs.p.with_nz_from_value(*regs.a) } });
    define_op_test!(test_tya(regs) { op: tya, update_regs: { a: *regs.y, p: regs.p.with_nz_from_value(*regs.y) } });

    // Arithmetic ops
    define_op_test!(test_adc(regs, val, 10000) {
        op: adc,
        logic: {
            let wide_result = (*regs.a as u16).wrapping_add(val as u16)
                .wrapping_add(regs.p.c as u16);
            let result = wide_result as u8;
        },
        update_regs: {
            a: result,
            p: regs.p.with_nzcv_from_value(result, wide_result > 0xFF, (((result ^ *regs.a) & (result ^ val)) & 0x80) != 0)
        }
    });
    define_op_test!(test_sbc(regs, val, 10000) {
        op: sbc,
        logic: {
            let wide_result = (*regs.a as u16).wrapping_sub(val as u16)
                .wrapping_sub(!regs.p.c as u16);
            let result = wide_result as u8;
        },
        update_regs: {
            a: result,
            p: regs.p.with_nzcv_from_value(result, wide_result <= 0xFF, (((result ^ *regs.a) & (result ^ !val)) & 0x80) != 0)
        }
    });
    define_op_test!(test_inc(regs, val) {
        op: inc,
        update_regs: { p: regs.p.with_nz_from_value(val.wrapping_add(1)) },
        expected_val: val.wrapping_add(1)
    });
    define_op_test!(test_dec(regs, val) {
        op: dec,
        update_regs: { p: regs.p.with_nz_from_value(val.wrapping_sub(1)) },
        expected_val: val.wrapping_sub(1)
    });
    define_op_test!(test_inx(regs) {
        op: inx,
        update_regs: {
            x: regs.x.wrapping_add(1),
            p: regs.p.with_nz_from_value(regs.x.wrapping_add(1))
        }
    });
    define_op_test!(test_dex(regs) {
        op: dex,
        update_regs: {
            x: regs.x.wrapping_sub(1),
            p: regs.p.with_nz_from_value(regs.x.wrapping_sub(1))
        }
    });
    define_op_test!(test_iny(regs) {
        op: iny,
        update_regs: {
            y: regs.y.wrapping_add(1),
            p: regs.p.with_nz_from_value(regs.y.wrapping_add(1))
        }
    });
    define_op_test!(test_dey(regs) {
        op: dey,
        update_regs: {
            y: regs.y.wrapping_sub(1),
            p: regs.p.with_nz_from_value(regs.y.wrapping_sub(1))
        }
    });

    // Shift ops
    define_op_test!(test_asl(regs, val) {
        op: asl,
        logic: { let result = val << 1; },
        update_regs: { p: regs.p.with_nzc_from_value(result, val & 0x80 != 0) },
        expected_val: result
    });
    define_op_test!(test_lsr(regs, val) {
        op: lsr,
        logic: { let result = val >> 1; },
        update_regs: { p: regs.p.with_nzc(false, result == 0, val & 1 != 0) },
        expected_val: result
    });
    define_op_test!(test_rol(regs, val) {
        op: rol,
        logic: { let result = (val << 1) | (regs.p.c as u8); },
        update_regs: { p: regs.p.with_nzc_from_value(result, val & 0x80 != 0) },
        expected_val: result
    });
    define_op_test!(test_ror(regs, val) {
        op: ror,
        logic: { let result = (if regs.p.c {0x80} else {0x00}) | (val >> 1); },
        update_regs: { p: regs.p.with_nzc_from_value(result, val & 0x01 != 0) },
        expected_val: result
    });

    // Bitwise ops
    define_op_test!(test_and(regs, val) {
        op: and,
        logic: { let result = *regs.a & val; },
        update_regs: { a: result, p: regs.p.with_nz_from_value(result) }
    });
    define_op_test!(test_ora(regs, val) {
        op: ora,
        logic: { let result = *regs.a | val; },
        update_regs: { a: result, p: regs.p.with_nz_from_value(result) }
    });
    define_op_test!(test_eor(regs, val) {
        op: eor,
        logic: { let result = *regs.a ^ val; },
        update_regs: { a: result, p: regs.p.with_nz_from_value(result) }
    });
    define_op_test!(test_bit(regs, val) {
        op: bit,
        logic: {
            let m7 = val & 0x80 != 0;
            let m6 = val & 0x40 != 0;
            let z = *regs.a & val == 0;
        },
        update_regs: { p: regs.p.with_nzv(m7, z, m6) }
    });

    // Compare ops
    define_op_test!(test_cmp(regs, val) {
        op: cmp,
        update_regs: {
            p: regs.p.with_nzc(regs.a.wrapping_sub(val) & 0x80 != 0, *regs.a == val, *regs.a >= val)
        }
    });
    define_op_test!(test_cpx(regs, val) {
        op: cpx,
        update_regs: {
            p: regs.p.with_nzc(regs.x.wrapping_sub(val) & 0x80 != 0, *regs.x == val, *regs.x >= val)
        }
    });
    define_op_test!(test_cpy(regs, val) {
        op: cpy,
        update_regs: {
            p: regs.p.with_nzc(regs.y.wrapping_sub(val) & 0x80 != 0, *regs.y == val, *regs.y >= val)
        }
    });

    // Branch ops
    define_op_test!(test_bcc(regs) { op: bcc, expected_val: !regs.p.c as u8});
    define_op_test!(test_bcs(regs) { op: bcs, expected_val: regs.p.c as u8});
    define_op_test!(test_beq(regs) { op: beq, expected_val: regs.p.z as u8});
    define_op_test!(test_bne(regs) { op: bne, expected_val: !regs.p.z as u8});
    define_op_test!(test_bpl(regs) { op: bpl, expected_val: !regs.p.n as u8});
    define_op_test!(test_bmi(regs) { op: bmi, expected_val: regs.p.n as u8});
    define_op_test!(test_bvc(regs) { op: bvc, expected_val: !regs.p.v as u8});
    define_op_test!(test_bvs(regs) { op: bvs, expected_val: regs.p.v as u8});

    // Stack ops
    define_op_test!(test_pha(regs) { op: pha, expected_val: *regs.a });
    define_op_test!(test_php(regs) { op: php, expected_val: regs.p.as_stk_u8(false) });
    define_op_test!(test_pla(regs, val) { op: pla, update_regs: { a: val, p: regs.p.with_nz_from_value(val) } });
    define_op_test!(test_plp(regs, val) { op: plp, update_regs: { p: ArchPSR::from_stk_u8(val) } });
    define_op_test!(test_txs(regs) { op: txs, update_regs: { s: *regs.x } });
    define_op_test!(test_tsx(regs) { op: tsx, update_regs: { x: *regs.s, p: regs.p.with_nz_from_value(*regs.s) } });

    // Flags ops
    define_op_test!(test_clc(regs) { op: clc, update_regs: { p: regs.p.with_c(false) } });
    define_op_test!(test_sec(regs) { op: sec, update_regs: { p: regs.p.with_c(true) } });
    define_op_test!(test_cli(regs) { op: cli, update_regs: { p: regs.p.with_i(false) } });
    define_op_test!(test_sei(regs) { op: sei, update_regs: { p: regs.p.with_i(true) } });
    define_op_test!(test_cld(regs) { op: cld, update_regs: { p: regs.p.with_d(false) } });
    define_op_test!(test_sed(regs) { op: sed, update_regs: { p: regs.p.with_d(true) } });
    define_op_test!(test_clv(regs) { op: clv, update_regs: { p: regs.p.with_v(false) } });

    // Other ops
    define_op_test!(test_nop() { op: nop });
}
