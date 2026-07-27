//! # kinetix_lexer
//!
//! Tokenizer (lexer) for the **Kinetix** programming language.
//!
//! Kinetix is a dynamically-typed scripting language inspired by C++, Rust,
//! and MATLAB. This crate provides an error-resilient lexer that produces
//! a flat stream of [`Token`]s from a source string.
//!
//! ## Example
//!
//! ```rust
//! use kinetix_lexer::Lexer;
//!
//! let src = r#"fn main() { let x = 42 }"#;
//! let tokens: Vec<_> = Lexer::new(src).collect();
//! println!("{tokens:#?}");
//! ```

pub mod lexer;
pub mod token;

pub use lexer::Lexer;
pub use token::{Token, TokenKind, Span};
