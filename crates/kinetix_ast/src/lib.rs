//! # kinetix_ast
//!
//! Abstract Syntax Tree (AST) node definitions for the Kinetix language.
//!
//! The AST closely mirrors the surface syntax of Kinetix.  After parsing,
//! the tree is lowered to the HIR (High-level IR) for type-checking and
//! further compilation.

pub mod expr;
pub mod stmt;
pub mod types;

pub use expr::{Expr, ExprKind, BinOp, UnOp, Lit};
pub use stmt::{Stmt, StmtKind, Item, ItemKind, FnSig, Param};
pub use types::{TyAnnotation, TyKind};
use kinetix_lexer::Span;

/// A node in the AST — pairs any AST kind with its source span.
#[derive(Debug, Clone)]
pub struct Node<T> {
    pub inner: T,
    pub span:  Span,
}

impl<T> Node<T> {
    pub fn new(inner: T, span: Span) -> Self {
        Self { inner, span }
    }
}

/// The top-level compilation unit.
#[derive(Debug, Clone)]
pub struct SourceFile {
    /// All top-level items (fn, struct, enum, impl, import, …)
    pub items: Vec<Node<Item>>,
    /// Source file path (for diagnostics)
    pub path: Option<std::path::PathBuf>,
}
