use thiserror::Error;

/// Domain-specific errors returned during JSONL ingestion and parsing.
/// Kept distinct so I/O failures and malformed JSON lines surface clear context.
#[derive(Debug, Error)]
pub enum FrictionPointerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}
