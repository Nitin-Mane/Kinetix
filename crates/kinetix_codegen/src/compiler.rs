//! HIR → Bytecode compiler.

use std::collections::HashMap;
use std::sync::Arc;
use smol_str::SmolStr;
use thiserror::Error;
use kinetix_bytecode::{Chunk, Constant, Op};
use kinetix_hir::{
    HirFile, HirItem,
    expr::{BinOpKind, HirBlock, HirExpr, HirExprKind, HirStmt, UnOpKind},
    item::HirFn,
};

#[derive(Debug, Clone, Error)]
pub enum CodegenError {
    #[error("codegen: {0}")]
    Other(String),
    #[error("codegen: too many local variables (max 255)")]
    TooManyLocals,
    #[error("codegen: too many constants in constant pool (max 255)")]
    TooManyConstants,
}

pub type CodegenResult<T> = Result<T, CodegenError>;

// ── Local variable scope ──────────────────────────────────────────────────

#[derive(Debug)]
struct Local {
    name: SmolStr,
    slot: u8,
}

// ── Compiler ──────────────────────────────────────────────────────────────

pub struct Compiler {
    chunk:     Chunk,
    locals:    Vec<Local>,
    next_reg:  u8,
    /// Pending forward jumps that need patching.
    patches:   Vec<(usize, u8)>, // (instruction offset, dest reg)
}

