//! # kinetix_syntax
//!
//! Token-based syntax highlighting for the Kinetix language.
//!
//! Produces a stream of [`HighlightedToken`]s from Kinetix source code.
//! These can be used by the IDE (kinetix_ide), the LSP server, and
//! documentation generators.

use kinetix_lexer::{Lexer, Token, TokenKind};

/// Semantic highlight category for a token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenClass {
    Keyword,
    TypeKeyword,
    Identifier,
    TypeName,
    FunctionName,
    Number,
    String,
    Comment,
    Operator,
    Punctuation,
    Literal,
    Macro,
    Unknown,
}

impl TokenClass {
    /// Returns a CSS-friendly class name.
    pub fn css_class(self) -> &'static str {
        match self {
            TokenClass::Keyword      => "kx-keyword",
            TokenClass::TypeKeyword  => "kx-type-kw",
            TokenClass::Identifier   => "kx-ident",
            TokenClass::TypeName     => "kx-type-name",
            TokenClass::FunctionName => "kx-fn-name",
            TokenClass::Number       => "kx-number",
            TokenClass::String       => "kx-string",
            TokenClass::Comment      => "kx-comment",
            TokenClass::Operator     => "kx-operator",
            TokenClass::Punctuation  => "kx-punct",
            TokenClass::Literal      => "kx-literal",
            TokenClass::Macro        => "kx-macro",
            TokenClass::Unknown      => "kx-unknown",
        }
    }

    /// Returns an LSP SemanticTokenType index.
    pub fn lsp_token_type(self) -> u32 {
        match self {
            TokenClass::Keyword | TokenClass::TypeKeyword => 0,
            TokenClass::TypeName                          => 1,
            TokenClass::FunctionName                      => 2,
            TokenClass::Identifier                        => 3,
            TokenClass::Number                            => 4,
            TokenClass::String                            => 5,
            TokenClass::Comment                           => 6,
            TokenClass::Operator                          => 7,
            _                                             => 8,
        }
    }
}

/// A highlighted token with its source span and class.
#[derive(Debug, Clone)]
pub struct HighlightedToken {
    pub start:     usize,
    pub end:       usize,
    pub text:      String,
    pub class:     TokenClass,
}

/// Highlight a Kinetix source string.
///
/// Returns a list of highlighted tokens in source order.
pub fn highlight(src: &str) -> Vec<HighlightedToken> {
    let mut tokens = Vec::new();
    let mut prev_was_fn = false;

    for tok in Lexer::new(src) {
        let class = classify_token(&tok, prev_was_fn);
        prev_was_fn = matches!(tok.kind, TokenKind::Fn);

        let text = &src[tok.span.start..tok.span.end.min(src.len())];
        tokens.push(HighlightedToken {
            start: tok.span.start,
            end:   tok.span.end.min(src.len()),
            text:  text.to_owned(),
            class,
        });
    }

    tokens
}

fn classify_token(tok: &Token, prev_was_fn: bool) -> TokenClass {
    match &tok.kind {
        // Keywords
        TokenKind::Fn       | TokenKind::Let      | TokenKind::Mut     |
        TokenKind::If       | TokenKind::Else     | TokenKind::Elif    |
        TokenKind::Match    | TokenKind::Return    | TokenKind::Loop    |
        TokenKind::While    | TokenKind::For       | TokenKind::Break   |
        TokenKind::Continue | TokenKind::Struct    | TokenKind::Enum    |
        TokenKind::Trait    | TokenKind::Impl      | TokenKind::Import  |
        TokenKind::Export   | TokenKind::Const     | TokenKind::Mod     |
        TokenKind::Await    | TokenKind::Spawn     | TokenKind::True    |
        TokenKind::False    | TokenKind::Nil       => TokenClass::Keyword,

        // Type keywords
        TokenKind::TyInt    | TokenKind::TyFloat   | TokenKind::TyBool  |
        TokenKind::TyString | TokenKind::TyNil     | TokenKind::TyVec   |
        TokenKind::TyMap    | TokenKind::TyMatrix              => TokenClass::TypeKeyword,

        // Literals
        TokenKind::IntLiteral(_) | TokenKind::FloatLiteral(_)  => TokenClass::Number,
        TokenKind::StringLiteral(_)                            => TokenClass::String,

        // Comments
        TokenKind::LineComment(_) | TokenKind::BlockComment(_) => TokenClass::Comment,

        // Identifiers — contextual classification
        TokenKind::Ident(name) => {
            if prev_was_fn {
                TokenClass::FunctionName
            } else if name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                TokenClass::TypeName
            } else {
                TokenClass::Identifier
            }
        }

        // Operators
        TokenKind::Plus    | TokenKind::Minus   | TokenKind::Star   |
        TokenKind::Slash   | TokenKind::Percent | TokenKind::Amp    |
        TokenKind::Pipe    | TokenKind::Caret   | TokenKind::Bang   |
        TokenKind::Tilde   | TokenKind::Lt      | TokenKind::Gt     |
        TokenKind::LtEq    | TokenKind::GtEq    | TokenKind::EqEq   |
        TokenKind::BangEq  | TokenKind::AmpAmp  | TokenKind::PipePipe |
        TokenKind::StarStar | TokenKind::Arrow  | TokenKind::FatArrow |
        TokenKind::DotDot  | TokenKind::DotDotEq                    => TokenClass::Operator,

        TokenKind::Eq | TokenKind::PlusEq | TokenKind::MinusEq |
        TokenKind::StarEq | TokenKind::SlashEq | TokenKind::PercentEq => TokenClass::Operator,

        // Punctuation
        TokenKind::LParen | TokenKind::RParen | TokenKind::LBrace |
        TokenKind::RBrace | TokenKind::LBracket | TokenKind::RBracket |
        TokenKind::Comma  | TokenKind::Semicolon | TokenKind::Colon  |
        TokenKind::ColonColon | TokenKind::Dot | TokenKind::Question  => TokenClass::Punctuation,

        _ => TokenClass::Unknown,
    }
}

/// Generate HTML with inline syntax highlighting from Kinetix source.
pub fn to_html(src: &str) -> String {
    let tokens = highlight(src);
    let mut out = String::from("<pre class=\"kx-code\">");
    let mut prev_end = 0;

    for tok in &tokens {
        if tok.start > prev_end {
            // Unhighlighted gap (should not happen in normal code, but handle it)
            let gap = &src[prev_end..tok.start];
            out += &html_escape(gap);
        }
        out += &format!("<span class=\"{}\">{}</span>",
            tok.class.css_class(),
            html_escape(&tok.text)
        );
        prev_end = tok.end;
    }

    if prev_end < src.len() {
        out += &html_escape(&src[prev_end..]);
    }

    out += "</pre>";
    out
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
}
