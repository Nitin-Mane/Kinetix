//! Statement and item AST nodes for Kinetix.

use smol_str::SmolStr;
use kinetix_lexer::Span;
use crate::{Expr, TyAnnotation, types::GenericParam, expr::{Block, Pattern}};

// ── Statements ────────────────────────────────────────────────────────────

/// A statement — can appear inside a block.
#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    /// `let [mut] name[: type] = expr`
    Let {
        mutable: bool,
        pattern: Pattern,
        ty:      Option<TyAnnotation>,
        init:    Option<Box<Expr>>,
    },
    /// `const NAME: type = expr`
    Const {
        name: SmolStr,
        ty:   TyAnnotation,
        init: Box<Expr>,
    },
    /// An expression used as a statement (typically with side effects).
    Expr(Box<Expr>),
    /// A top-level item appearing inside a block (nested fn, struct, etc.)
    Item(Box<Item>),
    /// `;` — empty statement
    Empty,
}

// ── Top-level Items ───────────────────────────────────────────────────────

/// A top-level item in a Kinetix source file or module.
#[derive(Debug, Clone)]
pub struct Item {
    pub kind: ItemKind,
    pub span: Span,
    /// Documentation comment attached to this item.
    pub doc:  Option<String>,
}

#[derive(Debug, Clone)]
pub enum ItemKind {
    /// `fn name<Generics>(params) -> RetType { body }`
    Fn(FnDef),
    /// `struct Name<Generics> { fields }`
    Struct(StructDef),
    /// `enum Name<Generics> { variants }`
    Enum(EnumDef),
    /// `trait Name<Generics>[:SuperTraits] { items }`
    Trait(TraitDef),
    /// `impl [Trait for] Type<Generics> { items }`
    Impl(ImplBlock),
    /// `type Alias<Generics> = Type`
    TypeAlias { name: SmolStr, generics: Vec<GenericParam>, ty: TyAnnotation },
    /// `mod name { items }`
    Module { name: SmolStr, items: Vec<Item> },
    /// `import path::to::item` or `import path::*`
    Import(ImportPath),
    /// `export { names }`
    Export(Vec<SmolStr>),
    /// `const NAME: type = expr`
    Const { name: SmolStr, ty: TyAnnotation, init: Box<Expr> },
}

// ── Function ──────────────────────────────────────────────────────────────

/// A function definition (includes the signature and body).
#[derive(Debug, Clone)]
pub struct FnDef {
    pub sig:   FnSig,
    pub body:  Option<Block>, // None for trait method stubs
}

/// Function signature (shareable between FnDef and trait method declarations).
#[derive(Debug, Clone)]
pub struct FnSig {
    pub name:        SmolStr,
    pub generics:    Vec<GenericParam>,
    pub params:      Vec<Param>,
    pub return_type: Option<TyAnnotation>,
    pub is_async:    bool,
    pub is_pub:      bool,
}

/// A function parameter.
#[derive(Debug, Clone)]
pub struct Param {
    pub name:    SmolStr,
    pub ty:      TyAnnotation,
    /// `self` parameter (first param only).
    pub is_self: bool,
    /// `mut self`.
    pub is_mut:  bool,
}

// ── Struct ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct StructDef {
    pub name:     SmolStr,
    pub generics: Vec<GenericParam>,
    pub fields:   Vec<FieldDef>,
    pub is_pub:   bool,
}

#[derive(Debug, Clone)]
pub struct FieldDef {
    pub name:   SmolStr,
    pub ty:     TyAnnotation,
    pub is_pub: bool,
    pub span:   Span,
}

// ── Enum ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct EnumDef {
    pub name:     SmolStr,
    pub generics: Vec<GenericParam>,
    pub variants: Vec<EnumVariant>,
    pub is_pub:   bool,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name:   SmolStr,
    pub fields: EnumVariantFields,
    pub span:   Span,
}

#[derive(Debug, Clone)]
pub enum EnumVariantFields {
    /// `Variant`
    Unit,
    /// `Variant(T, U)`
    Tuple(Vec<TyAnnotation>),
    /// `Variant { x: T, y: U }`
    Struct(Vec<FieldDef>),
}

// ── Trait ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TraitDef {
    pub name:        SmolStr,
    pub generics:    Vec<GenericParam>,
    pub super_traits: Vec<TyAnnotation>,
    pub items:       Vec<TraitItem>,
    pub is_pub:      bool,
}

#[derive(Debug, Clone)]
pub enum TraitItem {
    Method(FnDef),
    Const { name: SmolStr, ty: TyAnnotation, default: Option<Box<Expr>> },
    Type  { name: SmolStr, bounds: Vec<TyAnnotation> },
}

// ── Impl ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ImplBlock {
    pub generics:    Vec<GenericParam>,
    /// `Some(trait)` for `impl Trait for Type`, `None` for inherent impl.
    pub trait_ty:    Option<TyAnnotation>,
    /// The type being implemented.
    pub self_ty:     TyAnnotation,
    pub items:       Vec<FnDef>,
}

// ── Import ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ImportPath {
    pub segments: Vec<SmolStr>,
    pub kind:     ImportKind,
}

#[derive(Debug, Clone)]
pub enum ImportKind {
    /// `import a::b::c`
    Simple,
    /// `import a::b::*`
    Glob,
    /// `import a::b::{c, d}`
    Group(Vec<SmolStr>),
    /// `import a::b as alias`
    Alias(SmolStr),
}
