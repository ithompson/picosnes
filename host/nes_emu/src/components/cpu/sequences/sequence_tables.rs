use super::*;

seq!(RESET_SEQUENCE => [
    (DEC_S, ReadStk),
    (DEC_S, ReadStk),
    (DEC_S, ReadStk),
    (SET_RESET_VEC, ReadTmp),
    (SET_PC_LO_INC_TMP, ReadTmp),
    (SET_PC_HI, ReadPC),
]);
seq!(IRQ_SEQUENCE => [
    (SAVE_PC_HI, IncPushStk),
    (SAVE_PC_LO, PushStk),
    (SAVE_P, PushStk),
    (SET_IRQ_VEC, ReadTmp),
    (SET_PC_LO_INC_TMP, ReadTmp),
    (SET_PC_HI, ReadPC),
]);
seq!(NMI_SEQUENCE => [
    (SAVE_PC_HI, IncPushStk),
    (SAVE_PC_LO, PushStk),
    (SAVE_P, PushStk),
    (SET_NMI_VEC, ReadTmp),
    (SET_PC_LO_INC_TMP, ReadTmp),
    (SET_PC_HI, ReadPC),
]);
seq!(DISPATCH_SEQUENCE => [
    (DISPATCH, IncReadPC),
]);
seq!(ABS_JMP_SEQUENCE => [
    (SET_TMP_LO, IncReadPC),
    (SET_PC_FULL, ReadPC),
]);
seq!(ABS_JSR_SEQUENCE => [
    (SET_TMP_LO, IncReadStk),
    (SAVE_PC_HI, PushStk),
    (SAVE_PC_LO, PushStk),
    (NOP, ReadPC),
    (SET_PC_FULL, ReadPC),
]);
seq!(ABS_RMW_SEQUENCE => [
    (SET_TMP_LO, IncReadPC),
    (SET_TMP_HI, IncReadTmp),
    (SAVE_RD_VAL, WriteTmp),
    (INVOKE_OP_DAT, WriteTmp),
    (NOP, ReadPC),
]);
seq!(ABS_READ_SEQUENCE => [
    (SET_TMP_LO, IncReadPC),
    (SET_TMP_HI, IncReadTmp),
    (INVOKE_OP_RD_VAL, ReadPC),
]);
seq!(ABS_WRITE_SEQUENCE => [
    (SET_TMP_LO, IncReadPC),
    (SET_TMP_HI_INVOKE_OP_DAT, IncWriteTmp),
    (NOP, ReadPC),
]);
seq!(ABSIND_JMP_SEQUENCE => [
    (SET_TMP_LO, IncReadPC),
    (SET_TMP_HI, IncReadTmp),
    (SET_PC_LO_INC_TMP, ReadTmp),
    (SET_PC_HI, ReadPC),
]);
seq!(ABSX_RMW_SEQUENCE => [
    (SET_TMP_LO, IncReadPC),
    (SET_TMP_HI_INC_BY_X_RECORD_CARRY, IncReadTmp),
    (CARRY_INTO_TMP_HI, ReadTmp),
    (SAVE_RD_VAL, WriteTmp),
    (INVOKE_OP_DAT, WriteTmp),
    (NOP, ReadPC),
]);
seq!(ABSX_READ_SEQUENCE => [
    (SET_TMP_LO, IncReadPC),
    (SET_TMP_HI_INC_BY_X_SKIP_IF_NO_CARRY, IncReadTmp),
    (INC_TMP_HI, ReadTmp),
    (INVOKE_OP_RD_VAL, ReadPC),
]);
seq!(ABSX_WRITE_SEQUENCE => [
    (SET_TMP_LO, IncReadPC),
    (SET_TMP_HI_INC_BY_X_RECORD_CARRY, IncReadPC),
    (CARRY_INTO_TMP_HI_INVOKE_OP_DAT, WriteTmp),
    (NOP, ReadPC),
]);
seq!(ABSY_READ_SEQUENCE => [
    (SET_TMP_LO, IncReadPC),
    (SET_TMP_HI_INC_BY_Y_SKIP_IF_NO_CARRY, IncReadTmp),
    (INC_TMP_HI, ReadTmp),
    (INVOKE_OP_RD_VAL, ReadPC),
]);
seq!(ABSY_WRITE_SEQUENCE => [
    (SET_TMP_LO, IncReadPC),
    (SET_TMP_HI_INC_BY_Y_RECORD_CARRY, IncReadPC),
    (CARRY_INTO_TMP_HI_INVOKE_OP_DAT, WriteTmp),
    (NOP, ReadPC),
]);
seq!(ACC_RMW_SEQUENCE => [
    (INVOKE_OP_A, ReadPC),
]);
seq!(IMM_READ_SEQUENCE => [
    (INVOKE_OP_RD_VAL, IncReadPC),
]);
seq!(IMP_BRK_SEQUENCE => [
    (SAVE_PC_HI, IncPushStk),
    (SAVE_PC_LO, PushStk),
    (SAVE_P_BRK, PushStk),
    (SET_IRQ_VEC, ReadTmp),
    (SET_PC_LO_INC_TMP, ReadTmp),
    (SET_PC_HI, ReadPC),
]);
seq!(IMP_NOMEM_SEQUENCE => [
    (INVOKE_OP, ReadPC),
]);
seq!(IMP_POP_SEQUENCE => [
    (NOP, ReadStk),
    (NOP, PopStk),
    (INVOKE_OP_RD_VAL, ReadPC),
]);
seq!(IMP_PUSH_SEQUENCE => [
    (INVOKE_OP_DAT, PushStk),
    (NOP, ReadPC),
]);
seq!(IMP_RTI_SEQUENCE => [
    (NOP, ReadStk),
    (NOP, PopStk),
    (SET_P, PopStk),
    (SET_PC_LO, PopStk),
    (SET_PC_HI, ReadPC),
]);
seq!(IMP_RTS_SEQUENCE => [
    (NOP, ReadStk),
    (NOP, PopStk),
    (SET_PC_LO, PopStk),
    (SET_PC_HI, ReadPC),
    (NOP, IncReadPC),
]);
seq!(INDX_RMW_SEQUENCE => [
    (SET_TMP_ZP, IncReadTmp),
    (INC_TMP_BY_X, ReadTmp),
    (SAVE_RD_VAL_INC_TMP, ReadTmp),
    (SET_TMP_FULL, ReadTmp),
    (SAVE_RD_VAL, WriteTmp),
    (INVOKE_OP_DAT, WriteTmp),
    (NOP, ReadPC),
]);
seq!(INDX_READ_SEQUENCE => [
    (SET_TMP_ZP, IncReadTmp),
    (INC_TMP_BY_X, ReadTmp),
    (SAVE_RD_VAL_INC_TMP, ReadTmp),
    (SET_TMP_FULL, ReadTmp),
    (INVOKE_OP_RD_VAL, ReadPC),
]);
seq!(INDX_WRITE_SEQUENCE => [
    (SET_TMP_ZP, IncReadTmp),
    (INC_TMP_BY_X, ReadTmp),
    (SAVE_RD_VAL_INC_TMP, ReadTmp),
    (SET_TMP_FULL_INVOKE_OP_DAT, WriteTmp),
    (NOP, ReadPC),
]);
seq!(INDY_RMW_SEQUENCE => [
    (SET_TMP_ZP, IncReadTmp),
    (SAVE_RD_VAL_INC_TMP, ReadTmp),
    (SET_TMP_FULL_INC_BY_Y_RECORD_CARRY, ReadTmp),
    (CARRY_INTO_TMP_HI, ReadTmp),
    (SAVE_RD_VAL, WriteTmp),
    (INVOKE_OP_DAT, WriteTmp),
    (NOP, ReadPC),
]);
seq!(INDY_READ_SEQUENCE => [
    (SET_TMP_ZP, IncReadTmp),
    (SAVE_RD_VAL_INC_TMP, ReadTmp),
    (SET_TMP_FULL_INC_BY_Y_SKIP_IF_NO_CARRY, ReadTmp),
    (INC_TMP_HI, ReadTmp),
    (INVOKE_OP_RD_VAL, ReadPC),
]);
seq!(INDY_WRITE_SEQUENCE => [
    (SET_TMP_ZP, IncReadTmp),
    (SAVE_RD_VAL_INC_TMP, ReadTmp),
    (SET_TMP_FULL_INC_BY_Y_RECORD_CARRY, ReadTmp),
    (CARRY_INTO_TMP_HI_INVOKE_OP_DAT, WriteTmp),
    (NOP, ReadPC),
]);
seq!(REL_BRANCH_SEQUENCE => [
    (SAVE_RD_VAL_STOP_IF_NO_BRANCH, IncReadPC),
    (ADVANCE_PC_BY_DAT_STOP_IF_NO_CARRY, ReadPC),
    (INC_PC_HI, ReadPC),
]);
seq!(ZP_RMW_SEQUENCE => [
    (SET_TMP_ZP, IncReadTmp),
    (SAVE_RD_VAL, WriteTmp),
    (INVOKE_OP_DAT, WriteTmp),
    (NOP, ReadPC),
]);
seq!(ZP_READ_SEQUENCE => [
    (SET_TMP_ZP, IncReadTmp),
    (INVOKE_OP_RD_VAL, ReadPC),
]);
seq!(ZP_WRITE_SEQUENCE => [
    (SET_TMP_ZP_INVOKE_OP_DAT, IncWriteTmp),
    (NOP, ReadPC),
]);
seq!(ZPX_RMW_SEQUENCE => [
    (SET_TMP_ZP, IncReadTmp),
    (INC_TMP_BY_X, ReadTmp),
    (SAVE_RD_VAL, WriteTmp),
    (INVOKE_OP_DAT, WriteTmp),
    (NOP, ReadPC),
]);
seq!(ZPX_READ_SEQUENCE => [
    (SET_TMP_ZP, IncReadTmp),
    (INC_TMP_BY_X, ReadTmp),
    (INVOKE_OP_RD_VAL, ReadPC),
]);
seq!(ZPX_WRITE_SEQUENCE => [
    (SET_TMP_ZP, IncReadTmp),
    (INC_TMP_BY_X_INVOKE_OP_DAT, WriteTmp),
    (NOP, ReadPC),
]);
seq!(ZPY_READ_SEQUENCE => [
    (SET_TMP_ZP, IncReadTmp),
    (INC_TMP_BY_Y, ReadTmp),
    (INVOKE_OP_RD_VAL, ReadPC),
]);
seq!(ZPY_WRITE_SEQUENCE => [
    (SET_TMP_ZP, IncReadTmp),
    (INC_TMP_BY_Y_INVOKE_OP_DAT, WriteTmp),
    (NOP, ReadPC),
]);
