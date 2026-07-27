//! # kinetix_parser
//!
//! Recursive-descent / Pratt parser that turns a stream of [`kinetix_lexer::Token`]s
//! into a [`kinetix_ast::SourceFile`] AST.
//!
//! The parser is **error-recovering**: on a syntax error it emits a
//! [`ParseError`] into a list and attempts to resynchronise so that
//! it can continue parsing the rest of the file. This is crucial for IDE
//! use-cases where the file is always partially broken.
//!
//! # Entry points
//! ```no_run
//! use kinetix_parser::Parser;
//!
//! let src = r#"fn main() { let x = 42 }"#;
//! let (ast, errors) = Parser::new(src).parse_file();
//! ```

pub mod error;
pub mod parser;
pub mod pratt;

pub use error::ParseError;
pub use parser::Parser;

use kinetix_ast::SourceFile;

/// Parse a complete Kinetix source file.
///
/// Returns the AST (which may be incomplete on errors) and a list of
/// parse errors encountered.  An empty error list means the parse was
/// successful.
pub fn parse_file(src: &str) -> (SourceFile, Vec<ParseError>) {
    Parser::new(src).parse_file()
}

/// Parse a single expression — useful for REPL and scripting contexts.
pub fn parse_expr(src: &str) -> (Option<kinetix_ast::Expr>, Vec<ParseError>) {
    Parser::new(src).parse_single_expr()
}
