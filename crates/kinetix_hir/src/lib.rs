//! # kinetix_hir
//!
//! High-level Intermediate Representation (HIR).
//!
//! The HIR is produced from the AST after type checking.  It is similar
//! to the AST but with all types resolved, all sugar de-sugared, and
//! implicit constructs made explicit.
//!
//! ## Key differences from the AST
//! - Every expression has an explicit, resolved [`Ty`].
//! - String interpolation is lowered to concatenation calls.
//! - `elif` chains are lowered to nested `if/else`.
//! - `for` loops are lowered to `loop { let item = iter.next(); if item == nil { break } ... }`.
//! - Operator overloads are resolved to explicit method calls.

use smol_str::SmolStr;
use kinetix_types::Ty;

pub mod expr;
pub mod item;
pub mod lower;

pub use expr::{HirExpr, HirExprKind};
pub use item::{HirItem, HirFn, HirParam};

/// A unique identifier for any HIR node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HirId(pub u32);

/// The HIR representation of a source file.
pub struct HirFile {
    pub items: Vec<HirItem>,
    pub path:  Option<std::path::PathBuf>,
}
