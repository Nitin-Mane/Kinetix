//! Expression AST nodes for Kinetix.

use smol_str::SmolStr;
use kinetix_lexer::Span;
use crate::{Node, TyAnnotation};

pub type ExprNode = Node<Expr>;

/// A Kinetix expression.
#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

/// All possible expression kinds in Kinetix.
#[derive(Debug, Clone)]
pub enum ExprKind {
    // ── Literals ──────────────────────────────────────────────────────────
    Lit(Lit),

    // ── Identifier / path ─────────────────────────────────────────────────
    /// Simple identifier: `x`
    Ident(SmolStr),
    /// Path: `std::io::println`
    Path(Vec<SmolStr>),

    // ── Operators ─────────────────────────────────────────────────────────
    /// Binary operation: `a + b`
    BinOp {
        op:  BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    /// Unary operation: `-x`, `!x`
    UnOp {
        op:      UnOp,
        operand: Box<Expr>,
    },
    /// Assignment: `x = expr`, `x += expr`
    Assign {
        op:  Option<BinOp>,  // None = plain `=`, Some(op) = compound assign
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },

    // ── Calls ─────────────────────────────────────────────────────────────
    /// Function call: `foo(a, b)`
    Call {
        callee: Box<Expr>,
        args:   Vec<Expr>,
    },
    /// Method call: `obj.method(a, b)`
    MethodCall {
        receiver: Box<Expr>,
        method:   SmolStr,
        args:     Vec<Expr>,
    },

    // ── Field / Index ─────────────────────────────────────────────────────
    /// Field access: `obj.field`
    Field {
        object: Box<Expr>,
        field:  SmolStr,
    },
    /// Index: `arr[i]` or `mat[i, j]`
    Index {
        object:  Box<Expr>,
        indices: Vec<Expr>,
    },
    /// Range slice: `..`, `..=10`, `1..`, `1..=5`
    Range {
        start:     Option<Box<Expr>>,
        end:       Option<Box<Expr>>,
        inclusive: bool,
    },

    // ── Collections ───────────────────────────────────────────────────────
    /// Vec literal: `[1, 2, 3]`
    VecLit(Vec<Expr>),
    /// Tuple literal: `(1, 2.0, "three")`
    Tuple(Vec<Expr>),
    /// Matrix literal (rows separated by `;`): `[[1,2]; [3,4]]`
    MatrixLit(Vec<Vec<Expr>>),
    /// Map literal: `{"key": value}`
    MapLit(Vec<(Expr, Expr)>),

    // ── Struct / Enum construction ────────────────────────────────────────
    /// Struct literal: `Point { x: 1.0, y: 2.0 }`
    StructLit {
        name:   Vec<SmolStr>,
        fields: Vec<(SmolStr, Expr)>,
    },
    /// Enum variant: `Shape::Circle(r)`  — parsed as a call on a path

    // ── Control flow expressions ──────────────────────────────────────────
    /// `if cond { then } [elif cond { ... }]* [else { else_block }]`
    If {
        branches:   Vec<(Expr, Block)>,  // (condition, body) pairs
        else_block: Option<Block>,
    },
    /// `match expr { pattern => expr, ... }`
    Match {
        scrutinee: Box<Expr>,
        arms:      Vec<MatchArm>,
    },
    /// Block expression: `{ stmt* expr? }`
    Block(Block),
    /// `loop { body }`
    Loop(Block),
    /// `while cond { body }`
    While {
        cond: Box<Expr>,
        body: Block,
    },
    /// `for pat in iter { body }`
    For {
        pattern: Pattern,
        iter:    Box<Expr>,
        body:    Block,
    },
    /// `return expr?`
    Return(Option<Box<Expr>>),
    /// `break expr?`
    Break(Option<Box<Expr>>),
    /// `continue`
    Continue,

    // ── Closures ──────────────────────────────────────────────────────────
    /// `|a, b| expr` or `|a: T, b: T| -> R { ... }`
    Closure {
        params:      Vec<ClosureParam>,
        return_type: Option<TyAnnotation>,
        body:        Box<Expr>,
    },

    // ── Async ─────────────────────────────────────────────────────────────
    Await(Box<Expr>),
    Spawn(Box<Expr>),

    // ── Cast ──────────────────────────────────────────────────────────────
    /// `expr as Type`
    Cast {
        expr: Box<Expr>,
        ty:   TyAnnotation,
    },

    // ── Misc ──────────────────────────────────────────────────────────────
    /// String interpolation — already resolved by parser into segments
    StringInterp(Vec<StringSegment>),
    /// `panic("message")`
    Panic(Box<Expr>),
}

// ── Literals ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Char(char),
    Nil,
}

// ── Operators ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    // Arithmetic
    Add, Sub, Mul, Div, Rem, Pow,
    // Comparison
    Eq, Ne, Lt, Le, Gt, Ge,
    // Logical
    And, Or,
    // Bitwise
    BitAnd, BitOr, BitXor, Shl, Shr,
    // Range
    Range,       // ..
    RangeInc,    // ..=
    // Element-wise (MATLAB-inspired)
    ElemMul,     // .*
    ElemDiv,     // ./
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Neg,    // -x
    Not,    // !x
    BitNot, // ~x
    Deref,  // *x
    Ref,    // &x
}

// ── Supporting types ───────────────────────────────────────────────────────

/// A block `{ stmts... [tail_expr] }`
#[derive(Debug, Clone)]
pub struct Block {
    pub stmts:     Vec<crate::Stmt>,
    pub tail_expr: Option<Box<Expr>>,
    pub span:      Span,
}

/// One arm in a `match` expression.
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard:   Option<Box<Expr>>,
    pub body:    Box<Expr>,
    pub span:    Span,
}

/// A Kinetix pattern (used in match arms, let bindings, for loops).
#[derive(Debug, Clone)]
pub enum Pattern {
    /// `_`
    Wildcard,
    /// `name`  (binding)
    Binding(SmolStr),
    /// `42`, `"str"`, `true`
    Lit(Lit),
    /// `1..=10`
    Range { start: Box<Lit>, end: Box<Lit>, inclusive: bool },
    /// `Point { x, y }`
    Struct { name: Vec<SmolStr>, fields: Vec<(SmolStr, Pattern)> },
    /// `Shape::Circle(r)`
    TupleVariant { path: Vec<SmolStr>, inner: Vec<Pattern> },
    /// `(a, b, c)`
    Tuple(Vec<Pattern>),
    /// `[first, ..rest]`
    Vec { head: Vec<Pattern>, rest: Option<SmolStr> },
    /// `x: int`  (type guard pattern)
    TypeGuard { binding: SmolStr, ty: TyAnnotation },
    /// `pat if guard`
    Guard { pattern: Box<Pattern>, guard: Box<Expr> },
}

/// A closure parameter.
#[derive(Debug, Clone)]
pub struct ClosureParam {
    pub name: SmolStr,
    pub ty:   Option<TyAnnotation>,
}

/// One segment of a string interpolation.
#[derive(Debug, Clone)]
pub enum StringSegment {
    Literal(String),
    Expr(Box<Expr>),
}
