//! Document state management for the LSP.

use std::path::PathBuf;
use kinetix_ast::SourceFile;
use kinetix_parser::ParseError;

/// An open document in the language server.
pub struct Document {
    pub uri:     String,
    pub text:    String,
    pub version: i32,
    /// Last parse result (may be stale if document was changed).
    pub ast:     Option<SourceFile>,
    pub parse_errors: Vec<ParseError>,
}

impl Document {
    pub fn new(uri: String, text: String) -> Self {
        Self {
            uri,
            text,
            version: 0,
            ast: None,
            parse_errors: Vec::new(),
        }
    }

    /// Re-parse the document and update stored AST + errors.
    pub fn reparse(&mut self) {
        let (ast, errors) = kinetix_parser::parse_file(&self.text);
        self.ast = Some(ast);
        self.parse_errors = errors;
    }

    /// Apply an incremental text change.
    pub fn apply_change(&mut self, new_text: String, version: i32) {
        self.text = new_text;
        self.version = version;
        self.reparse();
    }

    /// Convert a byte offset to (line, column).
    pub fn offset_to_line_col(&self, offset: usize) -> (u32, u32) {
        let mut line = 0u32;
        let mut col  = 0u32;
        for (i, ch) in self.text.char_indices() {
            if i >= offset { break; }
            if ch == '\n' { line += 1; col = 0; }
            else          { col  += 1; }
        }
        (line, col)
    }
}
