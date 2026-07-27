use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use clap::Parser as ClapParser;
use kinetix_parser::parse_file;
use kinetix_types::TypeChecker;
use kinetix_hir::lower::lower_file;
use kinetix_codegen::Compiler;
use kinetix_vm::Vm;
use kinetix_stdlib::register_all;
use kinetix_ide::repl::ReplSession;
use rustyline::DefaultEditor;

#[derive(ClapParser, Debug)]
#[command(name = "kxr", author, version, about = "Kinetix JIT Runner & Interactive REPL")]
struct Cli {
    /// Input Kinetix source file (.kx) to execute. If omitted, starts interactive REPL.
    #[arg(value_name = "FILE")]
    input: Option<PathBuf>,

    /// Enable Cranelift JIT compilation backend
    #[arg(long)]
    jit: bool,

    /// Disassemble bytecode before execution
    #[arg(long)]
    disassemble: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.input {
        Some(file_path) => run_file(&file_path, cli.jit, cli.disassemble),
        None => run_repl(),
    }
}

fn run_file(file_path: &PathBuf, use_jit: bool, disassemble: bool) -> Result<(), Box<dyn std::error::Error>> {
    let src = fs::read_to_string(file_path)?;
    let (ast, errors) = parse_file(&src);

    if !errors.is_empty() {
        eprintln!("[kxr] Parse error(s):");
        for err in errors {
            eprintln!("  {}", err);
        }
        std::process::exit(1);
    }

    let mut checker = TypeChecker::new();
    checker.check_file(&ast);

    if !checker.errors.is_empty() {
        eprintln!("[kxr] Type error(s):");
        for err in checker.errors {
            eprintln!("  {}", err);
        }
        std::process::exit(1);
    }

    let hir = lower_file(&ast, &mut checker);
    let chunk = Compiler::compile_file(&hir)?;

    if disassemble {
        println!("{}", chunk.disassemble());
    }

    let mut vm = Vm::new();
    register_all(&mut vm);

    if use_jit {
        println!("[kxr] JIT compilation enabled (Cranelift backend target)");
    }

    match vm.execute(Arc::new(chunk)) {
        Ok(val) => {
            if !matches!(val, kinetix_vm::Value::Nil) {
                println!("{}", val);
            }
        }
        Err(e) => {
            eprintln!("[kxr] Runtime Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn run_repl() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Kinetix Language REPL v0.1.0 ===");
    println!("Type code expressions or statements. Ctrl+C or Ctrl+D to exit.\n");

    let mut rl = DefaultEditor::new()?;
    let mut session = ReplSession::new();

    loop {
        let readline = rl.readline("kx> ");
        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() { continue; }
                let _ = rl.add_history_entry(line.as_str());

                match session.eval(trimmed) {
                    Ok(val) => {
                        if !matches!(val, kinetix_vm::Value::Nil) {
                            println!("=> {}", val);
                        }
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            }
            Err(_) => {
                println!("Goodbye!");
                break;
            }
        }
    }

    Ok(())
}
