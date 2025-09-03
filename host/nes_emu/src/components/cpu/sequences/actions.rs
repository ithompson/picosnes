use super::*;

action_defs! {
    NOP => |cpu| {
        // @pseudocode:
        unimplemented!("Action NOP");
    },
    DISPATCH => |cpu| {
        // @pseudocode: dispatch(rd_val)
        unimplemented!("Action DISPATCH");
    },
    SET_P => |cpu| {
        // @pseudocode: P = rd_val
        unimplemented!("Action SET_P");
    },
    DEC_S => |cpu| {
        // @pseudocode: S -= 1
        unimplemented!("Action DEC_S");
    },
    SET_PC_LO => |cpu| {
        // @pseudocode: PC.lo = rd_val
        unimplemented!("Action SET_PC_LO");
    },
    SET_PC_HI => |cpu| {
        // @pseudocode: PC.hi = rd_val
        unimplemented!("Action SET_PC_HI");
    },
    SET_PC_FULL => |cpu| {
        // @pseudocode: PC.hi = rd_val, PC.lo = tmp.lo
        unimplemented!("Action SET_PC_FULL");
    },
    SET_PC_LO_INC_TMP => |cpu| {
        // @pseudocode: PC.lo = rd_val, tmp.lo += 1
        unimplemented!("Action SET_PC_LO_INC_TMP");
    },
    ADVANCE_PC_BY_DAT_STOP_IF_NO_CARRY => |cpu| {
        // @pseudocode: PC.lo += dat, done if no carry
        unimplemented!("Action ADVANCE_PC_BY_DAT_STOP_IF_NO_CARRY");
    },
    INC_PC_HI => |cpu| {
        // @pseudocode: PC.hi += 1
        unimplemented!("Action INC_PC_HI");
    },
    INVOKE_OP => |cpu| {
        // @pseudocode: op()
        unimplemented!("Action INVOKE_OP");
    },
    INVOKE_OP_A => |cpu| {
        // @pseudocode: op(A)
        unimplemented!("Action INVOKE_OP_A");
    },
    INVOKE_OP_DAT => |cpu| {
        // @pseudocode: op(dat)
        unimplemented!("Action INVOKE_OP_DAT");
    },
    INVOKE_OP_RD_VAL => |cpu| {
        // @pseudocode: op(rd_val)
        unimplemented!("Action INVOKE_OP_RD_VAL");
    },
    SET_TMP_LO => |cpu| {
        // @pseudocode: tmp.lo = rd_val
        unimplemented!("Action SET_TMP_LO");
    },
    SET_TMP_HI => |cpu| {
        // @pseudocode: tmp.hi = rd_val
        unimplemented!("Action SET_TMP_HI");
    },
    SET_TMP_FULL => |cpu| {
        // @pseudocode: tmp.hi = rd_val, tmp.lo = dat
        unimplemented!("Action SET_TMP_FULL");
    },
    SET_TMP_ZP => |cpu| {
        // @pseudocode: tmp.lo = rd_val, tmp.hi = 0
        unimplemented!("Action SET_TMP_ZP");
    },
    SET_TMP_HI_INVOKE_OP_DAT => |cpu| {
        // @pseudocode: tmp.hi = rd_val, op(dat)
        unimplemented!("Action SET_TMP_HI_INVOKE_OP_DAT");
    },
    SET_TMP_ZP_INVOKE_OP_DAT => |cpu| {
        // @pseudocode: tmp.lo = rd_val, tmp.hi = 0, op(dat)
        unimplemented!("Action SET_TMP_ZP_INVOKE_OP_DAT");
    },
    SET_TMP_FULL_INVOKE_OP_DAT => |cpu| {
        // @pseudocode: tmp.hi = rd_val, tmp.lo = dat, op(dat)
        unimplemented!("Action SET_TMP_FULL_INVOKE_OP_DAT");
    },
    SET_TMP_HI_INC_BY_X_RECORD_CARRY => |cpu| {
        // @pseudocode: tmp.hi = rd_val, tmp.lo += X, record carry
        unimplemented!("Action SET_TMP_HI_INC_BY_X_RECORD_CARRY");
    },
    SET_TMP_HI_INC_BY_X_SKIP_IF_NO_CARRY => |cpu| {
        // @pseudocode: tmp.hi = rd_val, tmp.lo += X, skip next if no carry
        unimplemented!("Action SET_TMP_HI_INC_BY_X_SKIP_IF_NO_CARRY");
    },
    SET_TMP_HI_INC_BY_Y_RECORD_CARRY => |cpu| {
        // @pseudocode: tmp.hi = rd_val, tmp.lo += Y, record carry
        unimplemented!("Action SET_TMP_HI_INC_BY_Y_RECORD_CARRY");
    },
    SET_TMP_HI_INC_BY_Y_SKIP_IF_NO_CARRY => |cpu| {
        // @pseudocode: tmp.hi = rd_val, tmp.lo += Y, skip next if no carry
        unimplemented!("Action SET_TMP_HI_INC_BY_Y_SKIP_IF_NO_CARRY");
    },
    SET_TMP_FULL_INC_BY_Y_RECORD_CARRY => |cpu| {
        // @pseudocode: tmp.hi = rd_val, tmp.lo = dat + Y, record carry
        unimplemented!("Action SET_TMP_FULL_INC_BY_Y_RECORD_CARRY");
    },
    SET_TMP_FULL_INC_BY_Y_SKIP_IF_NO_CARRY => |cpu| {
        // @pseudocode: tmp.hi = rd_val, tmp.lo = dat + Y, skip next if no carry
        unimplemented!("Action SET_TMP_FULL_INC_BY_Y_SKIP_IF_NO_CARRY");
    },
    INC_TMP_HI => |cpu| {
        // @pseudocode: tmp.hi += 1
        unimplemented!("Action INC_TMP_HI");
    },
    CARRY_INTO_TMP_HI => |cpu| {
        // @pseudocode: tmp.hi += carry
        unimplemented!("Action CARRY_INTO_TMP_HI");
    },
    CARRY_INTO_TMP_HI_INVOKE_OP_DAT => |cpu| {
        // @pseudocode: tmp.hi += carry, op(dat)
        unimplemented!("Action CARRY_INTO_TMP_HI_INVOKE_OP_DAT");
    },
    INC_TMP_BY_X => |cpu| {
        // @pseudocode: tmp.lo += X
        unimplemented!("Action INC_TMP_BY_X");
    },
    INC_TMP_BY_X_INVOKE_OP_DAT => |cpu| {
        // @pseudocode: tmp.lo += X, op(dat)
        unimplemented!("Action INC_TMP_BY_X_INVOKE_OP_DAT");
    },
    INC_TMP_BY_Y => |cpu| {
        // @pseudocode: tmp.lo += Y
        unimplemented!("Action INC_TMP_BY_Y");
    },
    INC_TMP_BY_Y_INVOKE_OP_DAT => |cpu| {
        // @pseudocode: tmp.lo += Y, op(dat)
        unimplemented!("Action INC_TMP_BY_Y_INVOKE_OP_DAT");
    },
    SAVE_PC_HI => |cpu| {
        // @pseudocode: dat = PC.hi
        unimplemented!("Action SAVE_PC_HI");
    },
    SAVE_PC_LO => |cpu| {
        // @pseudocode: dat = PC.lo
        unimplemented!("Action SAVE_PC_LO");
    },
    SAVE_RD_VAL => |cpu| {
        // @pseudocode: dat = rd_val
        unimplemented!("Action SAVE_RD_VAL");
    },
    SAVE_P => |cpu| {
        // @pseudocode: dat = P
        unimplemented!("Action SAVE_P");
    },
    SAVE_P_BRK => |cpu| {
        // @pseudocode: dat = P+B
        unimplemented!("Action SAVE_P_BRK");
    },
    SAVE_RD_VAL_STOP_IF_NO_BRANCH => |cpu| {
        // @pseudocode: dat = rd_val, done if branch not taken
        unimplemented!("Action SAVE_RD_VAL_STOP_IF_NO_BRANCH");
    },
    SAVE_RD_VAL_INC_TMP => |cpu| {
        // @pseudocode: dat = rd_val, tmp.lo += 1
        unimplemented!("Action SAVE_RD_VAL_INC_TMP");
    },
    SET_RESET_VEC => |cpu| {
        // @pseudocode: tmp.hi = 0xFF, tmp.lo = 0xFC, P.I = 1
        unimplemented!("Action SET_RESET_VEC");
    },
    SET_IRQ_VEC => |cpu| {
        // @pseudocode: tmp.hi = 0xFF, tmp.lo = 0xFE, P.I = 1
        unimplemented!("Action SET_IRQ_VEC");
    },
    SET_NMI_VEC => |cpu| {
        // @pseudocode: tmp.hi = 0xFF, tmp.lo = 0xFA, P.I = 1
        unimplemented!("Action SET_NMI_VEC");
    },
}
