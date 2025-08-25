mod components;
mod nes;

use clap::Parser;

use crate::{components::tracer::Tracer, nes::NESSystem};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {}

fn main() {
    let args = Args::parse();

    let tracer = Tracer::new();
    let mut nes = NESSystem::new(&tracer);

    nes.run();
}
