//! Interactive REPL engine for Kinetix IDE and CLI tools.

use kinetix_parser::parse_file;
use kinetix_types::TypeChecker;
use kinetix_hir::lower::lower_file;
use kinetix_codegen::Compiler;
use kinetix_vm::{Vm, Value, VmError};
use kinetix_stdlib::register_all;

pub struct ReplSession {
    vm: Vm,
    type_checker: TypeChecker,
}

impl ReplSession {
    pub fn new() -> Self {
        let mut vm = Vm::new();
        register_all(&mut vm);
        Self {
            vm,
            type_checker: TypeChecker::new(),
        }
    }

    pub fn eval(&mut self, input: &str) -> Result<Value, String> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Ok(Value::Nil);
        }

        // Wrap raw expression into fn main() if not already containing top-level items
        let code = if !trimmed.contains("fn ") {
            format!("fn main() {{ {trimmed} }}")
        } else {
            trimmed.to_owned()
        };

        let (ast, errors) = parse_file(&code);
        if !errors.is_empty() {
            let err_msgs: Vec<_> = errors.iter().map(|e| e.to_string()).collect();
            return Err(format!("Parse error: {}", err_msgs.join(", ")));
        }

        self.type_checker.check_file(&ast);
        let hir = lower_file(&ast, &mut self.type_checker);

        let chunk = Compiler::compile_file(&hir)
            .map_err(|e| format!("Codegen error: {e}"))?;

        self.vm.execute(std::sync::Arc::new(chunk))
            .map_err(|e| format!("Runtime error: {e}"))
    }
}

impl Default for ReplSession {
    fn default() -> Self { Self::new() }
}
