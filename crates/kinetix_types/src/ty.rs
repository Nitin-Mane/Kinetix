//! Concrete type representation for the Kinetix type system.

use smol_str::SmolStr;

/// A unique identifier for a type in the type table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TyId(pub u32);

/// All concrete types in Kinetix (after type inference).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    // ── Primitives ────────────────────────────────────────────────────────
    Prim(PrimTy),

    // ── Compound ──────────────────────────────────────────────────────────
    /// `vec<T>`
    Vec(Box<Ty>),
    /// `map<K, V>`
    Map(Box<Ty>, Box<Ty>),
    /// `matrix<T>` — 2D numeric array
    Matrix(Box<Ty>),
    /// `[T; N]` — fixed-size array
    Array(Box<Ty>, usize),
    /// Tuple `(T1, T2, ...)`
    Tuple(Vec<Ty>),
    /// Optional `?T`
    Optional(Box<Ty>),

    // ── Structured types ──────────────────────────────────────────────────
    /// Named struct or enum (resolved to its definition).
    Named { name: SmolStr, args: Vec<Ty> },

    // ── Function ──────────────────────────────────────────────────────────
    Fn(FnTy),

    // ── References ───────────────────────────────────────────────────────
    Ref(Box<Ty>),
    RefMut(Box<Ty>),
    Box(Box<Ty>),
    Raw(Box<Ty>),

    // ── Trait objects ─────────────────────────────────────────────────────
    Dyn(SmolStr),

    // ── Special ───────────────────────────────────────────────────────────
    /// Unresolved type variable (used during inference).
    Var(TyVar),
    /// The type of expressions that never return (`loop`, `panic`, etc.).
    Never,
    /// Unit type — `()` or `nil` as a statement result.
    Unit,
    /// Opaque error marker — prevents cascading type errors.
    Error,
}

impl Ty {
    pub fn is_numeric(&self) -> bool {
        matches!(self, Ty::Prim(PrimTy::Int | PrimTy::Float))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Ty::Prim(PrimTy::Int))
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Ty::Prim(PrimTy::Float))
    }

    pub fn is_matrix_compatible(&self) -> bool {
        matches!(self, Ty::Prim(PrimTy::Int | PrimTy::Float))
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Ty::Error)
    }

    pub fn describe(&self) -> String {
        match self {
            Ty::Prim(p) => p.name().to_owned(),
            Ty::Vec(t) => format!("vec<{}>", t.describe()),
            Ty::Map(k, v) => format!("map<{}, {}>", k.describe(), v.describe()),
            Ty::Matrix(t) => format!("matrix<{}>", t.describe()),
            Ty::Tuple(ts) => {
                let s: Vec<_> = ts.iter().map(|t| t.describe()).collect();
                format!("({})", s.join(", "))
            }
            Ty::Optional(t) => format!("?{}", t.describe()),
            Ty::Named { name, args } => {
                if args.is_empty() {
                    name.to_string()
                } else {
                    let s: Vec<_> = args.iter().map(|t| t.describe()).collect();
                    format!("{}<{}>", name, s.join(", "))
                }
            }
            Ty::Fn(f) => f.describe(),
            Ty::Never => "!".to_owned(),
            Ty::Unit  => "()".to_owned(),
            Ty::Error => "<error>".to_owned(),
            Ty::Var(v) => format!("?{}", v.0),
            _ => "<type>".to_owned(),
        }
    }
}

/// Primitive types in Kinetix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimTy {
    Int,
    Float,
    Bool,
    Char,
    String,
    Nil,
}

impl PrimTy {
    pub fn name(self) -> &'static str {
        match self {
            PrimTy::Int    => "int",
            PrimTy::Float  => "float",
            PrimTy::Bool   => "bool",
            PrimTy::Char   => "char",
            PrimTy::String => "string",
            PrimTy::Nil    => "nil",
        }
    }
}

/// Function type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnTy {
    pub params:   Vec<Ty>,
    pub ret:      Box<Ty>,
    pub is_async: bool,
}

impl FnTy {
    pub fn describe(&self) -> String {
        let params: Vec<_> = self.params.iter().map(|t| t.describe()).collect();
        format!("fn({}) -> {}", params.join(", "), self.ret.describe())
    }
}

/// A type kind alias (mostly for use in the AST→type mapping).
pub type TyKind = Ty;

/// A type variable used during Hindley-Milner type inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TyVar(pub u32);
