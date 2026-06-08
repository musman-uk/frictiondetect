# Documentation

## 1. Overview

FrictionDetect analyses **structured JSONL logs** and identifies operational friction patterns such as:

- errors and warnings  
- slow operations  
- long gaps between events  
- repeated failures  

It is intentionally strict, predictable, and designed to compose cleanly with tools like LogTidy.  
FrictionDetect does **not** analyse unstructured logs or infer meaning from free text messages.  
It relies entirely on explicit structured fields such as `timestamp`, `level`, and `duration_ms`.

If these fields are missing or invalid, the corresponding friction categories cannot be detected.

FrictionDetect provides one primary mode:

- **Analyse**: read structured JSONL from file or stdin and emit a friction summary

---

## 2. Installation

FrictionDetect can be installed in three ways: downloading a binary from GitHub Releases, installing from crates.io, or building from source using Cargo.

### Install from GitHub Releases

Download the latest binary for your platform from the Releases page:

https://github.com/musman-uk/frictiondetect/releases

Make it executable if needed, then place it somewhere on your PATH. For example on Linux or macOS:

```
chmod +x frictiondetect
mv frictiondetect ~/.cargo/bin/
```

Verify installation:

```
frictiondetect help
```

### Install from crates.io

If you have Rust and Cargo installed, you can install FrictionDetect directly from crates.io:

```
cargo install frictiondetect
```

This compiles the tool from source and places the binary in `~/.cargo/bin`.

### Build from source with Cargo

If you prefer to build the tool yourself, run:

```
cargo build --release
```

Then place the binary on your PATH:

```
cp target/release/frictiondetect ~/.cargo/bin/
```

Verify installation:

```
frictiondetect help
```

### Quick Start

```
cat logs.jsonl | frictiondetect analyse
```

This reads structured JSONL input and prints a friction report.


---

## 3. Commands

| Command       | Description |
|---------------|-------------|
| `analyse`     | Analyse structured JSONL logs and produce a friction report |
| `sample`      | Show example input and output |
| `docs`        | Display condensed documentation |
| `about`       | Show tool information |
| `license`     | Display authorship and rights information |
| `help`        | Show help for any command |

---

## 4. Usage

### 4.1 Analyse Mode

Analyse structured JSONL logs:

```
frictiondetect analyse --file logs.jsonl
```

Or via stdin:

```
cat logs.jsonl | frictiondetect analyse
```

Analyse mode identifies:

- explicit `"ERROR"` and `"WARN"` events  
- operations exceeding 500ms  
- timestamp gaps exceeding 60 seconds  
- repeated identical messages (2 or more occurrences)  

---

### 4.2 Sample Output

```
frictiondetect sample
```

Shows example input and output for reference.

---

### 4.3 Documentation and About

```
frictiondetect docs
frictiondetect about
```

---

## 5. Input Format

FrictionDetect consumes **JSON Lines (JSONL)**.  
Each line must be a **valid JSON object**.  
A single malformed line aborts the entire run.

Typical fields:

- `timestamp`: RFC3339 timestamp  
- `level`: `"INFO"`, `"WARN"`, `"ERROR"`  
- `message`: log message  
- `fields`: structured metadata  
- `duration_ms`: optional numeric duration  

Example:

```json
{"line":1,"timestamp":"2024-01-01T12:00:00Z","level":"INFO","message":"Start","fields":{}}
{"line":2,"timestamp":"2024-01-01T12:00:01Z","level":"ERROR","message":"Failed","fields":{"duration_ms":800}}
{"line":3,"timestamp":"2024-01-01T12:00:02Z","level":"INFO","message":"Retry","fields":{}}
```

---

## 6. Behaviour

FrictionDetect applies strict, explicit rules.

### 6.1 JSON Parsing
- Every non empty line must be valid JSON.  
- One malformed line aborts the entire analysis.  
- Empty lines are skipped.

### 6.2 Level Detection
- Only `"ERROR"` and `"WARN"` (uppercase) are recognised.  
- `"error"`, `"Error"`, `"warning"` are ignored.  
- If `level` is missing, the event is not classified.

### 6.3 Slow Operation Detection
- Uses `fields.duration_ms` if present and numeric.  
- Slow operations are those greater than 500ms (hardcoded).  
- No inference from message text.

### 6.4 Long Gap Detection
- Requires a valid RFC3339 timestamp.  
- Gaps greater than 60 seconds are reported (hardcoded).  
- Invalid or missing timestamps disable gap detection for that entry.

### 6.5 Repeated Failure Detection
- Messages are grouped by **exact string match**.  
- Case sensitive, whitespace sensitive, punctuation sensitive.  
- Minimum occurrences: 2 (hardcoded).  
- No fuzzy matching or normalisation.

### 6.6 Execution Model
- Entire log is loaded into memory.  
- No streaming mode.  
- No partial recovery.  
- Deterministic output.

---

## 7. Known Limitations

- Requires structured JSONL; raw logs are not supported  
- One malformed JSON line aborts the run  
- No streaming mode (entire file loaded into memory)  
- Only uppercase `"ERROR"` and `"WARN"` are detected  
- Only RFC3339 timestamps are supported  
- Thresholds are hardcoded (500ms slow ops, 60s gaps, 2 occurrences)  
- No fuzzy matching for repeated failures  
- No semantic grouping (no retry or backoff detection)  
- No correlation analysis  
- No JSON output mode (stdout only)  
- Missing fields are silently ignored  


---

## 8. Troubleshooting

### Command not found
Ensure Cargo bin directory is on your PATH:

```
~/.cargo/bin
```

### No friction detected
Check that your logs include:

- valid RFC3339 timestamps  
- uppercase `"ERROR"` or `"WARN"`  
- numeric `duration_ms` fields  

### JSON parse error
A single malformed line will abort the run.  
Validate your JSONL input.

---

## 9. Summary

FrictionDetect is a focused analysis tool for structured JSONL logs and highlights operational friction using explicit fields only, without inference or heuristics. Its behaviour is deterministic, minimal, and designed to integrate cleanly with real pipelines.
