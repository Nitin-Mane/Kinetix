//! Type checker for the Kinetix language.

use std::collections::HashMap;
use smol_str::SmolStr;
use thiserror::Error;
use kinetix_lexer::Span;
use kinetix_ast::{
    expr::{BinOp, Expr, ExprKind, Lit, UnOp},
    stmt::{Stmt, StmtKind, Item, ItemKind},
    SourceFile,
};
use crate::ty::{FnTy, PrimTy, Ty};
use crate::inference::InferCtx;

// ── Type check error ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Error)]
pub enum CheckError {
    #[error("{span:?}: type mismatch — expected `{expected}`, found `{found}`")]
    TypeMismatch { span: Span, expected: String, found: String },

    #[error("{span:?}: unknown variable `{name}`")]
    UnknownVariable { span: Span, name: String },

    #[error("{span:?}: unknown type `{name}`")]
    UnknownType { span: Span, name: String },

    #[error("{span:?}: wrong number of arguments — expected {expected}, found {found}")]
    ArityMismatch { span: Span, expected: usize, found: usize },

    #[error("{span:?}: operator `{op}` not supported for type `{ty}`")]
    InvalidOperator { span: Span, op: String, ty: String },

    #[error("{span:?}: {message}")]
    Other { span: Span, message: String },
}

// ── Type environment ──────────────────────────────────────────────────────

/// Variable → type mapping, with lexical scoping.
pub struct TypeEnv {
    scopes: Vec<HashMap<SmolStr, Ty>>,
}

impl TypeEnv {
    pub fn new() -> Self {
        Self { scopes: vec![HashMap::new()] }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn insert(&mut self, name: SmolStr, ty: Ty) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, ty);
        }
    }

    pub fn get(&self, name: &str) -> Option<&Ty> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty);
            }
        }
        None
    }
}

impl Default for TypeEnv {
    fn default() -> Self { Self::new() }
}

// ── Type checker ──────────────────────────────────────────────────────────

