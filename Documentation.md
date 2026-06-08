# FrictionDetect Documentation

## 1. Overview

FrictionDetect analyses structured JSONL logs and identifies friction patterns such as slow operations, long gaps, repeated failures, and error clusters. It is designed for predictable CLI behaviour, clean composition with other tools, and reliable integration into pipelines. The tool focuses on taking structured log events and producing a clear, readable friction report.

### What FrictionDetect does not do

To keep FrictionDetect focused, it intentionally does **not**:

- analyse unstructured text logs  
- infer log levels from message content  
- scan for keywords such as “error”, “failed”, or “timeout”  
- guess timestamps or durations when fields are missing  
- interpret meaning from free‑text messages  
- classify friction based on wording alone  
- operate without structured JSONL input  

FrictionDetect relies on explicit fields such as `timestamp`, `level`, and `duration_ms`.  
If these fields are missing, the corresponding friction categories cannot be detected.

FrictionDetect provides a single primary mode of operation:

- Analyse mode, process structured JSONL logs from file or stdin

Two binaries are available:

- `frictiondetect`, full command  
- `fd`, short alias  

Both behave the same.

---

## 2. Installation

Build release binaries:

```
cargo build --release
```

Install them somewhere on your PATH:

```
cp target/release/frictiondetect ~/.cargo/bin/
cp target/release/fd ~/.cargo/bin/
```

Verify installation:

```
frictiondetect help
fd help
```

### Quick Start

To run a quick analysis on a JSONL log file:

```
cat logs.jsonl | frictiondetect analyse
```

Or on PowerShell:

```
Get-Content logs.jsonl | frictiondetect analyse
```

This reads structured JSONL input and emits a friction report.

---

## 3. Commands

FrictionDetect exposes a minimal, intentional command surface:

| Command       | Description |
|---------------|-------------|
| `analyse`     | Analyse structured JSONL logs and produce a friction report |
| `sample`      | Show example input and output |
| `docs`        | Display documentation reference |
| `about`       | Show tool information |
| `license`     | Display authorship and rights information |
| `help`        | Show help for any command |

---

## 4. Usage

### 4.1 Analyse Mode

Analyse mode processes structured JSONL logs and identifies friction patterns.

Example using a file:

```
frictiondetect analyse --file logs.jsonl
```

Example using stdin:

```
cat logs.jsonl | frictiondetect analyse
```

Short form:

```
cat logs.jsonl | fd analyse
```

Analyse mode is useful for:

- reviewing application behaviour  
- identifying slow or problematic operations  
- detecting repeated failures  
- summarising log health for debugging or reporting  

---

### 4.2 Sample Output

To view example input and output:

```
frictiondetect sample
```

This is useful for verifying installation or understanding the expected input format.

---

### 4.3 Documentation and About

Show documentation:

```
frictiondetect docs
```

Show tool information:

```
frictiondetect about
```

---

## 5. Input Format

FrictionDetect consumes JSON Lines (JSONL), where each line is a valid JSON object.

Typical fields include:

- `timestamp`, ISO 8601 timestamp  
- `level`, log level such as INFO, WARN, ERROR  
- `message`, log message  
- `fields`, additional structured metadata  
- `duration_ms`, optional duration field  

Example:

```json
{"line":1,"timestamp":"2024-01-01T12:00:00Z","level":"INFO","message":"Start","fields":{}}
{"line":2,"timestamp":"2024-01-01T12:00:01Z","level":"ERROR","message":"Failed","fields":{"duration_ms":800}}
{"line":3,"timestamp":"2024-01-01T12:00:02Z","level":"INFO","message":"Retry","fields":{}}
```

---

## 6. Output Format

FrictionDetect emits a human readable friction report summarising:

- errors and warnings  
- slow operations  
- long gaps between events  
- repeated failures  
- counts and aggregates  

The report is deterministic and stable across runs.

---

## 7. Platform Notes

### Windows PowerShell

Use Get-Content for consistent streaming:

```
Get-Content logs.jsonl | frictiondetect analyse
```

### Linux and macOS

Standard piping works as expected:

```
cat logs.jsonl | frictiondetect analyse
```

---

## 8. Troubleshooting

### FrictionDetect is not recognised as a command

Ensure Cargo’s bin directory is on your PATH:

```
~/.cargo/bin
```

Restart your terminal after installation.

### No output appears

Test with a simple inline example:

```
{"timestamp":"2024-01-01T12:00:00Z","level":"INFO","message":"Test"} | frictiondetect analyse
```

If this works, the issue is with the upstream command.

### File not found

Quote paths that contain spaces:

```
frictiondetect analyse --file "C:/logs/app logs.jsonl"
```

---

## 9. Summary

FrictionDetect is a focused analysis tool. It reads structured JSONL logs and produces a clear friction report that highlights operational issues without interpreting meaning beyond the data provided. It is predictable, pipeline friendly, and designed to integrate cleanly with tools like LogTidy. It aims to provide insights that are both clear and useful within real workflows and pipelines.
