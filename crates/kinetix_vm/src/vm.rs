//! The Kinetix register-based virtual machine.

use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use log::trace;
use kinetix_bytecode::{Chunk, Op};
use crate::{
    frame::CallFrame,
    value::{KxMatrix, KxObj, KxVec, NativeFn, Value},
};

// ── Error ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Error)]
pub enum VmError {
    #[error("runtime error: {0}")]
    Runtime(String),
    #[error("stack overflow (call depth > {0})")]
    StackOverflow(usize),
    #[error("undefined variable: `{0}`")]
    UndefinedVariable(String),
    #[error("type error: expected {expected}, got {found}")]
    TypeError { expected: String, found: String },
    #[error("arithmetic error: {0}")]
    Arithmetic(String),
    #[error("index out of bounds: index {index} in length {len}")]
    IndexOutOfBounds { index: i64, len: usize },
    #[error("panic: {0}")]
    Panic(String),
}

pub type VmResult<T> = Result<T, VmError>;

// ── VM ────────────────────────────────────────────────────────────────────

const MAX_CALL_DEPTH: usize = 1000;

/// The Kinetix VM — executes compiled bytecode chunks.
pub struct Vm {
    /// Global variable store.
    globals: HashMap<String, Value>,
    /// Call stack.
    frames:  Vec<CallFrame>,
    /// Standard output sink (default: stdout).
    output:  Box<dyn std::io::Write + Send>,
}

impl Vm {
    pub fn new() -> Self {
        let mut vm = Self {
            globals: HashMap::new(),
            frames:  Vec::with_capacity(64),
            output:  Box::new(std::io::stdout()),
        };
        vm.register_builtins();
        vm
    }

    /// Redirect output (useful for testing).
    pub fn with_output(mut self, out: impl std::io::Write + Send + 'static) -> Self {
        self.output = Box::new(out);
        self
    }

    /// Register a native function as a global.
    pub fn register_native(&mut self, func: Arc<dyn NativeFn>) {
        self.globals.insert(func.name().to_owned(), Value::Native(func));
    }

    fn register_builtins(&mut self) {
        // println built-in
        #[derive(Debug)]
        struct Println;
        impl NativeFn for Println {
            fn call(&self, args: &[Value]) -> Result<Value, String> {
                let s = args.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ");
                println!("{s}");
                Ok(Value::Nil)
            }
            fn name(&self) -> &str { "println" }
        }

        #[derive(Debug)]
        struct Print;
        impl NativeFn for Print {
            fn call(&self, args: &[Value]) -> Result<Value, String> {
                let s = args.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ");
                print!("{s}");
                Ok(Value::Nil)
            }
            fn name(&self) -> &str { "print" }
        }

        #[derive(Debug)]
        struct Len;
        impl NativeFn for Len {
            fn call(&self, args: &[Value]) -> Result<Value, String> {
                match args.first() {
                    Some(Value::Vec(v))    => Ok(Value::Int(v.lock().unwrap().len() as i64)),
                    Some(Value::String(s)) => Ok(Value::Int(s.len() as i64)),
                    _ => Err("len() expects a vec or string".to_owned()),
                }
            }
            fn name(&self) -> &str { "len" }
            fn arity(&self) -> Option<usize> { Some(1) }
        }

        self.register_native(Arc::new(Println));
        self.register_native(Arc::new(Print));
        self.register_native(Arc::new(Len));
    }

    /// Execute a chunk (entry point).
    pub fn execute(&mut self, chunk: Arc<Chunk>) -> VmResult<Value> {
        let reg_count = chunk.register_count as usize;
        self.frames.push(CallFrame::new(chunk, reg_count, 0));
        self.run()
    }