/// Walks the AST and assigns a [`Ty`] to each expression.
pub struct TypeChecker {
    pub env:    TypeEnv,
    pub infer:  InferCtx,
    pub errors: Vec<CheckError>,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut env = TypeEnv::new();
        // Seed built-in functions
        env.insert(SmolStr::new("println"), Ty::Fn(FnTy {
            params: vec![Ty::Prim(PrimTy::String)],
            ret: Box::new(Ty::Unit),
            is_async: false,
        }));
        Self { env, infer: InferCtx::new(), errors: Vec::new() }
    }

    /// Type-check an entire source file.
    pub fn check_file(&mut self, file: &SourceFile) {
        for node in &file.items {
            self.check_item(&node.inner);
        }
    }

    fn check_item(&mut self, item: &Item) {
        match &item.kind {
            ItemKind::Fn(fn_def) => {
                // Register function signature in env
                let param_tys: Vec<Ty> = fn_def.sig.params.iter()
                    .map(|p| self.ast_ty_to_ty(&p.ty))
                    .collect();
                let ret_ty = fn_def.sig.return_type.as_ref()
                    .map(|r| self.ast_ty_to_ty(r))
                    .unwrap_or(Ty::Unit);
                let fn_ty = Ty::Fn(FnTy {
                    params:   param_tys.clone(),
                    ret:      Box::new(ret_ty.clone()),
                    is_async: fn_def.sig.is_async,
                });
                self.env.insert(fn_def.sig.name.clone(), fn_ty);

                // Check body
                if let Some(body) = &fn_def.body {
                    self.env.push_scope();
                    for (param, ty) in fn_def.sig.params.iter().zip(param_tys.iter()) {
                        self.env.insert(param.name.clone(), ty.clone());
                    }
                    for stmt in &body.stmts {
                        self.check_stmt(stmt);
                    }
                    if let Some(tail) = &body.tail_expr {
                        let actual = self.check_expr(tail);
                        if let Err(msg) = self.infer.unify(&actual, &ret_ty) {
                            self.errors.push(CheckError::TypeMismatch {
                                span: tail.span,
                                expected: ret_ty.describe(),
                                found: actual.describe(),
                            });
                            let _ = msg;
                        }
                    }
                    self.env.pop_scope();
                }
            }
            _ => {
                // Other items: struct, enum, impl, trait — handled in future passes
            }
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::Let { pattern, ty, init, .. } => {
                let init_ty = init.as_ref().map(|e| self.check_expr(e)).unwrap_or_else(|| self.infer.fresh_var());
                if let Some(ann) = ty {
                    let ann_ty = self.ast_ty_to_ty(ann);
                    if let Err(_) = self.infer.unify(&init_ty, &ann_ty) {
                        self.errors.push(CheckError::TypeMismatch {
                            span: stmt.span,
                            expected: ann_ty.describe(),
                            found: init_ty.describe(),
                        });
                    }
                }
                // Bind pattern name
                if let kinetix_ast::expr::Pattern::Binding(name) = pattern {
                    let resolved = self.infer.resolve(&init_ty);
                    self.env.insert(name.clone(), resolved);
                }
            }
            StmtKind::Expr(expr) => { self.check_expr(expr); }
            _ => {}
        }
    }

    pub fn check_expr(&mut self, expr: &Expr) -> Ty {
        match &expr.kind {
            ExprKind::Lit(lit) => self.check_lit(lit),

            ExprKind::Ident(name) => {
                self.env.get(name.as_str()).cloned().unwrap_or_else(|| {
                    self.errors.push(CheckError::UnknownVariable {
                        span: expr.span,
                        name: name.to_string(),
                    });
                    Ty::Error
                })
            }

            ExprKind::BinOp { op, lhs, rhs } => self.check_binop(*op, lhs, rhs, expr.span),

            ExprKind::UnOp { op, operand } => {
                let ty = self.check_expr(operand);
                match op {
                    UnOp::Neg => {
                        if !ty.is_numeric() {
                            self.errors.push(CheckError::InvalidOperator {
                                span: expr.span,
                                op: "-".to_owned(),
                                ty: ty.describe(),
                            });
                            Ty::Error
                        } else { ty }
                    }
                    UnOp::Not => Ty::Prim(PrimTy::Bool),
                    _ => ty,
                }
            }

            ExprKind::Call { callee, args } => {
                let callee_ty = self.check_expr(callee);
                let arg_tys: Vec<_> = args.iter().map(|a| self.check_expr(a)).collect();
                if let Ty::Fn(fn_ty) = &callee_ty {
                    if fn_ty.params.len() != arg_tys.len() {
                        self.errors.push(CheckError::ArityMismatch {
                            span: expr.span,
                            expected: fn_ty.params.len(),
                            found: arg_tys.len(),
                        });
                        return Ty::Error;
                    }
                    *fn_ty.ret.clone()
                } else {
                    self.infer.fresh_var()
                }
            }

            ExprKind::If { branches, else_block } => {
                let mut result_ty = Ty::Unit;
                for (cond, body) in branches {
                    let cond_ty = self.check_expr(cond);
                    if let Err(_) = self.infer.unify(&cond_ty, &Ty::Prim(PrimTy::Bool)) {
                        self.errors.push(CheckError::TypeMismatch {
                            span: expr.span,
                            expected: "bool".to_owned(),
                            found: cond_ty.describe(),
                        });
                    }
                    self.env.push_scope();
                    for s in &body.stmts { self.check_stmt(s); }
                    if let Some(tail) = &body.tail_expr {
                        result_ty = self.check_expr(tail);
                    }
                    self.env.pop_scope();
                }
                if let Some(else_b) = else_block {
                    self.env.push_scope();
                    for s in &else_b.stmts { self.check_stmt(s); }
                    if let Some(tail) = &else_b.tail_expr {
                        let else_ty = self.check_expr(tail);
                        let _ = self.infer.unify(&result_ty, &else_ty);
                    }
                    self.env.pop_scope();
                }
                result_ty
            }

            ExprKind::Block(block) => {
                self.env.push_scope();
                for s in &block.stmts { self.check_stmt(s); }
                let ty = block.tail_expr.as_ref()
                    .map(|e| self.check_expr(e))
                    .unwrap_or(Ty::Unit);
                self.env.pop_scope();
                ty
            }

            ExprKind::VecLit(elems) => {
                let elem_ty = if elems.is_empty() {
                    self.infer.fresh_var()
                } else {
                    let first = self.check_expr(&elems[0]);
                    for e in &elems[1..] {
                        let t = self.check_expr(e);
                        let _ = self.infer.unify(&first, &t);
                    }
                    first
                };
                Ty::Vec(Box::new(elem_ty))
            }

            ExprKind::MatrixLit(rows) => {
                let elem_ty = if rows.is_empty() || rows[0].is_empty() {
                    Ty::Prim(PrimTy::Float)
                } else {
                    self.check_expr(&rows[0][0])
                };
                Ty::Matrix(Box::new(elem_ty))
            }

            ExprKind::Return(val) => {
                if let Some(v) = val { self.check_expr(v); }
                Ty::Never
            }

            ExprKind::Assign { lhs, rhs, .. } => {
                let lhs_ty = self.check_expr(lhs);
                let rhs_ty = self.check_expr(rhs);
                let _ = self.infer.unify(&lhs_ty, &rhs_ty);
                Ty::Unit
            }

            _ => self.infer.fresh_var(),
        }
    }

    fn check_lit(&self, lit: &Lit) -> Ty {
        match lit {
            Lit::Int(_)    => Ty::Prim(PrimTy::Int),
            Lit::Float(_)  => Ty::Prim(PrimTy::Float),
            Lit::Bool(_)   => Ty::Prim(PrimTy::Bool),
            Lit::String(_) => Ty::Prim(PrimTy::String),
            Lit::Char(_)   => Ty::Prim(PrimTy::Char),
            Lit::Nil       => Ty::Unit,
        }
    }

    fn check_binop(&mut self, op: BinOp, lhs: &Expr, rhs: &Expr, span: Span) -> Ty {
        let lhs_ty = self.check_expr(lhs);
        let rhs_ty = self.check_expr(rhs);

        match op {
            // Arithmetic — both sides must be numeric
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Rem | BinOp::Pow => {
                if let Err(_) = self.infer.unify(&lhs_ty, &rhs_ty) {
                    self.errors.push(CheckError::TypeMismatch {
                        span, expected: lhs_ty.describe(), found: rhs_ty.describe(),
                    });
                    Ty::Error
                } else { lhs_ty }
            }
            // Comparison — result is bool
            BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                Ty::Prim(PrimTy::Bool)
            }
            // Logical — both must be bool, result is bool
            BinOp::And | BinOp::Or => {
                let _ = self.infer.unify(&lhs_ty, &Ty::Prim(PrimTy::Bool));
                let _ = self.infer.unify(&rhs_ty, &Ty::Prim(PrimTy::Bool));
                Ty::Prim(PrimTy::Bool)
            }
            // Range — produces a vec<int> or vec<float>
            BinOp::Range | BinOp::RangeInc => {
                Ty::Vec(Box::new(lhs_ty))
            }
            _ => self.infer.fresh_var(),
        }
    }

    /// Convert an AST type annotation to a concrete [`Ty`].
    pub fn ast_ty_to_ty(&mut self, ann: &kinetix_ast::TyAnnotation) -> Ty {
        use kinetix_ast::types::TyKind;
        match &ann.kind {
            TyKind::Int    => Ty::Prim(PrimTy::Int),
            TyKind::Float  => Ty::Prim(PrimTy::Float),
            TyKind::Bool   => Ty::Prim(PrimTy::Bool),
            TyKind::Char   => Ty::Prim(PrimTy::Char),
            TyKind::String => Ty::Prim(PrimTy::String),
            TyKind::Nil    => Ty::Unit,
            TyKind::Vec(t) => Ty::Vec(Box::new(self.ast_ty_to_ty(t))),
            TyKind::Map(k, v) => Ty::Map(Box::new(self.ast_ty_to_ty(k)), Box::new(self.ast_ty_to_ty(v))),
            TyKind::Matrix(t) => Ty::Matrix(Box::new(self.ast_ty_to_ty(t))),
            TyKind::Optional(t) => Ty::Optional(Box::new(self.ast_ty_to_ty(t))),
            TyKind::Infer  => self.infer.fresh_var(),
            TyKind::Never  => Ty::Never,
            TyKind::Named { name, generics } => {
                Ty::Named {
                    name: name.clone(),
                    args: generics.iter().map(|g| self.ast_ty_to_ty(g)).collect(),
                }
            }
            _ => self.infer.fresh_var(),
        }
    }
}

impl Default for TypeChecker {
    fn default() -> Self { Self::new() }
}
