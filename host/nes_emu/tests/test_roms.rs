use std::{env, fs::File, path::PathBuf};

use nes_emu::{
    components::{EmuError, tracer::Tracer},
    nes::NESSystem,
    nes_file::NesFile,
};

fn run_test_rom(rom_path: &str, tick_limit: u64) {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    path.pop();
    path.pop();
    path.push("tests/nes-test-roms");
    path.push(rom_path);

    let mut rom_file = File::open(&path).expect("Failed to open ROM file");
    let rom = NesFile::from_stream(&mut rom_file).expect("Failed to read NES file");

    let tracer = Tracer::new::<&str>(&[], None);
    let mut nes = NESSystem::new(&tracer, rom);

    let run_result = (|| {
        nes.start_simulation()?;
        nes.run(Some(tick_limit))
    })();

    nes.end_simulation();

    assert_eq!(run_result.err(), Some(EmuError::StopEmulation));
}

#[test]
fn test_single_01_implied() {
    run_test_rom("nes_instr_test/rom_singles/01-implied.nes", 1_703_000);
}

#[test]
fn test_single_02_immediate() {
    run_test_rom("nes_instr_test/rom_singles/02-immediate.nes", 1_541_000);
}

#[test]
fn test_single_03_zero_page() {
    run_test_rom("nes_instr_test/rom_singles/03-zero_page.nes", 2_053_000);
}

#[test]
fn test_single_04_zp_xy() {
    run_test_rom("nes_instr_test/rom_singles/04-zp_xy.nes", 5_030_000);
}

#[test]
fn test_single_05_absolute() {
    run_test_rom("nes_instr_test/rom_singles/05-absolute.nes", 1_954_000);
}

#[test]
fn test_single_06_abs_xy() {
    run_test_rom("nes_instr_test/rom_singles/06-abs_xy.nes", 7_080_000);
}

#[test]
fn test_single_07_ind_x() {
    run_test_rom("nes_instr_test/rom_singles/07-ind_x.nes", 2_919_000);
}

#[test]
fn test_single_08_ind_y() {
    run_test_rom("nes_instr_test/rom_singles/08-ind_y.nes", 2_747_000);
}

#[test]
fn test_single_09_branches() {
    run_test_rom("nes_instr_test/rom_singles/09-branches.nes", 794_000);
}

#[test]
fn test_single_10_stack() {
    run_test_rom("nes_instr_test/rom_singles/10-stack.nes", 3_614_000);
}

#[test]
fn test_single_11_special() {
    run_test_rom("nes_instr_test/rom_singles/11-special.nes", 1_139_000);
}

#[test]
fn test_cpu_reset_registers() {
    run_test_rom("cpu_reset/registers.nes", 4_044_000);
}

#[test]
fn test_cpu_reset_ram() {
    run_test_rom("cpu_reset/ram_after_reset.nes", 4_046_000);
}
