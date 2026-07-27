//! # kinetix_bytecode
//!
//! Bytecode format for the Kinetix VM.
//!
//! The Kinetix bytecode uses a **register-based** instruction set (not
//! stack-based) for better JIT optimization opportunities.
//!
//! Each instruction is 4 bytes (u32):
//! - bits  [0..7]  = opcode (256 opcodes max)
//! - bits  [8..15] = dest register
//! - bits [16..23] = src1 register
//! - bits [24..31] = src2 register / immediate low byte
//! For wide immediates, the next instruction word holds the full constant.

pub mod opcode;
pub mod chunk;

pub use opcode::Op;
pub use chunk::{Chunk, Constant};
