//! Error types for the omawrite component.

/// Errors in document operations.
#[derive(Debug, thiserror::Error)]
pub enum OmawriteError {
    /// I/O error.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// Invalid document state.
    #[error("document error: {0}")]
    Document(String),
}
