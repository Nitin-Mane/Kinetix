//! # kinetix_types
//!
//! Type system, type inference engine, and type checker for the Kinetix language.
//!
//! ## Architecture
//!
//! 1. **[`Ty`]** — the concrete type representation used throughout the compiler.
//! 2. **[`TypeEnv`]** — the typing environment (variable → type mappings).
//! 3. **[`TypeChecker`]** — walks the AST and resolves all types.
//! 4. **[`Inference`]** — Hindley-Milner style unification for type variables.

pub mod ty;
pub mod checker;
pub mod inference;

pub use ty::{Ty, TyId, PrimTy, FnTy, TyKind, TyVar};
pub use checker::{TypeChecker, TypeEnv, CheckError};
pub use inference::InferCtx;

