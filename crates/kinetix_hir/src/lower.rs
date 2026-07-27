//! AST → HIR lowering pass.
//!
//! This module converts the parsed AST (with type annotations from the
//! type checker) into the typed HIR.

use smol_str::SmolStr;
use kinetix_ast::{SourceFile, expr::ExprKind, stmt::StmtKind};
use kinetix_types::{TypeChecker, Ty, PrimTy};
use crate::{
    HirFile, HirId,
    expr::{BinOpKind, HirBlock, HirExpr, HirExprKind, HirStmt, UnOpKind},
    item::{HirFn, HirItem, HirParam},
};

/// Lowers an AST source file to HIR using type information from `checker`.
pub fn lower_file(file: &SourceFile, checker: &mut TypeChecker) -> HirFile {
    let mut lowerer = Lowerer::new(checker);
    let items: Vec<_> = file.items.iter()
        .filter_map(|node| lowerer.lower_item(&node.inner))
        .collect();
    HirFile { items, path: file.path.clone() }
}

struct Lowerer<'c> {
    checker: &'c mut TypeChecker,
    next_id: u32,
}

impl<'c> Lowerer<'c> {
    fn new(checker: &'c mut TypeChecker) -> Self {
        Self { checker, next_id: 0 }
    }

    fn fresh_id(&mut self) -> HirId {
        let id = self.next_id;
        self.next_id += 1;
        HirId(id)
    }

    fn lower_item(&mut self, item: &kinetix_ast::stmt::Item) -> Option<HirItem> {
        use kinetix_ast::stmt::ItemKind;
        match &item.kind {
            ItemKind::Fn(fn_def) => {
                let params: Vec<HirParam> = fn_def.sig.params.iter().map(|p| HirParam {
                    name: p.name.clone(),
                    ty: self.checker.ast_ty_to_ty(&p.ty),
                }).collect();
                let ret_ty = fn_def.sig.return_type.as_ref()
                    .map(|r| self.checker.ast_ty_to_ty(r))
                    .unwrap_or(Ty::Unit);
                let body = fn_def.body.as_ref().map(|b| self.lower_block(b));
                Some(HirItem::Fn(HirFn {
                    name:     fn_def.sig.name.clone(),
                    params,
                    ret_ty,
                    body,
                    is_async: fn_def.sig.is_async,
                }))
            }
            _ => None,
        }
    }

    fn lower_block(&mut self, block: &kinetix_ast::expr::Block) -> HirBlock {
        let stmts: Vec<_> = block.stmts.iter()
            .filter_map(|s| self.lower_stmt(s))
            .collect();
        let tail_expr = block.tail_expr.as_ref()
            .map(|e| Box::new(self.lower_expr(e)));
        HirBlock { stmts, tail_expr }
    }

    fn lower_stmt(&mut self, stmt: &kinetix_ast::stmt::Stmt) -> Option<HirStmt> {
        match &stmt.kind {
            StmtKind::Let { pattern, ty, init, .. } => {
                use kinetix_ast::expr::Pattern;
                let name = match pattern {
                    Pattern::Binding(n) => n.clone(),
                    _ => SmolStr::new("_"),
                };
                let inferred = init.as_ref()
                    .map(|e| self.checker.check_expr(e))
                    .unwrap_or(Ty::Unit);
                let ann_ty = ty.as_ref()
                    .map(|t| self.checker.ast_ty_to_ty(t))
                    .unwrap_or(inferred);
                let init_hir = init.as_ref().map(|e| self.lower_expr(e));
                Some(HirStmt::Let { name, ty: ann_ty, init: init_hir })
            }
            StmtKind::Expr(e) => Some(HirStmt::Expr(self.lower_expr(e))),
            _ => None,
        }
    }

    fn lower_expr(&mut self, expr: &kinetix_ast::expr::Expr) -> HirExpr {
        let id = self.fresh_id();
        let ty = self.checker.check_expr(expr);
        let kind = match &expr.kind {
            ExprKind::Lit(lit) => {
                use kinetix_ast::expr::Lit;
                match lit {
                    Lit::Int(v)    => HirExprKind::IntLit(*v),
                    Lit::Float(v)  => HirExprKind::FloatLit(*v),
                    Lit::Bool(v)   => HirExprKind::BoolLit(*v),
                    Lit::String(s) => HirExprKind::StringLit(s.clone()),
                    Lit::Nil       => HirExprKind::Nil,
                    Lit::Char(c)   => HirExprKind::StringLit(c.to_string()),
                }
            }
            ExprKind::Ident(name) => HirExprKind::Var(name.clone()),
            ExprKind::BinOp { op, lhs, rhs } => {
                use kinetix_ast::expr::BinOp;
                let hir_op = match op {
                    BinOp::Add => BinOpKind::Add,
                    BinOp::Sub => BinOpKind::Sub,
                    BinOp::Mul => BinOpKind::Mul,
                    BinOp::Div => BinOpKind::Div,
                    BinOp::Rem => BinOpKind::Rem,
                    BinOp::Pow => BinOpKind::Pow,
                    BinOp::Eq  => BinOpKind::Eq,
                    BinOp::Ne  => BinOpKind::Ne,
                    BinOp::Lt  => BinOpKind::Lt,
                    BinOp::Le  => BinOpKind::Le,
                    BinOp::Gt  => BinOpKind::Gt,
                    BinOp::Ge  => BinOpKind::Ge,
                    BinOp::And => BinOpKind::And,
                    BinOp::Or  => BinOpKind::Or,
                    _          => BinOpKind::Add,
                };
                HirExprKind::BinOp {
                    op:  hir_op,
                    lhs: Box::new(self.lower_expr(lhs)),
                    rhs: Box::new(self.lower_expr(rhs)),
                }
            }
            ExprKind::Call { callee, args } => HirExprKind::Call {
                callee: Box::new(self.lower_expr(callee)),
                args:   args.iter().map(|a| self.lower_expr(a)).collect(),
            },
            ExprKind::Return(val) => HirExprKind::Return(
                val.as_ref().map(|v| Box::new(self.lower_expr(v)))
            ),
            ExprKind::Block(b) => HirExprKind::Block(self.lower_block(b)),
            ExprKind::VecLit(elems) => HirExprKind::VecLit(
                elems.iter().map(|e| self.lower_expr(e)).collect()
            ),
            ExprKind::MatrixLit(rows) => HirExprKind::MatrixLit(
                rows.iter().map(|row| row.iter().map(|e| self.lower_expr(e)).collect()).collect()
            ),
            _ => HirExprKind::Nil,
        };
        HirExpr { id, kind, ty }
    }
}
