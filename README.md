# FrictionDetect

FrictionDetect is a log analysis tool that identifies friction points in structured JSONL logs. It is designed for predictable CLI behaviour, simple composition with other tools, and clean integration into pipelines.

---

## Project Contents

A short summary of what each part of the project contains:

- `src/` – Rust source files for the CLI, analysis logic, and reporting modules  
- `tests/` – integration tests and sample fixtures  
- `Documentation.md` – full usage, examples, and troubleshooting  
- `Cargo.toml` – package metadata and binary definitions (`frictiondetect` and `fd`)  
- `LICENSE` – project license  
- `README.md` – high‑level overview of the project, including purpose and structure  

---

## Development Overview

A simplified view of how the tool is developed:

| Stage     | Location       | Purpose |
|-----------|----------------|---------|
| Implement | `src/`         | Core logic, CLI behaviour, and analysis modules |
| Test      | `tests/`       | Integration tests and fixture‑based validation |
| Build     | `cargo build`  | Produces binaries (`frictiondetect`, `fd`) for release |
| Record    | `RECORD.md`    | High‑level design notes, development history, and architectural decisions |

---

## Status

Version `0.1.0` – In Development
