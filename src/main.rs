pub mod documents;
pub mod generators;
pub mod schemas;
pub mod utils;

mod programs;

use clap::Parser;
use programs::{run_program, ProgramOptions};

fn main() {
    let options = ProgramOptions::parse();
    run_program(options).unwrap()
}
