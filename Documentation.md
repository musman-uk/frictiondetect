# Documentation

## 1. Overview

FrictionPointer analyses **structured JSONL logs** and points out operational
friction patterns such as:

- errors and warnings  
- slow operations  
- long gaps between events  
- repeated failures  

It is intentionally strict, predictable, and designed to work standalone on
structured JSONL logs or compose cleanly with tools like LogTidier.  
FrictionPointer does **not** analyse unstructured raw logs or infer meaning from
free text messages.  
It relies entirely on explicit structured fields such as `timestamp`, `level`,
`message`, and `fields.duration_ms`.

If these fields are missing or invalid, the corresponding friction categories
cannot be detected.

FrictionPointer provides one primary mode:

- **Analyse**: read structured JSONL from file or stdin and emit a friction
  summary

---

## 2. Prerequisites

Before installing FrictionPointer, ensure the following requirements are met:

### 2.1 A terminal
FrictionPointer is a command‑line tool. You will need:

- **Windows**: PowerShell or Command Prompt (including VS Code's integrated
  terminal)  
- **macOS**: Terminal  
- **Linux**: Any shell (bash, zsh, etc.)

### 2.2 Understanding PATH
Your **PATH** is the list of folders your system searches when you type a
command.

If a program is inside a folder on your PATH, you can run it from any directory:

```
frictionpointer help
```

If it is *not* on your PATH, you must run it using the full file path:

```
C:\Users\<you>\Downloads\frictionpointer.exe help
```

### 2.3 A folder on PATH
You will need a folder where you can place the FrictionPointer binaries
(`frictionpointer` and `fp`).

If you have Rust installed, the Cargo bin directory is already on PATH:

