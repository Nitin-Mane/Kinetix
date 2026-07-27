//! Diagnostic conversion (parse errors → LSP diagnostics).

use kinetix_parser::ParseError;
use kinetix_lexer::Span;

/// An LSP-compatible diagnostic (simplified).
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub start_line: u32,
    pub start_col:  u32,
    pub end_line:   u32,
    pub end_col:    u32,
    pub message:    String,
    pub severity:   Severity,
}

#[derive(Debug, Clone, Copy)]
pub enum Severity { Error, Warning, Info, Hint }

impl Diagnostic {
    pub fn from_parse_error(err: &ParseError, text: &str) -> Self {
        let span = err.span();
        let (start_line, start_col) = span_to_line_col(text, span.start);
        let (end_line, end_col) = span_to_line_col(text, span.end.max(span.start));
        Self {
            start_line,
            start_col,
            end_line,
            end_col,
            message:  err.to_string(),
            severity: Severity::Error,
        }
    }
}

pub fn span_to_line_col(text: &str, offset: usize) -> (u32, u32) {
    let mut line = 0u32;
    let mut col  = 0u32;
    for (i, ch) in text.char_indices() {
        if i >= offset { break; }
        if ch == '\n' { line += 1; col = 0; }
        else          { col  += 1; }
    }
    (line, col)
}
