mod components;
mod nes;
mod nes_file;

use std::{fs::File, path::PathBuf};

use clap::Parser;

use crate::{components::tracer::Tracer, nes::NESSystem, nes_file::NesFile};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    rom_path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut rom_file = File::open(args.rom_path).expect("Failed to open ROM file");
    let rom = NesFile::from_stream(&mut rom_file).expect("Failed to read NES file");

    let tracer = Tracer::new();
    let mut nes = NESSystem::new(&tracer, rom);

    nes.run();
}
