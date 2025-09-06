use super::*;

action_defs! {
    NOP => || {
        // @pseudocode:
    },
    DISPATCH => |cpu| {
        // @pseudocode: dispatch(rd_val)
        cpu.dispatch(cpu.internal.rd_val)
    },
    SET_P => |cpu| {
        // @pseudocode: P = rd_val
        todo!("Action SET_P");
    },
    DEC_S => |cpu| {
        // @pseudocode: S -= 1
        cpu.regs.s.update(|s| s.wrapping_sub(1));
    },
    SET_PC_LO => |cpu| {
        // @pseudocode: PC.lo = rd_val
        todo!("Action SET_PC_LO");
    },
    SET_PC_HI => |cpu| {
        // @pseudocode: PC.hi = rd_val
        cpu.regs.pc.update(|pc| (pc & 0x00FF) | ((cpu.internal.rd_val as u16) << 8));
    },
    SET_PC_FULL => |cpu| {
        // @pseudocode: PC.hi = rd_val, PC.lo = tmp.lo
        cpu.regs.pc.set((cpu.internal.rd_val as u16) << 8 | cpu.internal.tmp_lo as u16);
    },
    SET_PC_LO_INC_TMP => |cpu| {
        // @pseudocode: PC.lo = rd_val, tmp.lo += 1
        cpu.regs.pc.update(|pc| (pc & 0xFF00) | (cpu.internal.rd_val as u16));
        cpu.internal.tmp_lo = cpu.internal.tmp_lo.wrapping_add(1);
    },
    ADVANCE_PC_BY_DAT_STOP_IF_NO_CARRY => |cpu| {
        // @pseudocode: PC.lo += dat, done if no carry
        todo!("Action ADVANCE_PC_BY_DAT_STOP_IF_NO_CARRY");
    },
    INC_PC_HI => |cpu| {
        // @pseudocode: PC.hi += 1
        todo!("Action INC_PC_HI");
    },
    INVOKE_OP => |cpu| {
        // @pseudocode: op()
        let mut val = 0;
        (cpu.op_func)(cpu, &mut val);
    },
    INVOKE_OP_A => |cpu| {
        // @pseudocode: op(A)
        todo!("Action INVOKE_OP_A");
    },
    INVOKE_OP_DAT => |cpu| {
        // @pseudocode: op(dat)
        let mut val = cpu.internal.dat;
        (cpu.op_func)(cpu, &mut val);
        cpu.internal.dat = val;
    },
    INVOKE_OP_RD_VAL => |cpu| {
        // @pseudocode: op(rd_val)
        let mut val = cpu.internal.rd_val;
        (cpu.op_func)(cpu, &mut val);
    },
    SET_TMP_LO => |cpu| {
        // @pseudocode: tmp.lo = rd_val
        cpu.internal.tmp_lo = cpu.internal.rd_val;
    },
    SET_TMP_HI => |cpu| {
        // @pseudocode: tmp.hi = rd_val
        todo!("Action SET_TMP_HI");
    },
    SET_TMP_FULL => |cpu| {
        // @pseudocode: tmp.hi = rd_val, tmp.lo = dat
        todo!("Action SET_TMP_FULL");
    },
    SET_TMP_ZP => |cpu| {
        // @pseudocode: tmp.lo = rd_val, tmp.hi = 0
        cpu.internal.tmp_lo = cpu.internal.rd_val;
        cpu.internal.tmp_hi = 0;
    },
    SET_TMP_HI_INVOKE_OP_DAT => |cpu| {
        // @pseudocode: tmp.hi = rd_val, op(dat)
        cpu.internal.tmp_hi = cpu.internal.rd_val;
        let mut val = cpu.internal.dat;
        (cpu.op_func)(cpu, &mut val);
        cpu.internal.dat = val;
    },
    SET_TMP_ZP_INVOKE_OP_DAT => |cpu| {
        // @pseudocode: tmp.lo = rd_val, tmp.hi = 0, op(dat)
        cpu.internal.tmp_lo = cpu.internal.rd_val;
        cpu.internal.tmp_hi = 0;
        let mut val = cpu.internal.dat;
        (cpu.op_func)(cpu, &mut val);
        cpu.internal.dat = val;
    },
    SET_TMP_FULL_INVOKE_OP_DAT => |cpu| {
        // @pseudocode: tmp.hi = rd_val, tmp.lo = dat, op(dat)
        cpu.internal.tmp_hi = cpu.internal.rd_val;
        cpu.internal.tmp_lo = cpu.internal.dat;
        let mut val = cpu.internal.dat;
        (cpu.op_func)(cpu, &mut val);
        cpu.internal.dat = val;
    },
    SET_TMP_HI_INC_BY_X_RECORD_CARRY => |cpu| {
        // @pseudocode: tmp.hi = rd_val, tmp.lo += X, record carry
        todo!("Action SET_TMP_HI_INC_BY_X_RECORD_CARRY");
    },
    SET_TMP_HI_INC_BY_X_SKIP_IF_NO_CARRY => |cpu| {
        // @pseudocode: tmp.hi = rd_val, tmp.lo += X, skip next if no carry
        todo!("Action SET_TMP_HI_INC_BY_X_SKIP_IF_NO_CARRY");
    },
    SET_TMP_HI_INC_BY_Y_RECORD_CARRY => |cpu| {
        // @pseudocode: tmp.hi = rd_val, tmp.lo += Y, record carry
        todo!("Action SET_TMP_HI_INC_BY_Y_RECORD_CARRY");
    },
    SET_TMP_HI_INC_BY_Y_SKIP_IF_NO_CARRY => |cpu| {
        // @pseudocode: tmp.hi = rd_val, tmp.lo += Y, skip next if no carry
        todo!("Action SET_TMP_HI_INC_BY_Y_SKIP_IF_NO_CARRY");
    },
    SET_TMP_FULL_INC_BY_Y_RECORD_CARRY => |cpu| {
        // @pseudocode: tmp.hi = rd_val, tmp.lo = dat + Y, record carry
        todo!("Action SET_TMP_FULL_INC_BY_Y_RECORD_CARRY");
    },
    SET_TMP_FULL_INC_BY_Y_SKIP_IF_NO_CARRY => |cpu| {
        // @pseudocode: tmp.hi = rd_val, tmp.lo = dat + Y, skip next if no carry
        todo!("Action SET_TMP_FULL_INC_BY_Y_SKIP_IF_NO_CARRY");
    },
    INC_TMP_HI => |cpu| {
        // @pseudocode: tmp.hi += 1
        todo!("Action INC_TMP_HI");
    },
    CARRY_INTO_TMP_HI => |cpu| {
        // @pseudocode: tmp.hi += carry
        todo!("Action CARRY_INTO_TMP_HI");
    },
    CARRY_INTO_TMP_HI_INVOKE_OP_DAT => |cpu| {
        // @pseudocode: tmp.hi += carry, op(dat)
        todo!("Action CARRY_INTO_TMP_HI_INVOKE_OP_DAT");
    },
    INC_TMP_BY_X => |cpu| {
        // @pseudocode: tmp.lo += X
        todo!("Action INC_TMP_BY_X");
    },
    INC_TMP_BY_X_INVOKE_OP_DAT => |cpu| {
        // @pseudocode: tmp.lo += X, op(dat)
        todo!("Action INC_TMP_BY_X_INVOKE_OP_DAT");
    },
    INC_TMP_BY_Y => |cpu| {
        // @pseudocode: tmp.lo += Y
        todo!("Action INC_TMP_BY_Y");
    },
    INC_TMP_BY_Y_INVOKE_OP_DAT => |cpu| {
        // @pseudocode: tmp.lo += Y, op(dat)
        todo!("Action INC_TMP_BY_Y_INVOKE_OP_DAT");
    },
    SAVE_PC_HI => |cpu| {
        // @pseudocode: dat = PC.hi
        cpu.internal.dat = (cpu.regs.pc.get() >> 8) as u8;
    },
    SAVE_PC_LO => |cpu| {
        // @pseudocode: dat = PC.lo
        cpu.internal.dat = (cpu.regs.pc.get() & 0xFF) as u8;
    },
    SAVE_RD_VAL => |cpu| {
        // @pseudocode: dat = rd_val
        cpu.internal.dat = cpu.internal.rd_val;
    },
    SAVE_P => |cpu| {
        // @pseudocode: dat = P
        todo!("Action SAVE_P");
    },
    SAVE_P_BRK => |cpu| {
        // @pseudocode: dat = P+B
        todo!("Action SAVE_P_BRK");
    },
    SAVE_RD_VAL_STOP_IF_NO_BRANCH => |cpu| {
        // @pseudocode: dat = rd_val, done if branch not taken
        todo!("Action SAVE_RD_VAL_STOP_IF_NO_BRANCH");
    },
    SAVE_RD_VAL_INC_TMP => |cpu| {
        // @pseudocode: dat = rd_val, tmp.lo += 1
        cpu.internal.dat = cpu.internal.rd_val;
        cpu.internal.tmp_lo = cpu.internal.tmp_lo.wrapping_add(1);
    },
    SET_RESET_VEC => |cpu| {
        // @pseudocode: tmp.hi = 0xFF, tmp.lo = 0xFC, P.I = 1
        cpu.internal.tmp_hi = 0xFF;
        cpu.internal.tmp_lo = 0xFC;
        cpu.regs.p.update(|p| p.with_i(true));
    },
    SET_IRQ_VEC => |cpu| {
        // @pseudocode: tmp.hi = 0xFF, tmp.lo = 0xFE, P.I = 1
        todo!("Action SET_IRQ_VEC");
    },
    SET_NMI_VEC => |cpu| {
        // @pseudocode: tmp.hi = 0xFF, tmp.lo = 0xFA, P.I = 1
        todo!("Action SET_NMI_VEC");
    },
}
