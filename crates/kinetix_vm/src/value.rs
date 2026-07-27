//! Runtime value types for the Kinetix VM.
//!
//! All runtime values use tagged-union representation.
//! Reference types (Vec, Matrix, Object, String, Closure) are heap-allocated
//! and reference-counted (Arc<T>).

use std::sync::Arc;
use std::fmt;
use std::collections::HashMap;
use kinetix_bytecode::Chunk;

// ── Value ─────────────────────────────────────────────────────────────────

/// A runtime Kinetix value.
#[derive(Clone, Debug)]
pub enum Value {
    // Primitive (stack-allocated)
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),

    // Heap-allocated (reference-counted)
    String(Arc<String>),
    Vec(Arc<std::sync::Mutex<KxVec>>),
    Matrix(Arc<std::sync::Mutex<KxMatrix>>),
    Object(Arc<std::sync::Mutex<KxObj>>),
    Closure(Arc<KxClosure>),

    /// A native Rust function callable from Kinetix.
    Native(Arc<dyn NativeFn>),
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Nil        => "nil",
            Value::Bool(_)    => "bool",
            Value::Int(_)     => "int",
            Value::Float(_)   => "float",
            Value::String(_)  => "string",
            Value::Vec(_)     => "vec",
            Value::Matrix(_)  => "matrix",
            Value::Object(_)  => "object",
            Value::Closure(_) => "closure",
            Value::Native(_)  => "native_fn",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil        => false,
            Value::Bool(b)    => *b,
            Value::Int(v)     => *v != 0,
            Value::Float(v)   => *v != 0.0,
            Value::String(s)  => !s.is_empty(),
            _                 => true,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(v)   => Some(*v),
            Value::Float(v) => Some(*v as i64),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(v) => Some(*v),
            Value::Int(v)   => Some(*v as f64),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self { Value::Bool(b) => Some(*b), _ => None }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self { Value::String(s) => Some(s.as_str()), _ => None }
    }

    /// Equality comparison (structural, not reference).
    pub fn equal(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil)           => true,
            (Value::Bool(a), Value::Bool(b))   => a == b,
            (Value::Int(a), Value::Int(b))     => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Int(a), Value::Float(b))   => (*a as f64) == *b,
            (Value::Float(a), Value::Int(b))   => *a == (*b as f64),
            (Value::String(a), Value::String(b)) => a == b,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil        => write!(f, "nil"),
            Value::Bool(b)    => write!(f, "{b}"),
            Value::Int(v)     => write!(f, "{v}"),
            Value::Float(v)   => write!(f, "{v}"),
            Value::String(s)  => write!(f, "{s}"),
            Value::Vec(v)     => {
                let v = v.lock().unwrap();
                write!(f, "[")?;
                for (i, elem) in v.elements.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{elem}")?;
                }
                write!(f, "]")
            }
            Value::Matrix(m)  => {
                let m = m.lock().unwrap();
                write!(f, "matrix({}x{})", m.rows, m.cols)
            }
            Value::Object(o)  => {
                let o = o.lock().unwrap();
                write!(f, "{{{}}}", o.type_name)
            }
            Value::Closure(c) => write!(f, "<closure '{}'>", c.chunk.name),
            Value::Native(_)  => write!(f, "<native fn>"),
        }
    }
}

// ── Heap types ────────────────────────────────────────────────────────────

/// A Kinetix runtime vector (growable array).
#[derive(Debug, Clone)]
pub struct KxVec {
    pub elements: Vec<Value>,
}

impl KxVec {
    pub fn new() -> Self { Self { elements: Vec::new() } }
    pub fn from_vec(v: Vec<Value>) -> Self { Self { elements: v } }
    pub fn len(&self) -> usize { self.elements.len() }
    pub fn push(&mut self, v: Value) { self.elements.push(v); }
    pub fn get(&self, idx: usize) -> Option<&Value> { self.elements.get(idx) }
    pub fn set(&mut self, idx: usize, v: Value) -> bool {
        if idx < self.elements.len() { self.elements[idx] = v; true } else { false }
    }
}

/// A Kinetix runtime matrix (2D numeric array, row-major).
#[derive(Debug, Clone)]
pub struct KxMatrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,   // row-major f64 storage
}

impl KxMatrix {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self { rows, cols, data: vec![0.0; rows * cols] }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        if row < self.rows && col < self.cols {
            Some(self.data[row * self.cols + col])
        } else { None }
    }

    pub fn set(&mut self, row: usize, col: usize, v: f64) -> bool {
        if row < self.rows && col < self.cols {
            self.data[row * self.cols + col] = v;
            true
        } else { false }
    }

    /// Matrix transpose.
    pub fn transpose(&self) -> Self {
        let mut out = Self::zeros(self.cols, self.rows);
        for r in 0..self.rows {
            for c in 0..self.cols {
                out.set(c, r, self.get(r, c).unwrap_or(0.0));
            }
        }
        out
    }

    /// Element-wise addition.
    pub fn add(&self, other: &KxMatrix) -> Option<KxMatrix> {
        if self.rows != other.rows || self.cols != other.cols { return None; }
        let data = self.data.iter().zip(&other.data).map(|(a, b)| a + b).collect();
        Some(KxMatrix { rows: self.rows, cols: self.cols, data })
    }

    /// Matrix multiply.
    pub fn matmul(&self, other: &KxMatrix) -> Option<KxMatrix> {
        if self.cols != other.rows { return None; }
        let mut out = KxMatrix::zeros(self.rows, other.cols);
        for r in 0..self.rows {
            for c in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.get(r, k).unwrap_or(0.0) * other.get(k, c).unwrap_or(0.0);
                }
                out.set(r, c, sum);
            }
        }
        Some(out)
    }

    /// Trace (sum of diagonal).
    pub fn trace(&self) -> f64 {
        let n = self.rows.min(self.cols);
        (0..n).map(|i| self.get(i, i).unwrap_or(0.0)).sum()
    }
}

/// A Kinetix struct instance.
#[derive(Debug, Clone)]
pub struct KxObj {
    pub type_name: String,
    pub fields:    HashMap<String, Value>,
}

impl KxObj {
    pub fn new(type_name: impl Into<String>) -> Self {
        Self { type_name: type_name.into(), fields: HashMap::new() }
    }
}

/// A Kinetix closure (function + captured upvalues).
#[derive(Debug)]
pub struct KxClosure {
    pub chunk:    Arc<Chunk>,
    pub upvalues: Vec<Value>,
}

// ── Native function trait ─────────────────────────────────────────────────

pub trait NativeFn: Send + Sync + fmt::Debug {
    fn call(&self, args: &[Value]) -> Result<Value, String>;
    fn name(&self) -> &str;
    fn arity(&self) -> Option<usize> { None }  // None = variadic
}
