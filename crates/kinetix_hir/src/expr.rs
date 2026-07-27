//! HIR expression nodes (type-annotated).

use kinetix_types::Ty;
use smol_str::SmolStr;
use crate::HirId;

/// A type-annotated HIR expression.
#[derive(Debug, Clone)]
pub struct HirExpr {
    pub id:   HirId,
    pub kind: HirExprKind,
    pub ty:   Ty,
}

#[derive(Debug, Clone)]
pub enum HirExprKind {
    IntLit(i64),
    FloatLit(f64),
    BoolLit(bool),
    StringLit(String),
    Nil,

    Var(SmolStr),

    Call {
        callee: Box<HirExpr>,
        args:   Vec<HirExpr>,
    },
    MethodCall {
        receiver: Box<HirExpr>,
        method:   SmolStr,
        args:     Vec<HirExpr>,
    },
    Field {
        object: Box<HirExpr>,
        field:  SmolStr,
    },
    Index {
        object:  Box<HirExpr>,
        index:   Box<HirExpr>,
    },

    BinOp {
        op:  BinOpKind,
        lhs: Box<HirExpr>,
        rhs: Box<HirExpr>,
    },
    UnOp {
        op:      UnOpKind,
        operand: Box<HirExpr>,
    },

    Assign {
        lhs: Box<HirExpr>,
        rhs: Box<HirExpr>,
    },

    If {
        cond:       Box<HirExpr>,
        then_block: HirBlock,
        else_block: Option<HirBlock>,
    },
    Loop(HirBlock),
    Return(Option<Box<HirExpr>>),
    Break(Option<Box<HirExpr>>),
    Continue,

    Block(HirBlock),
    VecLit(Vec<HirExpr>),
    MatrixLit(Vec<Vec<HirExpr>>),
    TupleLit(Vec<HirExpr>),

    Closure {
        params: Vec<HirClosureParam>,
        body:   Box<HirExpr>,
        ty:     Ty,
    },

    Cast { expr: Box<HirExpr>, ty: Ty },

    /// Panic intrinsic
    Panic(Box<HirExpr>),
}

#[derive(Debug, Clone)]
pub struct HirBlock {
    pub stmts:     Vec<HirStmt>,
    pub tail_expr: Option<Box<HirExpr>>,
}

#[derive(Debug, Clone)]
pub enum HirStmt {
    Let { name: SmolStr, ty: Ty, init: Option<HirExpr> },
    Expr(HirExpr),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOpKind {
    Add, Sub, Mul, Div, Rem, Pow,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or,
    BitAnd, BitOr, BitXor, Shl, Shr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOpKind { Neg, Not, BitNot }

#[derive(Debug, Clone)]
pub struct HirClosureParam {
    pub name: SmolStr,
    pub ty:   Ty,
}