impl Compiler {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            chunk:    Chunk::new(name),
            locals:   Vec::new(),
            next_reg: 0,
            patches:  Vec::new(),
        }
    }

    /// Compile a whole HIR file — returns a top-level chunk.
    pub fn compile_file(file: &HirFile) -> CodegenResult<Chunk> {
        let mut compiler = Self::new("<main>");
        for item in &file.items {
            match item {
                HirItem::Fn(f) => compiler.compile_fn(f)?,
            }
        }
        compiler.chunk.emit(Op::ReturnNil, 0, 0, 0, 0);
        compiler.chunk.register_count = compiler.next_reg;
        Ok(compiler.chunk)
    }

    fn compile_fn(&mut self, func: &HirFn) -> CodegenResult<()> {
        // Register the function name as a global constant
        let name_idx = self.add_const(Constant::String(func.name.to_string()))?;

        if let Some(body) = &func.body {
            // Compile body
            let mut fn_compiler = Compiler::new(func.name.as_str());
            // Bind parameters as locals
            for param in &func.params {
                fn_compiler.define_local(param.name.clone())?;
            }
            fn_compiler.compile_block(body)?;
            fn_compiler.chunk.emit(Op::ReturnNil, 0, 0, 0, 0);
            fn_compiler.chunk.param_count = func.params.len() as u8;
            fn_compiler.chunk.register_count = fn_compiler.next_reg;

            // Emit a closure instruction referencing the nested function
            let fn_idx = self.chunk.nested.len() as u8;
            self.chunk.nested.push(fn_compiler.chunk);
            let fn_const = self.add_const(Constant::FnRef(fn_idx as u32))?;
            let reg = self.alloc_reg();
            self.chunk.emit(Op::LoadConst, reg, fn_const, 0, 0);
            self.chunk.emit(Op::StoreGlobal, reg, name_idx, 0, 0);
        }
        Ok(())
    }

    fn compile_block(&mut self, block: &HirBlock) -> CodegenResult<()> {
        for stmt in &block.stmts {
            self.compile_stmt(stmt)?;
        }
        if let Some(tail) = &block.tail_expr {
            self.compile_expr(tail)?;
        }
        Ok(())
    }

    fn compile_stmt(&mut self, stmt: &HirStmt) -> CodegenResult<()> {
        match stmt {
            HirStmt::Let { name, init, .. } => {
                let slot = if let Some(expr) = init {
                    let reg = self.compile_expr(expr)?;
                    self.define_local_at(name.clone(), reg)?;
                    reg
                } else {
                    let reg = self.alloc_reg();
                    self.chunk.emit(Op::LoadNil, reg, 0, 0, 0);
                    self.define_local_at(name.clone(), reg)?;
                    reg
                };
                let _ = slot;
            }
            HirStmt::Expr(expr) => { self.compile_expr(expr)?; }
        }
        Ok(())
    }

    fn compile_expr(&mut self, expr: &HirExpr) -> CodegenResult<u8> {
        match &expr.kind {
            HirExprKind::IntLit(v) => {
                let reg = self.alloc_reg();
                if *v >= i16::MIN as i64 && *v <= i16::MAX as i64 {
                    let imm = *v as i16 as u16;
                    let hi  = ((imm >> 8) & 0xFF) as u8;
                    let lo  = (imm & 0xFF) as u8;
                    self.chunk.emit(Op::LoadInt, reg, lo, hi, 0);
                } else {
                    let idx = self.add_const(Constant::Int(*v))?;
                    self.chunk.emit(Op::LoadConst, reg, idx, 0, 0);
                }
                Ok(reg)
            }
            HirExprKind::FloatLit(v) => {
                let idx = self.add_const(Constant::Float(*v))?;
                let reg = self.alloc_reg();
                self.chunk.emit(Op::LoadConst, reg, idx, 0, 0);
                Ok(reg)
            }
            HirExprKind::BoolLit(v) => {
                let reg = self.alloc_reg();
                self.chunk.emit(if *v { Op::LoadTrue } else { Op::LoadFalse }, reg, 0, 0, 0);
                Ok(reg)
            }
            HirExprKind::StringLit(s) => {
                let idx = self.add_const(Constant::String(s.clone()))?;
                let reg = self.alloc_reg();
                self.chunk.emit(Op::LoadConst, reg, idx, 0, 0);
                Ok(reg)
            }
            HirExprKind::Nil => {
                let reg = self.alloc_reg();
                self.chunk.emit(Op::LoadNil, reg, 0, 0, 0);
                Ok(reg)
            }
            HirExprKind::Var(name) => {
                // Look up as local first, then global
                if let Some(local) = self.locals.iter().rev().find(|l| l.name == *name) {
                    Ok(local.slot)
                } else {
                    let name_idx = self.add_const(Constant::String(name.to_string()))?;
                    let reg = self.alloc_reg();
                    self.chunk.emit(Op::LoadGlobal, reg, name_idx, 0, 0);
                    Ok(reg)
                }
            }
            HirExprKind::BinOp { op, lhs, rhs } => {
                let l = self.compile_expr(lhs)?;
                let r = self.compile_expr(rhs)?;
                let dest = self.alloc_reg();
                let bc_op = match op {
                    BinOpKind::Add => Op::Add,
                    BinOpKind::Sub => Op::Sub,
                    BinOpKind::Mul => Op::Mul,
                    BinOpKind::Div => Op::Div,
                    BinOpKind::Rem => Op::Rem,
                    BinOpKind::Pow => Op::Pow,
                    BinOpKind::Eq  => Op::Eq,
                    BinOpKind::Ne  => Op::Ne,
                    BinOpKind::Lt  => Op::Lt,
                    BinOpKind::Le  => Op::Le,
                    BinOpKind::Gt  => Op::Gt,
                    BinOpKind::Ge  => Op::Ge,
                    BinOpKind::And => Op::And,
                    BinOpKind::Or  => Op::Or,
                    BinOpKind::Shl => Op::Shl,
                    BinOpKind::Shr => Op::Shr,
                    BinOpKind::BitAnd => Op::BitAnd,
                    BinOpKind::BitOr  => Op::BitOr,
                    BinOpKind::BitXor => Op::BitXor,
                };
                self.chunk.emit(bc_op, dest, l, r, 0);
                Ok(dest)
            }
            HirExprKind::UnOp { op, operand } => {
                let src = self.compile_expr(operand)?;
                let dest = self.alloc_reg();
                let bc_op = match op {
                    UnOpKind::Neg    => Op::Neg,
                    UnOpKind::Not    => Op::Not,
                    UnOpKind::BitNot => Op::BitNot,
                };
                self.chunk.emit(bc_op, dest, src, 0, 0);
                Ok(dest)
            }
            HirExprKind::Call { callee, args } => {
                let fn_reg  = self.compile_expr(callee)?;
                let mut arg_regs = Vec::new();
                for arg in args {
                    arg_regs.push(self.compile_expr(arg)?);
                }
                let dest = self.alloc_reg();
                self.chunk.emit(Op::Call, dest, fn_reg, args.len() as u8, 0);
                Ok(dest)
            }
            HirExprKind::Return(val) => {
                let reg = if let Some(v) = val {
                    self.compile_expr(v)?
                } else {
                    let r = self.alloc_reg();
                    self.chunk.emit(Op::LoadNil, r, 0, 0, 0);
                    r
                };
                self.chunk.emit(Op::Return, reg, 0, 0, 0);
                Ok(reg)
            }
            HirExprKind::Block(block) => {
                self.compile_block(block)?;
                Ok(0)
            }
            HirExprKind::VecLit(elems) => {
                let vec_reg = self.alloc_reg();
                self.chunk.emit(Op::NewVec, vec_reg, 0, 0, 0);
                for elem in elems {
                    let elem_reg = self.compile_expr(elem)?;
                    self.chunk.emit(Op::VecPush, vec_reg, elem_reg, 0, 0);
                }
                Ok(vec_reg)
            }
            HirExprKind::Panic(msg) => {
                let msg_reg = self.compile_expr(msg)?;
                self.chunk.emit(Op::Panic, msg_reg, 0, 0, 0);
                Ok(msg_reg)
            }
            _ => {
                // Unimplemented node — emit nil
                let reg = self.alloc_reg();
                self.chunk.emit(Op::LoadNil, reg, 0, 0, 0);
                Ok(reg)
            }
        }
    }

    fn alloc_reg(&mut self) -> u8 {
        let r = self.next_reg;
        self.next_reg = self.next_reg.saturating_add(1);
        if r > self.chunk.register_count { self.chunk.register_count = r; }
        r
    }

    fn add_const(&mut self, c: Constant) -> CodegenResult<u8> {
        if self.chunk.constants.len() >= 256 {
            return Err(CodegenError::TooManyConstants);
        }
        Ok(self.chunk.add_constant(c))
    }

    fn define_local(&mut self, name: SmolStr) -> CodegenResult<u8> {
        let reg = self.alloc_reg();
        self.define_local_at(name, reg)?;
        Ok(reg)
    }

    fn define_local_at(&mut self, name: SmolStr, slot: u8) -> CodegenResult<()> {
        if self.locals.len() >= 255 {
            return Err(CodegenError::TooManyLocals);
        }
        self.locals.push(Local { name, slot });
        Ok(())
    }
}
