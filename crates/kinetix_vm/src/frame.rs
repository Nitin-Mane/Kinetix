//! Call frame for the Kinetix VM.

use std::sync::Arc;
use kinetix_bytecode::Chunk;
use crate::value::Value;

/// A single call frame on the VM call stack.
pub struct CallFrame {
    /// The function being executed.
    pub chunk:    Arc<Chunk>,
    /// Instruction pointer — index into `chunk.code`.
    pub ip:       usize,
    /// Register file for this frame (indexed by slot number).
    pub registers: Vec<Value>,
    /// Upvalue captures (for closures).
    pub upvalues: Vec<Value>,
    /// Base index into the global call stack (for stack traces).
    pub stack_base: usize,
}

impl CallFrame {
    pub fn new(chunk: Arc<Chunk>, register_count: usize, stack_base: usize) -> Self {
        Self {
            chunk,
            ip: 0,
            registers: vec![Value::Nil; register_count.max(32)],
            upvalues: Vec::new(),
            stack_base,
        }
    }

    /// Read a register value.
    #[inline]
    pub fn reg(&self, slot: u8) -> &Value {
        &self.registers[slot as usize]
    }

    /// Write to a register.
    #[inline]
    pub fn set_reg(&mut self, slot: u8, val: Value) {
        let slot = slot as usize;
        if slot >= self.registers.len() {
            self.registers.resize(slot + 1, Value::Nil);
        }
        self.registers[slot] = val;
    }

    /// Fetch the current instruction and advance IP.
    #[inline]
    pub fn fetch(&mut self) -> Option<kinetix_bytecode::chunk::Instruction> {
        let instr = self.chunk.code.get(self.ip).copied();
        self.ip += 1;
        instr
    }

    /// Peek at the next instruction without advancing IP.
    #[inline]
    pub fn peek_instr(&self) -> Option<kinetix_bytecode::chunk::Instruction> {
        self.chunk.code.get(self.ip).copied()
    }
}
