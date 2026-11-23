# RECORD

## 1. Design Principles

### Tool Identity
- Command‑line utility for analysing structured JSONL logs and identifying friction patterns  
- Clear, minimal command set with consistent behaviour across commands  
- Output designed for readability and downstream processing  
- No implicit heuristics; all friction detection rules are explicit and intentional  

### Interaction Model
- Commands follow a predictable pattern: heading → separator → content  
- Primary mode (`analyse`) reflects the core purpose of the tool  
- Documentation and examples remain concise and directly actionable  
- Defaults avoid side effects and require no configuration to begin using the tool  

### Output Philosophy
- JSONL used as the primary input format  
- Deterministic analysis across runs and environments  
- Errors surfaced with context to aid debugging  
- Examples demonstrate expected friction detection without unnecessary detail  

---

## 2. Development Process

### Architecture
- Modular Rust codebase with clear separation of concerns  
- CLI layer implemented via `clap` and kept intentionally thin  
- Core logic encapsulated in modules (`patterns`, `reader`, `counters`, `report`)  
- Error handling unified through `thiserror` for consistent failure modes  

### Command Structure
- `analyse` – processes structured JSONL logs and produces a friction summary  
- `sample` – provides minimal example input and output  
- `docs` – summarises usage, inputs, outputs, and examples  
- `about` – displays tool name, version, and repository source  
- `license` – displays authorship and rights information  

### Source Layout
| File / Module       | Purpose |
|---------------------|---------|
| `src/main.rs`       | CLI entry point; command dispatch for all modes. |
| `src/patterns.rs`   | Core friction detection logic (slow ops, gaps, errors, repeats). |
| `src/reader.rs`     | JSONL reader for file and stdin input. |
| `src/counters.rs`   | Aggregation helpers for friction metrics. |
| `src/report.rs`     | Human‑readable output formatting. |
| `src/errors.rs`     | Error types and error handling used across the tool. |

### Versioning
- Version sourced from `Cargo.toml` via `env!("CARGO_PKG_VERSION")`  
- Current stage: **v0.1.0 (draft / pre‑release)**  
- Versioning will stabilise once friction rules and thresholds mature  

---

## 3. Milestones

### Initial Structure
- Repository initialised with modular Rust layout  
- Core friction detection logic drafted  
- JSONL reader implemented to validate architecture  
- Early examples created to test output consistency  

### CLI & Behaviour
- Command definitions stabilised  
- Help, about, and docs commands implemented  
- License command added for authorship and rights declaration  
- Default command output aligned with overall tool behaviour  

### Documentation & Workflow
- README drafted to outline purpose and usage  
- RECORD.md introduced to track design and development decisions  
- Pre‑release version tagged as v0.1.0  
- Repository prepared for iterative refinement and future expansion  

---

## 4. Version History

### v0.1.0 – Published
- Implemented friction analysis pipeline (errors, slow ops, gaps, repeats)  
- Added example and documentation commands  
- Introduced identity commands (`about`, `license`)  
- Established consistent CLI output structure  
- Finalised initial analysis and reporting pipeline  
- Prepared repository for future refinement and public release  

