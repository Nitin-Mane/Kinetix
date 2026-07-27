//! Editor state model for the Kinetix IDE.

use kinetix_syntax::{highlight, HighlightedToken};
use kinetix_parser::parse_file;

#[derive(Debug, Clone)]
pub struct DiagnosticItem {
    pub line:    u32,
    pub col:     u32,
    pub message: String,
}

pub struct EditorState {
    pub code:         String,
    pub file_path:    Option<String>,
    pub cursor_line:  usize,
    pub cursor_col:   usize,
    pub diagnostics: Vec<DiagnosticItem>,
}

impl EditorState {
    pub fn new(code: impl Into<String>) -> Self {
        let text = code.into();
        let mut state = Self {
            code: text,
            file_path: None,
            cursor_line: 1,
            cursor_col: 1,
            diagnostics: Vec::new(),
        };
        state.revalidate();
        state
    }

    pub fn set_code(&mut self, code: String) {
        self.code = code;
        self.revalidate();
    }

    pub fn revalidate(&mut self) {
        let (ast, errors) = parse_file(&self.code);
        let _ = ast;
        self.diagnostics = errors.into_iter().map(|e| {
            let span = e.span();
            let (line, col) = span_to_line_col(&self.code, span.start);
            DiagnosticItem {
                line,
                col,
                message: e.to_string(),
            }
        }).collect();
    }

    pub fn get_tokens(&self) -> Vec<HighlightedToken> {
        highlight(&self.code)
    }
}

fn span_to_line_col(text: &str, offset: usize) -> (u32, u32) {
    let mut line = 1u32;
    let mut col  = 1u32;
    for (i, ch) in text.char_indices() {
        if i >= offset { break; }
        if ch == '\n' { line += 1; col = 1; }
        else          { col  += 1; }
    }
    (line, col)
}
