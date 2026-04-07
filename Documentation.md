# Documentation

## 1. Overview

FrictionDetect analyses structured JSONL logs and identifies friction patterns such as slow operations, long gaps, repeated failures, and error clusters. It is designed for predictable CLI behaviour, simple composition with other tools, and clean integration into pipelines. This document provides full usage guidance, examples, and troubleshooting notes.

FrictionDetect provides a single primary mode of operation:

- **Analyse mode** — processes structured JSONL logs (from file or stdin)

The tool emits a human‑readable friction report summarising issues detected in the log stream.

Two binaries are available:

- `frictiondetect` — full command name  
- `fd` — short alias for daily use  

Both map to the same executable logic.

---

## 2. Installation

Build the release binaries:

```
cargo build --release
```

Install them into a directory on your `PATH` (for example, Cargo’s bin directory):

```
cp target/release/frictiondetect ~/.cargo/bin/
cp target/release/fd ~/.cargo/bin/
```

Verify installation:

```
frictiondetect
fd
```

---

## 3. Commands

FrictionDetect exposes a minimal, intentional command surface:

| Command       | Description |
|---------------|-------------|
| `analyse`     | Analyse structured JSONL logs and produce a friction report |
| `sample`      | Display example input and output |
| `docs`        | Show documentation reference |
| `about`       | Display tool information |
| `license`     | Display authorship and rights information |
| `help`        | Show help for any command |

---

## 4. Usage

### 4.1. Analyse Mode

Analyse mode processes structured JSONL logs and identifies friction patterns.

Example (file input):

```
frictiondetect analyse --file logs.jsonl
```

Example (stdin):

```
cat logs.jsonl | frictiondetect analyse
```

Short form:

```
cat logs.jsonl | fd analyse
```

Analyse mode is ideal for:

- reviewing application behaviour  
- identifying slow or problematic operations  
- detecting repeated failures  
- summarising log health for debugging or reporting  

### 4.2. Sample Output

To view example input and output:

```
frictiondetect sample
```

This is useful for verifying installation or understanding the expected input format.

### 4.3. Documentation and About

Show documentation reference:

```
frictiondetect docs
```

Show tool information:

```
frictiondetect about
```

---

## 5. Input Format

FrictionDetect consumes **JSON Lines (JSONL)**, where each line is a valid JSON object.

Typical fields include:

- `timestamp` — ISO‑8601 timestamp  
- `level` — log level (INFO, WARN, ERROR, etc.)  
- `message` — log message  
- `fields` — additional structured metadata  
- `duration_ms` — operation duration (optional)  

Example:

```json
{"line":1,"timestamp":"2024-01-01T12:00:00Z","level":"INFO","message":"Start","fields":{}}
{"line":2,"timestamp":"2024-01-01T12:00:01Z","level":"ERROR","message":"Failed","fields":{"duration_ms":800}}
{"line":3,"timestamp":"2024-01-01T12:00:02Z","level":"INFO","message":"Retry","fields":{}}
```

---

## 6. Output Format

FrictionDetect emits a human‑readable friction report summarising:

- errors and warnings  
- slow operations  
- long gaps between events  
- repeated failures  
- counts and aggregates  

The report is deterministic and stable across runs.

---

## 7. Development Notes

FrictionDetect is implemented in Rust using:

- **Clap** for CLI parsing  
- **Serde** for JSON parsing  
- **Chrono** for timestamp handling  
- **ThisError** for error definitions  

Integration tests use:

- `assert_cmd`  
- `predicates`  

Fixtures live under `tests/fixtures/`.

---

## 8. Troubleshooting

### 8.1. FrictionDetect is not recognised as a command

Ensure Cargo’s bin directory is on your `PATH`:

```
~/.cargo/bin
```

Restart your terminal after installation.

### 8.2. No output appears

Confirm that input is being piped correctly:

```
cat logs.jsonl | frictiondetect analyse
```

If this works, the issue is with the upstream command.

### 8.3. File not found

Ensure the path is correct and quoted if it contains spaces:

```
frictiondetect analyse --file "C:/logs/app logs.jsonl"
```

---

## 9. Versioning

Current version: `0.1.0` — In Development

