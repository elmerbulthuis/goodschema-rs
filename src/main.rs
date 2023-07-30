pub mod documents;
mod programs;
pub mod schemas;
pub mod utils;

use clap::Parser;
use programs::{run_program, ProgramOptions};

fn main() {
    let options = ProgramOptions::parse();
    run_program(options).unwrap()
}
