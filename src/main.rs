mod documents;
mod generators;
mod models;
mod schemas;
mod selectors;
mod utils;

mod programs;

use clap::Parser;
use programs::{run_program, ProgramOptions};

fn main() {
    let options = ProgramOptions::parse();
    run_program(options).unwrap()
}
