//! std::matrix — MATLAB-inspired matrix operations for Kinetix.

use std::sync::{Arc, Mutex};
use kinetix_vm::{NativeFn, Value, KxMatrix, vm::Vm};

pub fn register(vm: &mut Vm) {
    // matrix::zeros(rows, cols) -> matrix<float>
    #[derive(Debug)] struct Zeros;
    impl NativeFn for Zeros {
        fn name(&self) -> &str { "zeros" }
        fn arity(&self) -> Option<usize> { Some(2) }
        fn call(&self, args: &[Value]) -> Result<Value, String> {
            let rows = args[0].as_int().ok_or("zeros: rows must be int")? as usize;
            let cols = args[1].as_int().ok_or("zeros: cols must be int")? as usize;
            Ok(Value::Matrix(Arc::new(Mutex::new(KxMatrix::zeros(rows, cols)))))
        }
    }

    // matrix::ones(rows, cols) -> matrix<float>
    #[derive(Debug)] struct Ones;
    impl NativeFn for Ones {
        fn name(&self) -> &str { "ones" }
        fn arity(&self) -> Option<usize> { Some(2) }
        fn call(&self, args: &[Value]) -> Result<Value, String> {
            let rows = args[0].as_int().ok_or("ones: rows must be int")? as usize;
            let cols = args[1].as_int().ok_or("ones: cols must be int")? as usize;
            let mut m = KxMatrix::zeros(rows, cols);
            for v in &mut m.data { *v = 1.0; }
            Ok(Value::Matrix(Arc::new(Mutex::new(m))))
        }
    }

    // matrix::eye(n) -> n×n identity matrix
    #[derive(Debug)] struct Eye;
    impl NativeFn for Eye {
        fn name(&self) -> &str { "eye" }
        fn arity(&self) -> Option<usize> { Some(1) }
        fn call(&self, args: &[Value]) -> Result<Value, String> {
            let n = args[0].as_int().ok_or("eye: n must be int")? as usize;
            let mut m = KxMatrix::zeros(n, n);
            for i in 0..n { m.set(i, i, 1.0); }
            Ok(Value::Matrix(Arc::new(Mutex::new(m))))
        }
    }

    // matrix::transpose(m) -> matrix<float>
    #[derive(Debug)] struct Transpose;
    impl NativeFn for Transpose {
        fn name(&self) -> &str { "transpose" }
        fn arity(&self) -> Option<usize> { Some(1) }
        fn call(&self, args: &[Value]) -> Result<Value, String> {
            if let Some(Value::Matrix(m)) = args.first() {
                let t = m.lock().unwrap().transpose();
                Ok(Value::Matrix(Arc::new(Mutex::new(t))))
            } else {
                Err("transpose: expected matrix".to_owned())
            }
        }
    }

    // matrix::matmul(a, b) -> matrix<float>
    #[derive(Debug)] struct Matmul;
    impl NativeFn for Matmul {
        fn name(&self) -> &str { "matmul" }
        fn arity(&self) -> Option<usize> { Some(2) }
        fn call(&self, args: &[Value]) -> Result<Value, String> {
            match (&args[0], &args[1]) {
                (Value::Matrix(a), Value::Matrix(b)) => {
                    let a = a.lock().unwrap();
                    let b = b.lock().unwrap();
                    let c = a.matmul(&b).ok_or("matmul: dimension mismatch")?;
                    Ok(Value::Matrix(Arc::new(Mutex::new(c))))
                }
                _ => Err("matmul: expected two matrices".to_owned()),
            }
        }
    }

    // matrix::trace(m) -> float
    #[derive(Debug)] struct Trace;
    impl NativeFn for Trace {
        fn name(&self) -> &str { "trace" }
        fn arity(&self) -> Option<usize> { Some(1) }
        fn call(&self, args: &[Value]) -> Result<Value, String> {
            if let Some(Value::Matrix(m)) = args.first() {
                Ok(Value::Float(m.lock().unwrap().trace()))
            } else {
                Err("trace: expected matrix".to_owned())
            }
        }
    }

    // linspace(start, end, n) -> vec<float>
    #[derive(Debug)] struct Linspace;
    impl NativeFn for Linspace {
        fn name(&self) -> &str { "linspace" }
        fn arity(&self) -> Option<usize> { Some(3) }
        fn call(&self, args: &[Value]) -> Result<Value, String> {
            let start = args[0].as_float().ok_or("linspace: start must be numeric")?;
            let end   = args[1].as_float().ok_or("linspace: end must be numeric")?;
            let n     = args[2].as_int().ok_or("linspace: n must be int")? as usize;
            if n < 2 { return Err("linspace: n must be >= 2".to_owned()); }
            let step = (end - start) / (n - 1) as f64;
            let values: Vec<Value> = (0..n).map(|i| Value::Float(start + step * i as f64)).collect();
            Ok(Value::Vec(Arc::new(std::sync::Mutex::new(kinetix_vm::KxVec::from_vec(values)))))
        }
    }

    vm.register_native(Arc::new(Zeros));
    vm.register_native(Arc::new(Ones));
    vm.register_native(Arc::new(Eye));
    vm.register_native(Arc::new(Transpose));
    vm.register_native(Arc::new(Matmul));
    vm.register_native(Arc::new(Trace));
    vm.register_native(Arc::new(Linspace));
}
