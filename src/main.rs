mod reader;
mod patterns;
mod counters;
mod report;
mod errors;

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut count = 0;

    for line in stdin.lock().lines() {
        if line.is_ok() {
            count += 1;
        }
    }

    println!("FrictionSpot scaffolding report:");
    println!("Lines processed: {}", count);
}
