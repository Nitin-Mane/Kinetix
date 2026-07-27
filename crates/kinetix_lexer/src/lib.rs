//! # kinetix_lexer
//!
//! Error-resilient tokenizer for the **Kinetix** programming language.
//!
//! Kinetix is a statically-typed, JIT-compiled language inspired by C++,
//! Rust, and MATLAB. This crate provides a streaming [`Lexer`] that produces
//! [`Token`]s from UTF-8 source text.
//!
//! On unrecognised input the lexer emits [`TokenKind::Unknown`] and
//! continues — it never panics, making it safe to use in IDE contexts
//! where source text may be partially broken.
//!
//! # Example
//! ```
//! use kinetix_lexer::{Lexer, TokenKind};
//!
//! let src = "fn main() { let x = 42 }";
//! let tokens: Vec<_> = Lexer::new(src)
//!     .filter(|t| !t.is_trivia())
//!     .collect();
//! assert_eq!(tokens[0].kind, TokenKind::Fn);
//! ```

pub mod lexer;
pub mod token;

pub use lexer::Lexer;
pub use token::{Span, Token, TokenKind, keyword_or_ident};
