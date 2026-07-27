//! # kinetix_mir
//!
//! Mid-level Intermediate Representation (MIR) and optimization passes.
//!
//! MIR uses a **Control Flow Graph (CFG)** with **basic blocks** and
//! **three-address code** (TAC) statements. This form is ideal for:
//!
//! - Constant folding and propagation
//! - Dead code elimination
//! - SSA (Static Single Assignment) form (future)
//! - Register allocation preparation

use smol_str::SmolStr;
use kinetix_types::Ty;

// ── MIR Operands ──────────────────────────────────────────────────────────

/// A MIR local variable slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Local(pub u32);

/// A MIR operand — either a local variable or a constant.
#[derive(Debug, Clone)]
pub enum Operand {
    Local(Local),
    ConstInt(i64),
    ConstFloat(f64),
    ConstBool(bool),
    ConstString(String),
    Nil,
}

// ── MIR Instructions (Three-address code) ────────────────────────────────

#[derive(Debug, Clone)]
pub enum Instr {
    /// `local = operand`
    Assign { dest: Local, src: Operand },
    /// `local = op(a, b)`
    BinOp  { dest: Local, op: MirBinOp, a: Operand, b: Operand },
    /// `local = op(a)`
    UnOp   { dest: Local, op: MirUnOp, a: Operand },
    /// `local = callee(args...)`
    Call   { dest: Option<Local>, callee: Operand, args: Vec<Operand> },
    /// `local = &base[index]`
    Index  { dest: Local, base: Operand, index: Operand },
    /// `local = alloc(ty)`
    Alloc  { dest: Local, ty: Ty },
    /// `local = vec[elems...]`
    VecNew { dest: Local, elems: Vec<Operand> },
    /// `local = matrix(rows, cols, data...)`
    MatrixNew { dest: Local, rows: usize, cols: usize, data: Vec<Operand> },
    /// NOP — useful as placeholder during optimization.
    Nop,
}

#[derive(Debug, Clone, Copy)]
pub enum MirBinOp { Add, Sub, Mul, Div, Rem, Pow, Eq, Ne, Lt, Le, Gt, Ge, And, Or, Shl, Shr, BitAnd, BitOr, BitXor }

#[derive(Debug, Clone, Copy)]
pub enum MirUnOp { Neg, Not, BitNot }

// ── Terminators ───────────────────────────────────────────────────────────

/// Each basic block ends with a terminator.
#[derive(Debug, Clone)]
pub enum Terminator {
    /// `return operand`
    Return(Option<Operand>),
    /// Unconditional jump to block.
    Goto(BlockId),
    /// `if cond { goto then } else { goto else }`
    Branch { cond: Operand, then_block: BlockId, else_block: BlockId },
    /// Unreachable (follows `panic`)
    Unreachable,
}

// ── Basic blocks ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub u32);

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id:         BlockId,
    pub instrs:     Vec<Instr>,
    pub terminator: Terminator,
}

// ── MIR Function ──────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct MirFn {
    pub name:       SmolStr,
    pub locals:     Vec<(Local, Ty)>,   // (slot, type)
    pub params:     Vec<Local>,
    pub ret_ty:     Ty,
    pub blocks:     Vec<BasicBlock>,
    pub entry:      BlockId,
}

// ── Optimization passes ───────────────────────────────────────────────────

/// Run constant folding over all blocks in a function.
pub fn const_fold(func: &mut MirFn) {
    for block in &mut func.blocks {
        for instr in &mut block.instrs {
            if let Instr::BinOp { dest, op, a, b } = instr {
                if let (Operand::ConstInt(ai), Operand::ConstInt(bi)) = (a.clone(), b.clone()) {
                    let result = match op {
                        MirBinOp::Add => Some(ai.wrapping_add(bi)),
                        MirBinOp::Sub => Some(ai.wrapping_sub(bi)),
                        MirBinOp::Mul => Some(ai.wrapping_mul(bi)),
                        MirBinOp::Div if bi != 0 => Some(ai / bi),
                        MirBinOp::Rem if bi != 0 => Some(ai % bi),
                        _ => None,
                    };
                    if let Some(v) = result {
                        *instr = Instr::Assign { dest: *dest, src: Operand::ConstInt(v) };
                    }
                }
            }
        }
    }
}

/// Dead code elimination: remove `Nop` instructions.
pub fn dead_code_elim(func: &mut MirFn) {
    for block in &mut func.blocks {
        block.instrs.retain(|i| !matches!(i, Instr::Nop));
    }
}
