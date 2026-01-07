mod components;
mod nes;
mod nes_file;

use std::{fs::File, path::PathBuf};

use clap::Parser;

use crate::{
    components::{EmuError, tracer::Tracer},
    nes::NESSystem,
    nes_file::NesFile,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    rom_path: PathBuf,

    #[arg(short, long)]
    trace: Vec<String>,

    #[arg(long)]
    trace_file: Option<PathBuf>,

    #[arg(long, short, help = "Number of CPU cycles to run before exiting")]
    cycles: Option<u64>,
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

    let run_result = (|| {
        nes.start_simulation()?;
        nes.run(args.cycles)
    })();

    nes.end_simulation();

    match run_result {
        Ok(_) => {}
        Err(e) => match e {
            EmuError::StopEmulation => {}
            EmuError::CycleLimitReached => {
                println!("Cycle limit exceeded");
            }
            _ => {
                eprintln!("Emulation error: {}", e);

                eprintln!("Register dump:");
                let regs = nes.get_regs();
                eprintln!("A:  0x{:02X}   S: 0x{:02X}", *regs.a, *regs.s);
                eprintln!("X:  0x{:02X}   Y: 0x{:02X}", *regs.x, *regs.y);
                eprintln!("P:  {}", *regs.p);
                eprintln!("PC: 0x{:04X}", *regs.pc);
            }
        },
    }

    println!("Total CPU cycles executed: {}", nes.get_tick_count());
}
