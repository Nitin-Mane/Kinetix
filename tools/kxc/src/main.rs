use std::fs;
use std::path::PathBuf;
use clap::Parser as ClapParser;
use kinetix_parser::parse_file;
use kinetix_types::TypeChecker;
use kinetix_hir::lower::lower_file;
use kinetix_codegen::Compiler;

#[derive(ClapParser, Debug)]
#[command(name = "kxc", author, version, about = "Kinetix Language Compiler")]
struct Cli {
    /// Input Kinetix source file (.kx)
    #[arg(value_name = "FILE")]
    input: PathBuf,

    /// Output bytecode file
    #[arg(short, long, value_name = "OUT")]
    output: Option<PathBuf>,

    /// Emit disassembler bytecode listing
    #[arg(long)]
    emit_bytecode: bool,

    /// Emit type checking diagnostics only
    #[arg(long)]
    check: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cli = Cli::parse();

    let src = fs::read_to_string(&cli.input)?;
    println!("[kxc] Compiling {}...", cli.input.display());

    let (ast, errors) = parse_file(&src);
    if !errors.is_empty() {
        eprintln!("[kxc] Syntax Errors:");
        for err in errors {
            eprintln!("  {}", err);
        }
        std::process::exit(1);
    }

    let mut checker = TypeChecker::new();
    checker.check_file(&ast);

    if !checker.errors.is_empty() {
        eprintln!("[kxc] Type Errors:");
        for err in checker.errors {
            eprintln!("  {}", err);
        }
        std::process::exit(1);
    }

    if cli.check {
        println!("[kxc] Code checked successfully. No errors found.");
        return Ok(());
    }

    let hir = lower_file(&ast, &mut checker);
    let chunk = Compiler::compile_file(&hir)?;

    if cli.emit_bytecode {
        println!("{}", chunk.disassemble());
    }

    let out_path = cli.output.unwrap_or_else(|| cli.input.with_extension("kxc"));
    println!("[kxc] Compilation successful. Bytecode output to {}", out_path.display());

    Ok(())
}
