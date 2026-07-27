//! # kinetix_codegen
//!
//! Compiles the HIR (High-level IR) to Kinetix bytecode chunks.

pub mod compiler;
pub use compiler::{Compiler, CodegenError};
