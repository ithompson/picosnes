use super::{ArchPSR, ArchRegs};

pub use proptest::prelude::*;

prop_compose! {
    pub fn arch_psr_arb()(n: bool, v: bool, d: bool, i: bool, z: bool, c: bool) -> ArchPSR {
        ArchPSR { n, v, d, i, z, c }
    }
}

prop_compose! {
    pub fn arch_regs_arb()(a: u8, x: u8, y: u8, s: u8, p in arch_psr_arb(), pc: u16) -> ArchRegs<'static> {
        ArchRegs {
            a: a.into(),
            x: x.into(),
            y: y.into(),
            s: s.into(),
            p: p.into(),
            pc: pc.into(),
        }
    }
}

#[macro_export]
macro_rules! define_op_test_helper {
        //    logic: yes, update_regs: yes, expected_val: yes
        ($test_name:ident($regs_name:ident, $val_name:ident, $num_cases:expr) { op: $op:ident, logic: { $($logic:stmt)* }, update_regs: { $($reg_name:ident: $reg_value:expr),* }, expected_val: $expected_val:expr }) => {
            proptest::proptest! {
                #![proptest_config($crate::components::cpu::ops::test_helpers::ProptestConfig::with_cases($num_cases))]
                #[test]
                fn $test_name($regs_name in $crate::components::cpu::ops::test_helpers::arch_regs_arb(), $val_name: u8) {
                    $($logic)*

                    let updated_regs = ArchRegs {
                        $($reg_name: $reg_value.into(),)*
                        ..$regs_name.clone()
                    };
                    let expected_val = $expected_val;

                    let mut op_regs = $regs_name.clone();
                    let mut op_val = $val_name;
                    $op(&mut op_regs, &mut op_val);
                    $crate::components::cpu::ops::test_helpers::prop_assert_eq!(op_regs, updated_regs);
                    $crate::components::cpu::ops::test_helpers::prop_assert_eq!(op_val, expected_val);
                }
            }
        };
        // Combinations:
        //    logic: yes, update_regs: yes, expected_val: no
        ($test_name:ident($regs_name:ident, $val_name:ident, $num_cases:expr) { op: $op:ident, logic: $logic:tt, update_regs: $update_regs:tt }) => {
            $crate::components::cpu::ops::test_helpers::define_op_test_helper!($test_name($regs_name, $val_name, $num_cases) {
                op: $op,
                logic: $logic,
                update_regs: $update_regs,
                expected_val: $val_name
            });
        };
        //    logic: yes, update_regs: no , expected_val: yes
        ($test_name:ident($regs_name:ident, $val_name:ident, $num_cases:expr) { op: $op:ident, logic: $logic:tt, expected_val: $expected_val:expr }) => {
            $crate::components::cpu::ops::test_helpers::define_op_test_helper!($test_name($regs_name, $val_name, $num_cases) {
                op: $op,
                logic: $logic,
                update_regs: {},
                expected_val: $expected_val
            });
        };
        //    logic: yes, update_regs: no , expected_val: no
        ($test_name:ident($regs_name:ident, $val_name:ident, $num_cases:expr) { op: $op:ident, logic: $logic:tt }) => {
            $crate::components::cpu::ops::test_helpers::define_op_test_helper!($test_name($regs_name, $val_name, $num_cases) {
                op: $op,
                logic: $logic,
                update_regs: {},
                expected_val: $val_name
            });
        };
        //    logic: no , update_regs: yes, expected_val: yes
        ($test_name:ident($regs_name:ident, $val_name:ident, $num_cases:expr) { op: $op:ident, update_regs: $update_regs:tt, expected_val: $expected_val:expr }) => {
            $crate::components::cpu::ops::test_helpers::define_op_test_helper!($test_name($regs_name, $val_name, $num_cases) {
                op: $op,
                logic: {},
                update_regs: $update_regs,
                expected_val: $expected_val
            });
        };
        //    logic: no , update_regs: yes, expected_val: no
        ($test_name:ident($regs_name:ident, $val_name:ident, $num_cases:expr) { op: $op:ident, update_regs: $update_regs:tt }) => {
            $crate::components::cpu::ops::test_helpers::define_op_test_helper!($test_name($regs_name, $val_name, $num_cases) {
                op: $op,
                logic: {},
                update_regs: $update_regs,
                expected_val: $val_name
            });
        };
        //    logic: no , update_regs: no , expected_val: yes
        ($test_name:ident($regs_name:ident, $val_name:ident, $num_cases:expr) { op: $op:ident, expected_val: $expected_val:expr }) => {
            $crate::components::cpu::ops::test_helpers::define_op_test_helper!($test_name($regs_name, $val_name, $num_cases) {
                op: $op,
                logic: {},
                update_regs: {},
                expected_val: $expected_val
            });
        };
        //    logic: no , update_regs: no , expected_val: no
        ($test_name:ident($regs_name:ident, $val_name:ident, $num_cases:expr) { op: $op:ident }) => {
            $crate::components::cpu::ops::test_helpers::define_op_test_helper!($test_name($regs_name, $val_name, $num_cases) {
                op: $op,
                logic: {},
                update_regs: {},
                expected_val: $val_name
            });
        };
    }
pub(crate) use define_op_test_helper;

macro_rules! define_op_test {
        ($test_name:ident($regs_name:ident, $val_name:ident, $num_cases:expr) { $($body:tt)* }) => {
            $crate::components::cpu::ops::test_helpers::define_op_test_helper!($test_name($regs_name, $val_name, $num_cases) { $($body)* });
        };
        ($test_name:ident($regs_name:ident, $val_name:ident) { $($body:tt)* }) => {
            $crate::components::cpu::ops::test_helpers::define_op_test_helper!($test_name($regs_name, $val_name, 1000) { $($body)* });
        };
        ($test_name:ident($regs_name:ident) { $($body:tt)* }) => {
            $crate::components::cpu::ops::test_helpers::define_op_test_helper!($test_name($regs_name, val, 1000) { $($body)* });
        };
        ($test_name:ident() { $($body:tt)* }) => {
            $crate::components::cpu::ops::test_helpers::define_op_test_helper!($test_name(regs, val, 1000) { $($body)* });
        };
    }
pub(crate) use define_op_test;
