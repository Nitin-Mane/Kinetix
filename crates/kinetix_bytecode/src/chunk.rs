//! Bytecode chunk — a compiled function's code + constant pool.

use crate::opcode::Op;

/// An entry in the constant pool.
#[derive(Debug, Clone)]
pub enum Constant {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Nil,
    /// Index into a function table (for closures).
    FnRef(u32),
}

impl Constant {
    pub fn describe(&self) -> String {
        match self {
            Constant::Int(v)    => v.to_string(),
            Constant::Float(v)  => v.to_string(),
            Constant::Bool(v)   => v.to_string(),
            Constant::String(s) => format!("{s:?}"),
            Constant::Nil       => "nil".to_owned(),
            Constant::FnRef(i)  => format!("<fn#{i}>"),
        }
    }
}

/// A single encoded instruction: 4 bytes.
/// Layout: `[opcode: u8, dest: u8, src1: u8, src2: u8]`
#[derive(Debug, Clone, Copy)]
pub struct Instruction(pub u32);

impl Instruction {
    pub fn encode(op: Op, dest: u8, src1: u8, src2: u8) -> Self {
        Self(((src2 as u32) << 24) | ((src1 as u32) << 16) | ((dest as u32) << 8) | (op as u32))
    }

    pub fn op(self)   -> u8 { (self.0 & 0xFF) as u8 }
    pub fn dest(self) -> u8 { ((self.0 >> 8) & 0xFF) as u8 }
    pub fn src1(self) -> u8 { ((self.0 >> 16) & 0xFF) as u8 }
    pub fn src2(self) -> u8 { ((self.0 >> 24) & 0xFF) as u8 }

    /// 16-bit signed immediate (combines src1 and src2).
    pub fn imm16(self) -> i16 {
        let hi = ((self.0 >> 24) & 0xFF) as u16;
        let lo = ((self.0 >> 16) & 0xFF) as u16;
        ((hi << 8) | lo) as i16
    }
}

/// A compiled function body (code + constant pool + metadata).
#[derive(Debug, Clone)]
pub struct Chunk {
    /// Function name (for disassembly / stack traces).
    pub name:        String,
    /// The encoded instructions.
    pub code:        Vec<Instruction>,
    /// Constant pool.
    pub constants:   Vec<Constant>,
    /// Number of register slots needed.
    pub register_count: u8,
    /// Number of parameters.
    pub param_count:    u8,
    /// Source line mapping: code_offset → line number.
    pub lines:       Vec<(usize, u32)>,
    /// Nested function chunks (for closures).
    pub nested:      Vec<Chunk>,
}

impl Chunk {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name:           name.into(),
            code:           Vec::new(),
            constants:      Vec::new(),
            register_count: 0,
            param_count:    0,
            lines:          Vec::new(),
            nested:         Vec::new(),
        }
    }

    /// Emit an instruction.
    pub fn emit(&mut self, op: Op, dest: u8, src1: u8, src2: u8, line: u32) -> usize {
        let offset = self.code.len();
        self.code.push(Instruction::encode(op, dest, src1, src2));
        if self.lines.last().map(|(_, l)| *l) != Some(line) {
            self.lines.push((offset, line));
        }
        offset
    }

    /// Add a constant to the pool and return its index.
    pub fn add_constant(&mut self, c: Constant) -> u8 {
        // Deduplicate string/int constants
        let idx = self.constants.len();
        self.constants.push(c);
        idx as u8
    }

    /// Get the source line for an instruction offset.
    pub fn line_at(&self, offset: usize) -> u32 {
        let mut line = 0;
        for &(off, l) in &self.lines {
            if off <= offset { line = l; } else { break; }
        }
        line
    }

    /// Disassemble the chunk to a string.
    pub fn disassemble(&self) -> String {
        let mut out = format!("=== {} ===\n", self.name);
        for (i, instr) in self.code.iter().enumerate() {
            let op_byte = instr.op();
            let op_name = format!("Op(0x{op_byte:02X})");
            let line = self.line_at(i);
            out += &format!(
                "{i:04}  L{line:04}  {op_name}  dest={} src1={} src2={}\n",
                instr.dest(), instr.src1(), instr.src2()
            );
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opcode::Op;

    #[test]
    fn test_instruction_encoding() {
        let instr = Instruction::encode(Op::Add, 1, 2, 3);
        assert_eq!(instr.op(),   Op::Add as u8);
        assert_eq!(instr.dest(), 1);
        assert_eq!(instr.src1(), 2);
        assert_eq!(instr.src2(), 3);
    }

    #[test]
    fn test_chunk_emit() {
        let mut chunk = Chunk::new("test");
        let _ = chunk.add_constant(Constant::Int(42));
        chunk.emit(Op::LoadConst, 0, 0, 0, 1);
        chunk.emit(Op::Return, 0, 0, 0, 1);
        assert_eq!(chunk.code.len(), 2);
    }
}
