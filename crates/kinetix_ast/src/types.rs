//! Type annotation AST nodes for Kinetix.

use smol_str::SmolStr;

/// A type annotation as written in source — e.g. `int`, `vec<float>`,
/// `fn(int) -> bool`, `?string`, `matrix<float>`.
#[derive(Debug, Clone)]
pub struct TyAnnotation {
    pub kind: TyKind,
}

impl TyAnnotation {
    pub fn new(kind: TyKind) -> Self { Self { kind } }
}

/// All syntactic type forms in Kinetix.
#[derive(Debug, Clone)]
pub enum TyKind {
    // ── Primitives ────────────────────────────────────────────────────────
    Int,
    Float,
    Bool,
    Char,
    String,
    Nil,

    // ── Named type (possibly generic) ─────────────────────────────────────
    /// `Foo` or `Foo<T, U>`
    Named {
        name:     SmolStr,
        generics: Vec<TyAnnotation>,
    },
    /// Qualified path: `std::vec::Vec<T>`
    Path {
        segments: Vec<SmolStr>,
        generics: Vec<TyAnnotation>,
    },

    // ── Built-in generic types ────────────────────────────────────────────
    /// `vec<T>`
    Vec(Box<TyAnnotation>),
    /// `map<K, V>`
    Map(Box<TyAnnotation>, Box<TyAnnotation>),
    /// `matrix<T>`
    Matrix(Box<TyAnnotation>),
    /// `[T; N]` — fixed-size array
    Array(Box<TyAnnotation>, usize),

    // ── Function type ─────────────────────────────────────────────────────
    /// `fn(T, U) -> R`
    Fn {
        params:  Vec<TyAnnotation>,
        ret:     Box<TyAnnotation>,
        is_async: bool,
    },

    // ── Tuple ─────────────────────────────────────────────────────────────
    /// `(T, U, V)`
    Tuple(Vec<TyAnnotation>),

    // ── Optional ──────────────────────────────────────────────────────────
    /// `?T`
    Optional(Box<TyAnnotation>),

    // ── Reference / pointer ───────────────────────────────────────────────
    /// `&T` — shared reference
    Ref(Box<TyAnnotation>),
    /// `&mut T` — mutable reference
    RefMut(Box<TyAnnotation>),
    /// `box<T>` — heap box
    Box(Box<TyAnnotation>),
    /// `raw<T>` — raw pointer (unsafe)
    Raw(Box<TyAnnotation>),

    // ── Dynamic dispatch ──────────────────────────────────────────────────
    /// `dyn Trait`
    Dyn(Box<TyAnnotation>),

    // ── Inferred ──────────────────────────────────────────────────────────
    /// `_` — let the compiler infer
    Infer,

    // ── Self ──────────────────────────────────────────────────────────────
    SelfTy,

    // ── Never ─────────────────────────────────────────────────────────────
    /// `!` — diverging type (never returns)
    Never,
}

// ── Generic parameter definition ─────────────────────────────────────────

/// A generic type/const parameter, e.g. `T: Ord + Copy` or `N: int`.
#[derive(Debug, Clone)]
pub struct GenericParam {
    pub name:   SmolStr,
    pub bounds: Vec<TyAnnotation>,
    pub kind:   GenericParamKind,
}

#[derive(Debug, Clone)]
pub enum GenericParamKind {
    /// Normal type parameter `T`
    Type,
    /// Const parameter `N: int`
    Const(TyAnnotation),
    /// Lifetime `'a` (future)
    Lifetime,
}
