//! std::math — mathematical functions for Kinetix.

use std::sync::Arc;
use kinetix_vm::{NativeFn, Value, vm::Vm};

macro_rules! math_fn {
    ($name:literal, $f:expr) => {{
        #[derive(Debug)]
        struct F;
        impl NativeFn for F {
            fn name(&self) -> &str { $name }
            fn arity(&self) -> Option<usize> { Some(1) }
            fn call(&self, args: &[Value]) -> Result<Value, String> {
                let x = args.first()
                    .and_then(|v| v.as_float())
                    .ok_or_else(|| format!("{}: expected numeric argument", $name))?;
                Ok(Value::Float($f(x)))
            }
        }
        Arc::new(F) as Arc<dyn NativeFn>
    }};
}

macro_rules! math_fn2 {
    ($name:literal, $f:expr) => {{
        #[derive(Debug)]
        struct F;
        impl NativeFn for F {
            fn name(&self) -> &str { $name }
            fn arity(&self) -> Option<usize> { Some(2) }
            fn call(&self, args: &[Value]) -> Result<Value, String> {
                let a = args.get(0).and_then(|v| v.as_float())
                    .ok_or_else(|| format!("{}: expected numeric arg 1", $name))?;
                let b = args.get(1).and_then(|v| v.as_float())
                    .ok_or_else(|| format!("{}: expected numeric arg 2", $name))?;
                Ok(Value::Float($f(a, b)))
            }
        }
        Arc::new(F) as Arc<dyn NativeFn>
    }};
}

pub fn register(vm: &mut Vm) {
    // Trigonometric
    vm.register_native(math_fn!("sin",   f64::sin));
    vm.register_native(math_fn!("cos",   f64::cos));
    vm.register_native(math_fn!("tan",   f64::tan));
    vm.register_native(math_fn!("asin",  f64::asin));
    vm.register_native(math_fn!("acos",  f64::acos));
    vm.register_native(math_fn!("atan",  f64::atan));
    vm.register_native(math_fn2!("atan2", f64::atan2));

    // Exponential / logarithm
    vm.register_native(math_fn!("sqrt",  f64::sqrt));
    vm.register_native(math_fn!("cbrt",  f64::cbrt));
    vm.register_native(math_fn!("exp",   f64::exp));
    vm.register_native(math_fn!("ln",    f64::ln));
    vm.register_native(math_fn!("log2",  f64::log2));
    vm.register_native(math_fn!("log10", f64::log10));
    vm.register_native(math_fn2!("log",  f64::log));
    vm.register_native(math_fn2!("pow",  f64::powf));

    // Rounding
    vm.register_native(math_fn!("floor", f64::floor));
    vm.register_native(math_fn!("ceil",  f64::ceil));
    vm.register_native(math_fn!("round", f64::round));
    vm.register_native(math_fn!("trunc", f64::trunc));
    vm.register_native(math_fn!("abs",   f64::abs));
    vm.register_native(math_fn!("sign",  f64::signum));

    // Constants
    #[derive(Debug)] struct Pi;
    impl NativeFn for Pi {
        fn name(&self) -> &str { "PI" }
        fn arity(&self) -> Option<usize> { Some(0) }
        fn call(&self, _: &[Value]) -> Result<Value, String> { Ok(Value::Float(std::f64::consts::PI)) }
    }
    #[derive(Debug)] struct E;
    impl NativeFn for E {
        fn name(&self) -> &str { "E" }
        fn arity(&self) -> Option<usize> { Some(0) }
        fn call(&self, _: &[Value]) -> Result<Value, String> { Ok(Value::Float(std::f64::consts::E)) }
    }
    #[derive(Debug)] struct Tau;
    impl NativeFn for Tau {
        fn name(&self) -> &str { "TAU" }
        fn arity(&self) -> Option<usize> { Some(0) }
        fn call(&self, _: &[Value]) -> Result<Value, String> { Ok(Value::Float(std::f64::consts::TAU)) }
    }
    vm.register_native(Arc::new(Pi));
    vm.register_native(Arc::new(E));
    vm.register_native(Arc::new(Tau));

    // min / max
    #[derive(Debug)] struct Min;
    impl NativeFn for Min {
        fn name(&self) -> &str { "min" }
        fn arity(&self) -> Option<usize> { Some(2) }
        fn call(&self, args: &[Value]) -> Result<Value, String> {
            match (&args[0], &args[1]) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int((*a).min(*b))),
                _ => {
                    let a = args[0].as_float().ok_or("min: expected numbers")?;
                    let b = args[1].as_float().ok_or("min: expected numbers")?;
                    Ok(Value::Float(a.min(b)))
                }
            }
        }
    }
    #[derive(Debug)] struct Max;
    impl NativeFn for Max {
        fn name(&self) -> &str { "max" }
        fn arity(&self) -> Option<usize> { Some(2) }
        fn call(&self, args: &[Value]) -> Result<Value, String> {
            match (&args[0], &args[1]) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int((*a).max(*b))),
                _ => {
                    let a = args[0].as_float().ok_or("max: expected numbers")?;
                    let b = args[1].as_float().ok_or("max: expected numbers")?;
                    Ok(Value::Float(a.max(b)))
                }
            }
        }
    }
    vm.register_native(Arc::new(Min));
    vm.register_native(Arc::new(Max));
}
