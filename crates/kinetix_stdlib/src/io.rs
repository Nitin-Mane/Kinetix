//! std::io — console and file I/O for Kinetix.

use std::sync::Arc;
use kinetix_vm::{NativeFn, Value, vm::Vm};

macro_rules! native {
    ($name:literal, $arity:expr, |$args:ident| $body:expr) => {{
        #[derive(Debug)]
        struct F;
        impl NativeFn for F {
            fn name(&self) -> &str { $name }
            fn arity(&self) -> Option<usize> { $arity }
            fn call(&self, $args: &[Value]) -> Result<Value, String> { $body }
        }
        Arc::new(F) as Arc<dyn NativeFn>
    }};
}

pub fn register(vm: &mut Vm) {
    // io::println(value) — print with newline
    vm.register_native(native!("println", None, |args| {
        let s = args.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ");
        println!("{s}");
        Ok(Value::Nil)
    }));

    // io::print(value) — print without newline
    vm.register_native(native!("print", None, |args| {
        let s = args.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ");
        print!("{s}");
        Ok(Value::Nil)
    }));

    // io::read_line() — read a line from stdin
    vm.register_native(native!("read_line", Some(0), |_args| {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)
            .map_err(|e| e.to_string())?;
        Ok(Value::String(Arc::new(line.trim_end_matches('\n').to_owned())))
    }));

    // io::eprintln(value) — print to stderr
    vm.register_native(native!("eprintln", None, |args| {
        let s = args.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ");
        eprintln!("{s}");
        Ok(Value::Nil)
    }));
}
