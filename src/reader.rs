use crate::errors::FrictionPointerError;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;
use std::io::{self, BufRead};

/// Represents a single structured JSONL log record.
/// All fields use `#[serde(default)]` so standalone JSONL logs without an explicit
/// `line` number or `fields` map still parse cleanly; missing `line` values default
/// to `0` and are backfilled with the 1-based input line number during ingestion.
#[derive(Debug, Deserialize)]
pub struct Entry {
    #[serde(default)]
    pub line: usize,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(default)]
    pub level: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub fields: Option<serde_json::Map<String, Value>>,
}

impl Entry {
    pub fn parsed_timestamp(&self) -> Option<DateTime<Utc>> {
        self.timestamp
            .as_ref()
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
    }

    pub fn duration_ms(&self) -> Option<i64> {
        self.fields
            .as_ref()
            .and_then(|m| m.get("duration_ms"))
            .and_then(|v| v.as_i64())
    }
}

/// Reads structured JSONL entries from stdin into memory, aborting on the first
/// malformed JSON line so partial or corrupt inputs never produce misleading reports.
pub fn read_stdin() -> Result<Vec<Entry>, FrictionPointerError> {
    let stdin = io::stdin();
    let mut entries = Vec::new();

    // Enumerate before skipping blank lines so fallback 1-based line numbers stay aligned
    // with the original input stream.
    for (idx, line) in stdin.lock().lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let mut entry: Entry = serde_json::from_str(&line)?;
        if entry.line == 0 {
            entry.line = idx + 1;
        }
        entries.push(entry);
    }

    Ok(entries)
}
