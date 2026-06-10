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

## 2. Prerequisites

Before installing FrictionDetect, there are a few requirements to ensure installation can run smoothly:

### 2.1 A terminal
FrictionDetect is a command‑line tool. You will need:

- **Windows**: PowerShell or Command Prompt  
- **macOS**: Terminal  
- **Linux**: Any shell (bash, zsh, etc.)

### 2.2 Understanding PATH
Your **PATH** is the list of folders your system searches when you type a command.

If a program is inside a folder on your PATH, you can run it from any directory:

```
frictiondetect help
```

If it is *not* on your PATH, you must run it using the full file path:

```
C:\Users\<you>\Downloads\frictiondetect.exe help
```

### 2.3 A folder on PATH
You will need a folder where you can place the FrictionDetect binary.

If you have Rust installed, the Cargo bin directory is already on PATH:

- **Windows**: `C:\Users\<you>\.cargo\bin\`
- **Linux/macOS**: `~/.cargo/bin/`

Placing the binary here makes it available globally.

### 2.4 Rust and Cargo (optional)
Rust is **not required** to run FrictionDetect, but it is useful if you want to:

- install via `cargo install frictiondetect`
- build from source
- use the `.cargo/bin` directory (already on PATH)

---

## 3. Installation

FrictionDetect can be installed in three ways: downloading a binary from GitHub Releases, installing from crates.io, or building from source using Cargo.

All installation methods assume the prerequisites above have been met.

---

### 3.1 Install from GitHub Releases

Download the latest binary from the Releases page:

https://github.com/musman-uk/frictiondetect/releases

**Note:**  
Currently only **Windows binaries** are provided.  
Linux and macOS users must install via crates.io or build from source.

---

### 3.1.1 Windows

1. Download `frictiondetect.exe` from the release.
2. Move it into your Cargo bin directory:

```
C:\Users\<you>\.cargo\bin\
```

3. Verify installation:

```
frictiondetect help
```

If you prefer not to move it, you can run it directly from the download location:

```
.\frictiondetect.exe help
```

#### Adding a custom folder to PATH (optional)

If you want to keep the binary somewhere else:

1. Open **Start Menu → Environment Variables**  
2. Click **Environment Variables…**  
3. Under **User variables**, select **Path** → **Edit**  
4. Click **New** and add the folder containing `frictiondetect.exe`  
5. Restart your terminal  

---

### 3.2 Install from crates.io

If you have Rust and Cargo installed:

```
cargo install frictiondetect
```

This compiles FrictionDetect from source and places the binary in `~/.cargo/bin`.

---

### 3.3 Build from source with Cargo

Clone the repository and build a release binary:

```
cargo build --release
```

Then move the binary to a folder on your PATH:

```
cp target/release/frictiondetect ~/.cargo/bin/
```

Verify installation:

```
frictiondetect help
```

---

### 3.4 Quick Start

Analyse structured JSONL logs:

```
cat logs.jsonl | frictiondetect analyse
```

This reads structured JSONL input and prints a friction report.

---

## 4. Commands

| Command       | Description |
|---------------|-------------|
| `analyse`     | Analyse structured JSONL logs and produce a friction report |
| `sample`      | Show example input and output |
| `docs`        | Display condensed documentation |
| `about`       | Show tool information |
| `license`     | Display authorship and rights information |
| `help`        | Show help for any command |

---

## 5. Usage

### 5.1 Analyse Mode

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

### 5.2 Sample Output

```
frictiondetect sample
```

Shows example input and output for reference.

---

### 5.3 Documentation and About

```
frictiondetect docs
frictiondetect about
```

---

## 6. Input Format

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

## 7. Behaviour

FrictionDetect applies strict, explicit rules.

### 7.1 JSON Parsing
- Every non empty line must be valid JSON.  
- One malformed line aborts the entire analysis.  
- Empty lines are skipped.

### 7.2 Level Detection
- Only `"ERROR"` and `"WARN"` (uppercase) are recognised.  
- `"error"`, `"Error"`, `"warning"` are ignored.  
- If `level` is missing, the event is not classified.

### 7.3 Slow Operation Detection
- Uses `fields.duration_ms` if present and numeric.  
- Slow operations are those greater than 500ms (hardcoded).  
- No inference from message text.

### 7.4 Long Gap Detection
- Requires a valid RFC3339 timestamp.  
- Gaps greater than 60 seconds are reported (hardcoded).  
- Invalid or missing timestamps disable gap detection for that entry.

### 7.5 Repeated Failure Detection
- Messages are grouped by **exact string match**.  
- Case sensitive, whitespace sensitive, punctuation sensitive.  
- Minimum occurrences: 2 (hardcoded).  
- No fuzzy matching or normalisation.

### 7.6 Execution Model
- Entire log is loaded into memory.  
- No streaming mode.  
- No partial recovery.  
- Deterministic output.

---

## 8. Known Limitations

- Requires structured JSONL; raw logs are not supported  
- One malformed JSON line aborts the run  
- No streaming mode (entire file loaded into memory)  
- Only uppercase `"ERROR"` and `"WARN"` are detected  
- Only RFC3339 timestamps are supported  
- Thresholds are hardcoded (500ms slow ops, 60s gaps, 2 occurrences)  
- No fuzzy matching for repeated failures  
- No semantic grouping  
- No correlation analysis  
- No JSON output mode  
- Missing fields are silently ignored  

---

## 9. Troubleshooting

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

## 10. Summary

FrictionDetect is a focused analysis tool for structured JSONL logs and highlights operational friction using explicit fields only, without inference or heuristics. Its behaviour is deterministic, minimal, and designed to integrate cleanly with real pipelines.