    fn run(&mut self) -> VmResult<Value> {
        loop {
            let frame = self.frames.last_mut().ok_or_else(|| VmError::Runtime("empty call stack".into()))?;
            let instr = match frame.fetch() {
                Some(i) => i,
                None    => return Ok(Value::Nil),
            };

            let op_byte = instr.op();
            let dest  = instr.dest();
            let src1  = instr.src1();
            let src2  = instr.src2();

            // Decode opcode
            let op = match op_byte {
                0x00 => Op::LoadConst,
                0x01 => Op::LoadNil,
                0x02 => Op::LoadTrue,
                0x03 => Op::LoadFalse,
                0x04 => Op::LoadInt,
                0x10 => Op::Mov,
                0x20 => Op::Add,
                0x21 => Op::Sub,
                0x22 => Op::Mul,
                0x23 => Op::Div,
                0x24 => Op::Rem,
                0x25 => Op::Pow,
                0x26 => Op::Neg,
                0x30 => Op::Eq,
                0x31 => Op::Ne,
                0x32 => Op::Lt,
                0x33 => Op::Le,
                0x34 => Op::Gt,
                0x35 => Op::Ge,
                0x40 => Op::Not,
                0x41 => Op::And,
                0x42 => Op::Or,
                0x60 => Op::NewVec,
                0x61 => Op::VecPush,
                0x62 => Op::VecGet,
                0x70 => Op::LoadLocal,
                0x71 => Op::StoreLocal,
                0x78 => Op::LoadGlobal,
                0x79 => Op::StoreGlobal,
                0x80 => Op::Jump,
                0x81 => Op::JumpFalse,
                0x82 => Op::JumpTrue,
                0x90 => Op::Call,
                0x92 => Op::Return,
                0x93 => Op::ReturnNil,
                0x94 => Op::CallNative,
                0xD0 => Op::Print,
                0xE0 => Op::Breakpoint,
                0xF0 => Op::Panic,
                0xFF => Op::Nop,
                _    => { trace!("unknown opcode: 0x{op_byte:02X}"); continue; }
            };

            match op {
                Op::LoadNil   => { frame.set_reg(dest, Value::Nil); }
                Op::LoadTrue  => { frame.set_reg(dest, Value::Bool(true)); }
                Op::LoadFalse => { frame.set_reg(dest, Value::Bool(false)); }
                Op::LoadInt   => {
                    let v = instr.imm16() as i64;
                    frame.set_reg(dest, Value::Int(v));
                }
                Op::LoadConst => {
                    let c = frame.chunk.constants.get(src1 as usize)
                        .ok_or_else(|| VmError::Runtime("invalid constant index".into()))?;
                    use kinetix_bytecode::Constant;
                    let val = match c {
                        Constant::Int(v)    => Value::Int(*v),
                        Constant::Float(v)  => Value::Float(*v),
                        Constant::Bool(v)   => Value::Bool(*v),
                        Constant::String(s) => Value::String(Arc::new(s.clone())),
                        Constant::Nil       => Value::Nil,
                        Constant::FnRef(_)  => Value::Nil,
                    };
                    frame.set_reg(dest, val);
                }
                Op::Mov => {
                    let val = frame.reg(src1).clone();
                    frame.set_reg(dest, val);
                }
                Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Rem | Op::Pow => {
                    let a = frame.reg(src1).clone();
                    let b = frame.reg(src2).clone();
                    let result = self.arith_op(op, &a, &b)?;
                    self.frames.last_mut().unwrap().set_reg(dest, result);
                }
                Op::Neg => {
                    let a = frame.reg(src1).clone();
                    let result = match &a {
                        Value::Int(v)   => Value::Int(-v),
                        Value::Float(v) => Value::Float(-v),
                        _ => return Err(VmError::TypeError { expected: "int or float".into(), found: a.type_name().into() }),
                    };
                    frame.set_reg(dest, result);
                }
                Op::Eq | Op::Ne | Op::Lt | Op::Le | Op::Gt | Op::Ge => {
                    let a = frame.reg(src1).clone();
                    let b = frame.reg(src2).clone();
                    let result = self.cmp_op(op, &a, &b)?;
                    self.frames.last_mut().unwrap().set_reg(dest, Value::Bool(result));
                }
                Op::Not => {
                    let v = frame.reg(src1).is_truthy();
                    frame.set_reg(dest, Value::Bool(!v));
                }
                Op::And => {
                    let a = frame.reg(src1).is_truthy();
                    let b = frame.reg(src2).is_truthy();
                    frame.set_reg(dest, Value::Bool(a && b));
                }
                Op::Or => {
                    let a = frame.reg(src1).is_truthy();
                    let b = frame.reg(src2).is_truthy();
                    frame.set_reg(dest, Value::Bool(a || b));
                }
                Op::Jump => {
                    let offset = instr.imm16() as isize;
                    let frame = self.frames.last_mut().unwrap();
                    frame.ip = (frame.ip as isize + offset) as usize;
                }
                Op::JumpFalse => {
                    let cond = frame.reg(dest).is_truthy();
                    if !cond {
                        let offset = instr.imm16() as isize;
                        let frame = self.frames.last_mut().unwrap();
                        frame.ip = (frame.ip as isize + offset) as usize;
                    }
                }
                Op::JumpTrue => {
                    let cond = frame.reg(dest).is_truthy();
                    if cond {
                        let offset = instr.imm16() as isize;
                        let frame = self.frames.last_mut().unwrap();
                        frame.ip = (frame.ip as isize + offset) as usize;
                    }
                }
                Op::LoadGlobal => {
                    let name_const = src1 as usize;
                    let chunk = frame.chunk.clone();
                    let name = match chunk.constants.get(name_const) {
                        Some(kinetix_bytecode::Constant::String(s)) => s.clone(),
                        _ => return Err(VmError::Runtime("LOAD_GLOBAL expects string constant".into())),
                    };
                    let val = self.globals.get(&name)
                        .ok_or_else(|| VmError::UndefinedVariable(name.clone()))?
                        .clone();
                    self.frames.last_mut().unwrap().set_reg(dest, val);
                }
                Op::StoreGlobal => {
                    let name_const = src1 as usize;
                    let chunk = frame.chunk.clone();
                    let name = match chunk.constants.get(name_const) {
                        Some(kinetix_bytecode::Constant::String(s)) => s.clone(),
                        _ => return Err(VmError::Runtime("STORE_GLOBAL expects string constant".into())),
                    };
                    let val = frame.reg(dest).clone();
                    self.globals.insert(name, val);
                }
                Op::Print => {
                    let val = frame.reg(dest).clone();
                    writeln!(self.output, "{val}").ok();
                }
                Op::CallNative => {
                    // src1 = global name idx, src2 = argc, dest = result reg
                    let name_const = src1 as usize;
                    let argc = src2 as usize;
                    let chunk = frame.chunk.clone();
                    let name = match chunk.constants.get(name_const) {
                        Some(kinetix_bytecode::Constant::String(s)) => s.clone(),
                        _ => return Err(VmError::Runtime("CALL_NATIVE expects string constant".into())),
                    };
                    let func = match self.globals.get(&name) {
                        Some(Value::Native(f)) => f.clone(),
                        _ => return Err(VmError::UndefinedVariable(name)),
                    };
                    let frame = self.frames.last().unwrap();
                    let args: Vec<Value> = (0..argc).map(|i| frame.reg(i as u8).clone()).collect();
                    let result = func.call(&args).map_err(VmError::Runtime)?;
                    self.frames.last_mut().unwrap().set_reg(dest, result);
                }
                Op::Return => {
                    let val = frame.reg(dest).clone();
                    self.frames.pop();
                    if self.frames.is_empty() {
                        return Ok(val);
                    }
                    // Place return value in dest register of caller (simplified)
                    self.frames.last_mut().unwrap().set_reg(0, val);
                }
                Op::ReturnNil => {
                    self.frames.pop();
                    if self.frames.is_empty() {
                        return Ok(Value::Nil);
                    }
                    self.frames.last_mut().unwrap().set_reg(0, Value::Nil);
                }
                Op::Panic => {
                    let msg = frame.reg(dest).to_string();
                    return Err(VmError::Panic(msg));
                }
                Op::Breakpoint => {
                    // In debug mode, trigger the debugger. In release, NOP.
                    trace!("BREAKPOINT hit at ip={}", self.frames.last().unwrap().ip - 1);
                }
                Op::Nop => {}
                Op::NewVec => {
                    use std::sync::Mutex;
                    frame.set_reg(dest, Value::Vec(Arc::new(Mutex::new(KxVec::new()))));
                }
                Op::VecPush => {
                    let val = frame.reg(src2).clone();
                    if let Value::Vec(v) = frame.reg(src1).clone() {
                        v.lock().unwrap().push(val);
                    }
                }
                Op::VecGet => {
                    let idx = match frame.reg(src2) {
                        Value::Int(i) => *i,
                        _ => return Err(VmError::TypeError { expected: "int".into(), found: "other".into() }),
                    };
                    let result = if let Value::Vec(v) = frame.reg(src1).clone() {
                        let v = v.lock().unwrap();
                        v.get(idx as usize).cloned().unwrap_or(Value::Nil)
                    } else { Value::Nil };
                    frame.set_reg(dest, result);
                }
                _ => { trace!("unimplemented op: {:?}", op); }
            }
        }
    }

