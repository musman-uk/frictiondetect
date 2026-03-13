mod counters;
mod errors;
mod patterns;
mod reader;
mod report;

use crate::errors::FrictionError;

fn main() {
    if let Err(e) = run() {
        eprintln!("frictiondetect error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), FrictionError> {
    let entries = reader::read_stdin()?;
    let summary = patterns::analyse(&entries);
    report::print(&summary);
    Ok(())
}
