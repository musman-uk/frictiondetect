# 🧨 FrictionPointer

This repository contains [FrictionPointer](https://github.com/musman-uk/frictionpointer/releases) - an analysis tool that points out friction patterns in structured JSONL logs. It is designed for predictable CLI behaviour, simple composition with other tools, and clean integration into pipelines.

---

## Project Contents

A short summary of what each part of the project contains:

- `src/` – Rust source files for the CLI, analysis logic, and reporting modules  
- `tests/` – integration tests and sample fixtures  
- `Documentation.md` – full usage, examples, and troubleshooting  
- `RECORD.md` – high‑level design notes, development history, and architectural decisions  
- `Cargo.toml` – package metadata and binary definitions (`frictionpointer` and `fp`)  
- `LICENSE` – project license (`GPL-3.0-only`)  
- `README.md` – high‑level overview of the project, including purpose and structure  

---

## Development Overview

A simplified view of how the tool is developed:

| Stage     | Location       | Purpose |
|-----------|----------------|---------|
| Implement | `src/`         | Core logic, CLI behaviour, and analysis modules |
| Test      | `tests/`       | Integration tests and fixture‑based validation |
| Build     | `cargo build`  | Produces binaries (`frictionpointer`, `fp`) for release |
| Record    | `RECORD.md`    | High‑level design notes, development history, and architectural decisions |

---

## License

Licensed under the [GNU General Public License v3.0](LICENSE) (`GPL-3.0-only`).

---

## Status

Version `0.2.0` – Published

---

<sub>© Mohammed Usman</sub>