    fn arith_op(&self, op: Op, a: &Value, b: &Value) -> VmResult<Value> {
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => {
                Ok(match op {
                    Op::Add => Value::Int(x.wrapping_add(*y)),
                    Op::Sub => Value::Int(x.wrapping_sub(*y)),
                    Op::Mul => Value::Int(x.wrapping_mul(*y)),
                    Op::Div => if *y == 0 { return Err(VmError::Arithmetic("division by zero".into())); } else { Value::Int(x / y) }
                    Op::Rem => if *y == 0 { return Err(VmError::Arithmetic("modulo by zero".into())); } else { Value::Int(x % y) }
                    Op::Pow => Value::Int(x.wrapping_pow((*y).max(0) as u32)),
                    _ => unreachable!(),
                })
            }
            (Value::Float(_), Value::Float(_)) |
            (Value::Float(_), Value::Int(_)) |
            (Value::Int(_), Value::Float(_)) => {
                let x = a.as_float().unwrap();
                let y = b.as_float().unwrap();
                Ok(match op {
                    Op::Add => Value::Float(x + y),
                    Op::Sub => Value::Float(x - y),
                    Op::Mul => Value::Float(x * y),
                    Op::Div => Value::Float(x / y),
                    Op::Rem => Value::Float(x % y),
                    Op::Pow => Value::Float(x.powf(y)),
                    _ => unreachable!(),
                })
            }
            (Value::String(s1), Value::String(s2)) if op == Op::Add => {
                Ok(Value::String(Arc::new(format!("{s1}{s2}"))))
            }
            _ => Err(VmError::TypeError {
                expected: "numeric types".into(),
                found: format!("{} and {}", a.type_name(), b.type_name()),
            }),
        }
    }

    fn cmp_op(&self, op: Op, a: &Value, b: &Value) -> VmResult<bool> {
        Ok(match op {
            Op::Eq => a.equal(b),
            Op::Ne => !a.equal(b),
            Op::Lt => self.partial_ord(a, b)?.is_lt(),
            Op::Le => !self.partial_ord(a, b)?.is_gt(),
            Op::Gt => self.partial_ord(a, b)?.is_gt(),
            Op::Ge => !self.partial_ord(a, b)?.is_lt(),
            _ => unreachable!(),
        })
    }

    fn partial_ord(&self, a: &Value, b: &Value) -> VmResult<std::cmp::Ordering> {
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Ok(x.cmp(y)),
            (Value::Float(x), Value::Float(y)) => x.partial_cmp(y).ok_or_else(|| VmError::Arithmetic("NaN comparison".into())),
            (Value::Int(x), Value::Float(y)) => (*x as f64).partial_cmp(y).ok_or_else(|| VmError::Arithmetic("NaN comparison".into())),
            (Value::Float(x), Value::Int(y)) => x.partial_cmp(&(*y as f64)).ok_or_else(|| VmError::Arithmetic("NaN comparison".into())),
            (Value::String(a), Value::String(b)) => Ok(a.as_str().cmp(b.as_str())),
            _ => Err(VmError::TypeError { expected: "comparable types".into(), found: format!("{} and {}", a.type_name(), b.type_name()) }),
        }
    }
}

impl Default for Vm {
    fn default() -> Self { Self::new() }
}
