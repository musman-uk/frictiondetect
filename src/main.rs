mod counters;
mod errors;
mod patterns;
mod reader;
mod report;

use crate::errors::FrictionError;
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{self, BufRead, BufReader, IsTerminal};
use std::path::PathBuf;

/// FrictionDetect — analyse structured JSONL logs for friction patterns.
///
/// This tool consumes JSON Lines (JSONL) logs produced by tools like LogTidy
/// and identifies friction indicators such as:
///   • errors and warnings
///   • slow operations
///   • long gaps between events
///   • repeated failures
#[derive(Parser)]
#[command(
    name = "frictiondetect",
    version,
    about = "Analyse structured logs for friction patterns",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Subcommands
#[derive(Subcommand)]
enum Commands {
    /// Analyse structured JSONL logs (from file or stdin)
    Analyse {
        /// Optional input file (JSONL). If omitted, stdin is used.
        #[arg(short, long)]
        file: Option<PathBuf>,
    },

    /// Show example input/output
    Sample,

    /// Show condensed documentation
    Docs,

    /// Show tool information
    About,
}

fn main() -> Result<(), FrictionError> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Analyse { file }) => {
            let entries = match file {
                Some(path) => read_file(&path)?,
                None => reader::read_stdin()?,
            };

            let summary = patterns::analyse(&entries);
            report::print(&summary);
        }

        Some(Commands::Sample) => {
            print_sample();
        }

        Some(Commands::Docs) => {
            print_docs();
        }

        Some(Commands::About) => {
            print_about();
        }

        None => {
            print_main_menu();
        }
    }

    Ok(())
}

/// Read JSONL entries from a file
fn read_file(path: &PathBuf) -> Result<Vec<reader::Entry>, FrictionError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut entries = Vec::new();
    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let mut entry: reader::Entry = serde_json::from_str(&line)?;
        if entry.line == 0 {
            entry.line = idx + 1;
        }
        entries.push(entry);
    }

    Ok(entries)
}

/// Main menu (shown when no subcommand is provided)
fn print_main_menu() {
    println!("FrictionDetect — Analyse structured logs for friction patterns");
    println!();
    println!("Commands:");
    println!("  analyse    Analyse structured JSONL logs");
    println!("  sample     Show example input/output");
    println!("  docs       Show condensed documentation");
    println!("  about      Show tool information");
    println!("  help       Show help");
}

/// Example usage and sample JSONL
fn print_sample() {
    println!("Example input (JSONL):");
    println!(r#"  {{"line":1,"timestamp":"2024-01-01T12:00:00Z","level":"INFO","message":"Start","fields":{{}}}}"#);
    println!(r#"  {{"line":2,"timestamp":"2024-01-01T12:00:01Z","level":"ERROR","message":"Failed","fields":{{"duration_ms":800}}}}"#);
    println!(r#"  {{"line":3,"timestamp":"2024-01-01T12:00:02Z","level":"INFO","message":"Retry","fields":{{}}}}"#);
    println!();
    println!("Run:");
    println!("  frictiondetect analyse --file sample.jsonl");
}

/// Condensed documentation
fn print_docs() {
    println!("Purpose:");
    println!("  Analyse structured JSONL logs and identify friction patterns.");
    println!();
    println!("Usage:");
    println!("  frictiondetect analyse --file <path>");
    println!("  cat logs.jsonl | frictiondetect analyse");
    println!();
    println!("Input:");
    println!("  - JSON Lines (JSONL), one JSON object per line");
    println!("  - Typically produced by LogTidy");
    println!();
    println!("Output:");
    println!("  - Errors and warnings");
    println!("  - Slow operations");
    println!("  - Long gaps between events");
    println!("  - Repeated failures");
    println!();
    println!("Examples:");
    println!("  For example input/output, run:");
    println!("    frictiondetect sample");
    println!();
    println!("Full Documentation:");
    println!("  https://github.com/musman-uk/frictiondetect/blob/main/Documentation.md");
}

/// About information
fn print_about() {
    println!(
        "FrictionDetect v{} · {}",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_LICENSE")
    );
    println!("https://github.com/musman-uk/frictiondetect");
    println!();

    if io::stdout().is_terminal() {
        println!(
            "\x1b]8;;https://github.com/musman-uk\x1b\\\x1b[3mMohammed Usman\x1b[0m\x1b]8;;\x1b\\"
        );
        println!();
        println!("\x1b[2m© Mohammed Usman\x1b[0m");
    } else {
        println!("Mohammed Usman");
        println!();
        println!("© Mohammed Usman");
    }
}