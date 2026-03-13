use crate::errors::FrictionError;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{Map, Value};
use std::io::{self, BufRead};

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub line: usize,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(default)]
    pub level: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub fields: Option<Map<String, Value>>,
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

pub fn read_stdin() -> Result<Vec<Entry>, FrictionError> {
    let stdin = io::stdin();
    let mut entries = Vec::new();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let entry: Entry = serde_json::from_str(&line)?;
        entries.push(entry);
    }

    Ok(entries)
}