- **Windows**: `C:\Users\<you>\.cargo\bin\`
- **Linux/macOS**: `~/.cargo/bin/`

Placing the binaries here makes them available globally.

### 2.4 Rust and Cargo (optional on Windows; required for source builds)
Rust is **not required** if you download a prebuilt binary from GitHub Releases,
but it is required if you want to:

- build or install from source (`cargo build --release` or
  `cargo install --path .`)
- use the `.cargo/bin` directory (already on PATH)

---

## 3. Installation

FrictionPointer can currently be installed by downloading a prebuilt binary from
**GitHub Releases** or building from source with **Cargo**. Publishing to
`crates.io` (`cargo install frictionpointer`) is planned for a future release.

---

### 3.1 Install from GitHub Releases

Download the latest release from:

https://github.com/musman-uk/frictionpointer/releases

**Note:**  
Currently only **Windows binaries** (`frictionpointer.exe` and `fp.exe`) are
provided on GitHub Releases.  
Linux and macOS users should build from source using Cargo (Section 3.2).

---

### 3.1.1 Windows

1. Download `frictionpointer.exe` (and optionally the `fp.exe` shorthand binary)
   from the release.
2. Move the binary into your Cargo bin directory (or another folder on your
   PATH):

```
C:\Users\<you>\.cargo\bin\
```

3. Verify installation:

```
frictionpointer help
```

If you prefer not to move it, you can run it directly from the download
location:

```
.\frictionpointer.exe help
```

#### Adding a custom folder to PATH (optional)

If you want to keep the binary somewhere else:

1. Open **Start Menu → Environment Variables**  
2. Click **Environment Variables…**  
3. Under **User variables**, select **Path** → **Edit**  
4. Click **New** and add the folder containing `frictionpointer.exe`  
5. Restart your terminal  

**Note:**  
Double-clicking the `.exe` file may briefly open and close a terminal window
immediately. As prebuilt binaries are not digitally signed, your browser or
Windows may show a prompt – choose **Keep** when downloading, and click
**More info → Run anyway** if Windows prompts you when running it.
Alternatively, you can build from source using Cargo (Section 3.2).

---

### 3.2 Build from source with Cargo

Clone the repository and build release binaries:

```
git clone [https://github.com/musman-uk/frictionpointer.git](https://github.com/musman-uk/frictionpointer.git)
cd frictionpointer
cargo build --release
```

Copy both binaries (`frictionpointer` and `fp`) to a folder on your PATH:

**Linux / macOS:**
```
cp target/release/frictionpointer target/release/fp ~/.cargo/bin/
```

**Windows (PowerShell):**
```
Copy-Item .\target\release\frictionpointer.exe, .\target\release\fp.exe $env:USERPROFILE\.cargo\bin\
```

Alternatively, install directly from the cloned directory with Cargo:

```
cargo install --path .
```

Verify installation:

```
frictionpointer help
```

---

### 3.3 Installing via crates.io (Planned)

Support for installing directly from `crates.io` (`cargo install frictionpointer`)
is planned and not yet live. Until published, use GitHub Releases or build from
source.

---

### 3.4 Quick Start

Analyse structured JSONL logs:

```
cat logs.jsonl | frictionpointer analyse
```

PowerShell:

```
Get-Content logs.jsonl | frictionpointer analyse
```

Piped directly from LogTidier:

```
Get-Content app.log | logtidier stream | frictionpointer analyse
```

This reads structured JSONL input and prints a friction report.

---

## 4. Commands

| Command   | Description |
|-----------|-------------|
| `analyse` | Analyse structured JSONL logs and produce a friction report |
| `sample`  | Show example input and output |
| `docs`    | Display condensed documentation reference |
| `about`   | Show tool name, version, license, source, and copyright notice |
| `help`    | Show help for any command |

---

## 5. Usage

### 5.1 Analyse Mode

Analyse structured JSONL logs from a file:

```
frictionpointer analyse --file logs.jsonl
```

Short flag and binary form:

```
fp analyse -f logs.jsonl
```

Or via stdin:

```
Get-Content logs.jsonl | frictionpointer analyse
```

Analyse mode identifies:

- explicit `"ERROR"` and `"WARN"` events  
- operations exceeding 500ms  
- timestamp gaps exceeding 60 seconds  
- repeated identical messages (2 or more occurrences)  

---

### 5.2 Sample Output

```
frictionpointer sample
```

Shows example input and output for reference.

---

### 5.3 Documentation and About

Show condensed documentation:

```
frictionpointer docs
```

Show tool information, version, and license:

```
frictionpointer about
```

---

## 6. Input Format

FrictionPointer consumes **JSON Lines (JSONL)**.  
Each non-empty line must be a **valid JSON object**.  
A single malformed line aborts the entire run.

Supported fields:

- `line`: optional 1-based line number (defaults to the input line number if
  omitted)  
- `timestamp`: optional RFC3339 timestamp  
- `level`: optional log level (`"ERROR"` and `"WARN"` are reported)  
- `message`: optional log message  
- `fields`: optional JSON object containing structured metadata (such as
  numeric `duration_ms`)  

Example:

```json
{"line":1,"timestamp":"2024-01-01T12:00:00Z","level":"INFO","message":"Start","fields":{}}
{"line":2,"timestamp":"2024-01-01T12:00:01Z","level":"ERROR","message":"Failed","fields":{"duration_ms":800}}
{"line":3,"timestamp":"2024-01-01T12:00:02Z","level":"INFO","message":"Retry","fields":{}}
```

---

## 7. Behaviour

FrictionPointer applies strict, explicit rules.

### 7.1 JSON Parsing
- Every non-empty line must be valid JSON.  
- One malformed line aborts the entire analysis.  
- Empty or whitespace-only lines are skipped.

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
- Gaps greater than 60 seconds between consecutive timestamped entries are
  reported (hardcoded).  
- Invalid or missing timestamps disable gap detection for that entry.

### 7.5 Repeated Failure Detection
- Non-empty `message` values are grouped by **exact string match**.  
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

- Requires structured JSONL; raw unstructured logs are not supported  
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

- **Windows**: `C:\Users\<you>\.cargo\bin\`
- **Linux/macOS**: `~/.cargo/bin/`

Restart your terminal after updating PATH.

### No friction detected
Check that your logs include:

- valid RFC3339 timestamps  
- uppercase `"ERROR"` or `"WARN"`  
- numeric `fields.duration_ms` values  

### JSON parse error
A single malformed line will abort the run.  
Validate your JSONL input.

---

## 10. Summary

FrictionPointer is a focused analysis tool for structured JSONL logs and
points out operational friction using explicit fields only, without inference or
heuristics. Its behaviour is deterministic, minimal, and designed to integrate
cleanly with real pipelines.
