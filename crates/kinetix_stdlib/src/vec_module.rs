//! std::vec — Vec operations for Kinetix.

use std::sync::{Arc, Mutex};
use kinetix_vm::{KxVec, NativeFn, Value, vm::Vm};

pub fn register(vm: &mut Vm) {
    // len(v) -> int
    #[derive(Debug)] struct Len;
    impl NativeFn for Len {
        fn name(&self) -> &str { "len" }
        fn arity(&self) -> Option<usize> { Some(1) }
        fn call(&self, args: &[Value]) -> Result<Value, String> {
            match args.first() {
                Some(Value::Vec(v))    => Ok(Value::Int(v.lock().unwrap().len() as i64)),
                Some(Value::String(s)) => Ok(Value::Int(s.len() as i64)),
                Some(Value::Matrix(m)) => {
                    let m = m.lock().unwrap();
                    Ok(Value::Int((m.rows * m.cols) as i64))
                }
                _ => Err("len: unsupported type".to_owned()),
            }
        }
    }

    // push(v, elem) — mutates vec in-place
    #[derive(Debug)] struct Push;
    impl NativeFn for Push {
        fn name(&self) -> &str { "push" }
        fn arity(&self) -> Option<usize> { Some(2) }
        fn call(&self, args: &[Value]) -> Result<Value, String> {
            if let (Some(Value::Vec(v)), Some(elem)) = (args.get(0), args.get(1)) {
                v.lock().unwrap().push(elem.clone());
                Ok(Value::Nil)
            } else {
                Err("push: expected (vec, value)".to_owned())
            }
        }
    }

    // pop(v) -> value | nil
    #[derive(Debug)] struct Pop;
    impl NativeFn for Pop {
        fn name(&self) -> &str { "pop" }
        fn arity(&self) -> Option<usize> { Some(1) }
        fn call(&self, args: &[Value]) -> Result<Value, String> {
            if let Some(Value::Vec(v)) = args.first() {
                Ok(v.lock().unwrap().elements.pop().unwrap_or(Value::Nil))
            } else {
                Err("pop: expected vec".to_owned())
            }
        }
    }

    // range(start, end) -> vec<int>
    #[derive(Debug)] struct Range;
    impl NativeFn for Range {
        fn name(&self) -> &str { "range" }
        fn arity(&self) -> Option<usize> { Some(2) }
        fn call(&self, args: &[Value]) -> Result<Value, String> {
            let start = args[0].as_int().ok_or("range: start must be int")?;
            let end   = args[1].as_int().ok_or("range: end must be int")?;
            let elems: Vec<Value> = (start..end).map(Value::Int).collect();
            Ok(Value::Vec(Arc::new(Mutex::new(KxVec::from_vec(elems)))))
        }
    }

    // sum(v) -> int | float
    #[derive(Debug)] struct Sum;
    impl NativeFn for Sum {
        fn name(&self) -> &str { "sum" }
        fn arity(&self) -> Option<usize> { Some(1) }
        fn call(&self, args: &[Value]) -> Result<Value, String> {
            if let Some(Value::Vec(v)) = args.first() {
                let v = v.lock().unwrap();
                let mut total_f = 0.0f64;
                let mut is_float = false;
                let mut total_i = 0i64;
                for elem in &v.elements {
                    match elem {
                        Value::Int(x)   => total_i = total_i.wrapping_add(*x),
                        Value::Float(x) => { total_f += x; is_float = true; }
                        _ => return Err("sum: vec must contain numbers".to_owned()),
                    }
                }
                if is_float { Ok(Value::Float(total_f + total_i as f64)) }
                else        { Ok(Value::Int(total_i)) }
            } else {
                Err("sum: expected vec".to_owned())
            }
        }
    }

    vm.register_native(Arc::new(Len));
    vm.register_native(Arc::new(Push));
    vm.register_native(Arc::new(Pop));
    vm.register_native(Arc::new(Range));
    vm.register_native(Arc::new(Sum));
}
