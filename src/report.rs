use crate::patterns::{FrictionSummary, LongGap, RepeatedFailureGroup};

pub fn print(summary: &FrictionSummary) {
    println!("FrictionDetect Report");
    println!("=====================\n");

    println!("Total:");
    println!("  Errors: {}", summary.errors.len());
    println!("  Warnings: {}", summary.warnings.len());
    println!("  Slow operations: {}", summary.slow_ops.len());
    println!("  Long gaps: {}", summary.long_gaps.len());
    println!("  Repeated failures: {}", summary.repeated_failures.len());
    println!();

    println!("Further Detail:");
    println!("---------------\n");

    print_errors(summary);
    print_warnings(summary);
    print_slow_ops(summary);
    print_long_gaps(summary);
    print_repeated_failures(summary);
}

fn print_errors(summary: &FrictionSummary) {
    println!("Errors:");
    if summary.errors.is_empty() {
        println!("  (none)\n");
        return;
    }
    for e in &summary.errors {
        let ts = e.timestamp.as_deref().unwrap_or("-");
        println!("  [line {}]  {}  {}", e.line, ts, e.detail);
    }
    println!();
}

fn print_warnings(summary: &FrictionSummary) {
    println!("Warnings:");
    if summary.warnings.is_empty() {
        println!("  (none)\n");
        return;
    }
    for w in &summary.warnings {
        let ts = w.timestamp.as_deref().unwrap_or("-");
        println!("  [line {}]  {}  {}", w.line, ts, w.detail);
    }
    println!();
}

fn print_slow_ops(summary: &FrictionSummary) {
    println!("Slow Operations:");
    if summary.slow_ops.is_empty() {
        println!("  (none)\n");
        return;
    }
    for s in &summary.slow_ops {
        let ts = s.timestamp.as_deref().unwrap_or("-");
        // detail is "<ms> ms"
        println!("  [line {}]   [{}]   {}   {}", s.line, s.detail, ts, slow_message_for_line(s.line, summary));
    }
    println!();
}

fn slow_message_for_line(line: usize, summary: &FrictionSummary) -> String {
    // Best-effort: find an error/warn/info message with same line in errors/warnings
    if let Some(e) = summary.errors.iter().find(|e| e.line == line) {
        return e.detail.clone();
    }
    if let Some(w) = summary.warnings.iter().find(|w| w.line == line) {
        return w.detail.clone();
    }
    String::new()
}

fn print_long_gaps(summary: &FrictionSummary) {
    println!("Long Gaps:");
    if summary.long_gaps.is_empty() {
        println!("  (none)\n");
        return;
    }
    for LongGap {
        from_line,
        to_line,
        from_ts,
        to_ts,
        gap_seconds,
    } in &summary.long_gaps
    {
        println!(
            "  [line {} → line {}]  {} → {} ({} seconds)",
            from_line, to_line, from_ts, to_ts, gap_seconds
        );
    }
    println!();
}

fn print_repeated_failures(summary: &FrictionSummary) {
    println!("Repeated Failures:");
    if summary.repeated_failures.is_empty() {
        println!("  (none)\n");
        return;
    }
    for RepeatedFailureGroup { message, occurrences } in &summary.repeated_failures {
        println!("  {} ({} occurrences)", message, occurrences.len());
        for o in occurrences {
            let ts = o.timestamp.as_deref().unwrap_or("-");
            println!("    - [line {}]  {}", o.line, ts);
        }
        println!();
    }
}
