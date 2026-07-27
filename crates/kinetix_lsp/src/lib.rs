//! # kinetix_lsp
//!
//! Language Server Protocol (LSP) implementation for Kinetix.
//!
//! Provides IDE features to any LSP-compatible editor:
//!
//! | Feature                  | Status  |
//! |--------------------------|---------|
//! | Diagnostics              | Phase 1 |
//! | Syntax highlighting      | Phase 1 (via `kinetix_syntax`) |
//! | Hover (type info)        | Phase 1 |
//! | Go-to definition         | Phase 1 |
//! | Auto-completion          | Phase 1 |
//! | Code formatting          | Phase 1 |
//! | Rename                   | Phase 2 |
//! | Find references          | Phase 2 |
//! | Inlay hints              | Phase 2 |

pub mod server;
pub mod document;
pub mod diagnostics;

pub use server::LspServer;
pub use document::Document;
