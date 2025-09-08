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

    #[arg(short, long)]
    trace: Vec<String>,

    #[arg(long)]
    trace_file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mut rom_file = File::open(args.rom_path).expect("Failed to open ROM file");
    let rom = NesFile::from_stream(&mut rom_file).expect("Failed to read NES file");

    let trace_file = args
        .trace_file
        .as_ref()
        .map(|path| File::create(path).expect("Failed to create trace output file"));
    let tracer = Tracer::new(&args.trace, trace_file);
    let mut nes = NESSystem::new(&tracer, rom);

    nes.run();
}
