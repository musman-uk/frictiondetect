use crate::counters::MessageCounts;
use crate::reader::Entry;
use chrono::{DateTime, Utc};

const SLOW_THRESHOLD_MS: i64 = 500;
const GAP_THRESHOLD_SECONDS: i64 = 60;
const REPEATED_MIN_OCCURRENCES: usize = 2;

#[derive(Debug)]
pub struct FrictionPoint {
    pub line: usize,
    pub timestamp: Option<String>,
    pub detail: String,
}

#[derive(Debug)]
pub struct SlowOp {
    pub line: usize,
    pub timestamp: Option<String>,
    pub duration_ms: i64,
    pub message: String,
}

#[derive(Debug)]
pub struct LongGap {
    pub from_line: usize,
    pub to_line: usize,
    pub from_ts: String,
    pub to_ts: String,
    pub gap_seconds: i64,
}

#[derive(Debug)]
pub struct RepeatedFailureGroup {
    pub message: String,
    pub occurrences: Vec<FrictionPoint>,
}

#[derive(Debug, Default)]
pub struct FrictionSummary {
    pub total_lines: usize,

    pub errors: Vec<FrictionPoint>,
    pub warnings: Vec<FrictionPoint>,
    pub slow_ops: Vec<SlowOp>,
    pub long_gaps: Vec<LongGap>,
    pub repeated_failures: Vec<RepeatedFailureGroup>,
}

pub fn analyse(entries: &[Entry]) -> FrictionSummary {
    let mut summary = FrictionSummary {
        total_lines: entries.len(),
        ..Default::default()
    };

    let mut prev_ts: Option<(DateTime<Utc>, usize)> = None;
    let mut msg_counts = MessageCounts::default();

    for e in entries {
        // Levels
        if let Some(level) = &e.level {
            let point = FrictionPoint {
                line: e.line,
                timestamp: e.timestamp.clone(),
                detail: e.message.clone().unwrap_or_default(),
            };

            match level.as_str() {
                "ERROR" => summary.errors.push(point),
                "WARN" => summary.warnings.push(point),
                _ => {}
            }
        }

        // Slow operations
        if let Some(ms) = e.duration_ms() {
            if ms > SLOW_THRESHOLD_MS {
                summary.slow_ops.push(SlowOp {
                    line: e.line,
                    timestamp: e.timestamp.clone(),
                    duration_ms: ms,
                    message: e.message.clone().unwrap_or_default(),
                });
            }
        }

        // Long gaps
        if let Some(ts) = e.parsed_timestamp() {
            if let Some((prev, prev_line)) = prev_ts {
                let gap = (ts - prev).num_seconds();
                if gap > GAP_THRESHOLD_SECONDS {
                    let from_ts = entries
                        .iter()
                        .find(|x| x.line == prev_line)
                        .and_then(|x| x.timestamp.clone())
                        .unwrap_or_else(|| prev.to_rfc3339());
                    let to_ts = e.timestamp.clone().unwrap_or_else(|| ts.to_rfc3339());

                    summary.long_gaps.push(LongGap {
                        from_line: prev_line,
                        to_line: e.line,
                        from_ts,
                        to_ts,
                        gap_seconds: gap,
                    });
                }
            }
            prev_ts = Some((ts, e.line));
        }

        // Repeated messages
        msg_counts.add(e);
    }

    // Build repeated failure groups
    for (msg, lines) in msg_counts.repeated(REPEATED_MIN_OCCURRENCES) {
        let mut occurrences = Vec::new();
        for line in lines {
            if let Some(entry) = entries.iter().find(|e| e.line == line) {
                occurrences.push(FrictionPoint {
                    line,
                    timestamp: entry.timestamp.clone(),
                    detail: String::new(),
                });
            }
        }
        summary.repeated_failures.push(RepeatedFailureGroup {
            message: msg,
            occurrences,
        });
    }

    summary
}