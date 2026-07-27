//! Language server stub.

use std::collections::HashMap;
use crate::document::Document;

/// The Kinetix Language Server.
pub struct LspServer {
    documents: HashMap<String, Document>,
}

impl LspServer {
    pub fn new() -> Self {
        Self { documents: HashMap::new() }
    }

    pub fn open_document(&mut self, uri: String, text: String) {
        let mut doc = Document::new(uri.clone(), text);
        doc.reparse();
        self.documents.insert(uri, doc);
    }

    pub fn update_document(&mut self, uri: &str, text: String, version: i32) {
        if let Some(doc) = self.documents.get_mut(uri) {
            doc.apply_change(text, version);
        }
    }

    pub fn close_document(&mut self, uri: &str) {
        self.documents.remove(uri);
    }

    pub fn get_diagnostics(&self, uri: &str) -> Vec<crate::diagnostics::Diagnostic> {
        if let Some(doc) = self.documents.get(uri) {
            doc.parse_errors.iter()
                .map(|e| crate::diagnostics::Diagnostic::from_parse_error(e, &doc.text))
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl Default for LspServer {
    fn default() -> Self { Self::new() }
}
