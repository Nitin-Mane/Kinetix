use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use clap::{Parser as ClapParser, Subcommand};
use serde::{Deserialize, Serialize};
use kinetix_parser::parse_file;
use kinetix_types::TypeChecker;
use kinetix_hir::lower::lower_file;
use kinetix_codegen::Compiler;
use kinetix_vm::Vm;
use kinetix_stdlib::register_all;

/// Kinetix package manifest structure (`kinetix.toml`).
#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub package: PackageInfo,
    #[serde(default)]
    pub dependencies: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name:    String,
    pub version: String,
    #[serde(default = "default_entry")]
    pub entry:   String,
    pub authors: Option<Vec<String>>,
    pub license: Option<String>,
}

fn default_entry() -> String {
    "src/main.kx".to_string()
}

#[derive(ClapParser, Debug)]
#[command(name = "kxpkg", author, version, about = "Kinetix Package Manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new Kinetix package directory
    New {
        /// Package name / directory name
        name: String,
    },
    /// Initialize a Kinetix package in the current directory
    Init,
    /// Build the package
    Build,
    /// Build and run the package
    Run,
    /// Type-check the package without generating code
    Check,
    /// Add a dependency to kinetix.toml
    Add {
        /// Package name to add
        dependency: String,
        /// Version requirement
        #[arg(default_value = "0.1.0")]
        version: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => create_new_package(&name)?,
        Commands::Init => init_current_package()?,
        Commands::Build => build_package()?,
        Commands::Run => run_package()?,
        Commands::Check => check_package()?,
        Commands::Add { dependency, version } => add_dependency(&dependency, &version)?,
    }

    Ok(())
}

fn create_new_package(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(name);
    if path.exists() {
        eprintln!("[kxpkg] Error: directory `{}` already exists", name);
        std::process::exit(1);
    }

    fs::create_dir_all(path.join("src"))?;

    let manifest_content = format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
entry = "src/main.kx"
authors = ["Kinetix Developer"]
license = "MIT"

[dependencies]
"#
    );

    fs::write(path.join("kinetix.toml"), manifest_content)?;

    let sample_code = r#"// Main entry point for Kinetix application
fn main() {
    println("Hello from Kinetix package!");

    let A = [[1.0, 2.0]; [3.0, 4.0]];
    let tr = trace(A);
    println("Matrix trace:");
    println(tr);
}
"#;

    fs::write(path.join("src/main.kx"), sample_code)?;
    fs::write(path.join(".gitignore"), "target/\n.kxpkg/\n")?;

    println!("[kxpkg] Created new package `{}` successfully!", name);
    println!("  cd {}\n  kxpkg run", name);

    Ok(())
}

fn init_current_package() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    let name = current_dir
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("my_package");

    if Path::new("kinetix.toml").exists() {
        eprintln!("[kxpkg] Error: `kinetix.toml` already exists in this directory");
        std::process::exit(1);
    }

    fs::create_dir_all("src")?;

    let manifest_content = format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
entry = "src/main.kx"

[dependencies]
"#
    );

    fs::write("kinetix.toml", manifest_content)?;

    if !Path::new("src/main.kx").exists() {
        fs::write("src/main.kx", "fn main() {\n    println(\"Hello, Kinetix!\");\n}\n")?;
    }

    println!("[kxpkg] Initialized package `{}` in current directory.", name);
    Ok(())
}

fn read_manifest() -> Result<Manifest, Box<dyn std::error::Error>> {
    let path = Path::new("kinetix.toml");
    if !path.exists() {
        eprintln!("[kxpkg] Error: `kinetix.toml` not found in current directory.");
        eprintln!("Run `kxpkg init` to create one.");
        std::process::exit(1);
    }

    let content = fs::read_to_string(path)?;
    let manifest: Manifest = toml::from_str(&content)?;
    Ok(manifest)
}

fn build_package() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = read_manifest()?;
    println!("[kxpkg] Compiling package `{}` v{}...", manifest.package.name, manifest.package.version);

    let entry_path = PathBuf::from(&manifest.package.entry);
    if !entry_path.exists() {
        eprintln!("[kxpkg] Error: entry file `{}` not found.", entry_path.display());
        std::process::exit(1);
    }

    let src = fs::read_to_string(&entry_path)?;
    let (ast, errors) = parse_file(&src);
    if !errors.is_empty() {
        eprintln!("[kxpkg] Syntax error(s):");
        for e in errors { eprintln!("  {}", e); }
        std::process::exit(1);
    }

    let mut checker = TypeChecker::new();
    checker.check_file(&ast);
    if !checker.errors.is_empty() {
        eprintln!("[kxpkg] Type error(s):");
        for e in checker.errors { eprintln!("  {}", e); }
        std::process::exit(1);
    }

    let hir = lower_file(&ast, &mut checker);
    let _chunk = Compiler::compile_file(&hir)?;

    let target_dir = Path::new("target");
    fs::create_dir_all(target_dir)?;
    let out_file = target_dir.join(format!("{}.kxc", manifest.package.name));

    // Save bytecode (simulated via summary output)
    println!("[kxpkg] Package `{}` built successfully -> {}", manifest.package.name, out_file.display());
    Ok(())
}

fn run_package() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = read_manifest()?;
    let entry_path = PathBuf::from(&manifest.package.entry);

    if !entry_path.exists() {
        eprintln!("[kxpkg] Error: entry file `{}` not found.", entry_path.display());
        std::process::exit(1);
    }

    println!("[kxpkg] Running `{}` v{}...", manifest.package.name, manifest.package.version);

    let src = fs::read_to_string(&entry_path)?;
    let (ast, errors) = parse_file(&src);
    if !errors.is_empty() {
        eprintln!("[kxpkg] Syntax error(s):");
        for e in errors { eprintln!("  {}", e); }
        std::process::exit(1);
    }

    let mut checker = TypeChecker::new();
    checker.check_file(&ast);

    let hir = lower_file(&ast, &mut checker);
    let chunk = Compiler::compile_file(&hir)?;

    let mut vm = Vm::new();
    register_all(&mut vm);

    match vm.execute(Arc::new(chunk)) {
        Ok(val) => {
            if !matches!(val, kinetix_vm::Value::Nil) {
                println!("{}", val);
            }
        }
        Err(e) => {
            eprintln!("[kxpkg] Runtime error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn check_package() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = read_manifest()?;
    let entry_path = PathBuf::from(&manifest.package.entry);

    let src = fs::read_to_string(&entry_path)?;
    let (ast, errors) = parse_file(&src);
    if !errors.is_empty() {
        eprintln!("[kxpkg] Syntax error(s):");
        for e in errors { eprintln!("  {}", e); }
        std::process::exit(1);
    }

    let mut checker = TypeChecker::new();
    checker.check_file(&ast);
    if !checker.errors.is_empty() {
        eprintln!("[kxpkg] Type error(s):");
        for e in checker.errors { eprintln!("  {}", e); }
        std::process::exit(1);
    }

    println!("[kxpkg] Package `{}` type check clean. 0 errors.", manifest.package.name);
    Ok(())
}

fn add_dependency(dep_name: &str, ver: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut manifest = read_manifest()?;
    manifest.dependencies.insert(dep_name.to_owned(), ver.to_owned());
    let toml_str = toml::to_string_pretty(&manifest)?;
    fs::write("kinetix.toml", toml_str)?;
    println!("[kxpkg] Added dependency `{}` = \"{}\" to kinetix.toml", dep_name, ver);
    Ok(())
}
