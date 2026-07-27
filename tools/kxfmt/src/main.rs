use std::fs;
use std::path::PathBuf;
use clap::Parser as ClapParser;
use kinetix_lexer::{Lexer, TokenKind};

#[derive(ClapParser, Debug)]
#[command(name = "kxfmt", author, version, about = "Kinetix Code Formatter")]
struct Cli {
    /// Input Kinetix source file(s) to format
    #[arg(value_name = "FILES", required = true)]
    files: Vec<PathBuf>,

    /// Write changes directly to file(s) in place
    #[arg(short, long)]
    write: bool,

    /// Check if files are formatted without editing
    #[arg(long)]
    check: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    for path in &cli.files {
        let content = fs::read_to_string(path)?;
        let formatted = format_source(&content);

        if cli.check {
            if content != formatted {
                eprintln!("[kxfmt] Code style error in {}", path.display());
                std::process::exit(1);
            } else {
                println!("[kxfmt] {} is formatted correctly.", path.display());
            }
        } else if cli.write {
            fs::write(path, &formatted)?;
            println!("[kxfmt] Formatted {}", path.display());
        } else {
            print!("{}", formatted);
        }
    }

    Ok(())
}

fn format_source(src: &str) -> String {
    let mut out = String::new();
    let mut indent_level = 0usize;
    let mut at_line_start = true;

    for tok in Lexer::new(src) {
        let token_text = &src[tok.span.start..tok.span.end.min(src.len())];

        match &tok.kind {
            TokenKind::LBrace => {
                if !out.ends_with(' ') && !at_line_start {
                    out.push(' ');
                }
                out.push('{');
                indent_level += 1;
                out.push('\n');
                at_line_start = true;
            }
            TokenKind::RBrace => {
                indent_level = indent_level.saturating_sub(1);
                if !at_line_start {
                    out.push('\n');
                }
                out.push_str(&"    ".repeat(indent_level));
                out.push('}');
                out.push('\n');
                at_line_start = true;
            }
            TokenKind::Semicolon => {
                out.push(';');
                out.push('\n');
                at_line_start = true;
            }
            TokenKind::Whitespace => {
                if token_text.contains('\n') {
                    // Normalize newlines
                    if !out.ends_with('\n') {
                        out.push('\n');
                        at_line_start = true;
                    }
                } else if !at_line_start && !out.ends_with(' ') {
                    out.push(' ');
                }
            }
            _ => {
                if at_line_start {
                    out.push_str(&"    ".repeat(indent_level));
                    at_line_start = false;
                }
                out.push_str(token_text);
            }
        }
    }

    if !out.ends_with('\n') {
        out.push('\n');
    }

    out
}
