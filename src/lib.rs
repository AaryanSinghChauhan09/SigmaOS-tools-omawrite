//! SigmaOS tools-omawrite component.
//!
//! Rust implementation of `omacom/omawrite`: "the essence of writing" —
//! Omarchy's minimalist markdown writing tool.
//!
//! This port implements the document model the editor manipulates:
//! block-structured documents (headings, paragraphs, quotes, code,
//! lists), markdown serialization/parsing, and writing stats — the
//! logic upstream keeps in its editor core.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod document;
pub mod error;
pub mod markdown;
pub mod stats;

pub use document::{Block, BlockKind, Document};
pub use error::OmawriteError;
pub use markdown::{parse_document, render_markdown};
pub use stats::{ReadingStats, reading_stats};
