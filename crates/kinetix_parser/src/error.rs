//! Parse error types for the Kinetix parser.

use kinetix_lexer::{Span, TokenKind};
use thiserror::Error;

/// A single parse error with position information and a human-readable message.
#[derive(Debug, Clone, Error)]
pub enum ParseError {
    #[error("{span:?}: unexpected token `{found}`, expected {expected}")]
    UnexpectedToken {
        span:     Span,
        found:    String,
        expected: String,
    },

    #[error("{span:?}: unexpected end of file, expected {expected}")]
    UnexpectedEof {
        span:     Span,
        expected: String,
    },

    #[error("{span:?}: {message}")]
    Custom {
        span:    Span,
        message: String,
    },

    #[error("{span:?}: unterminated string literal")]
    UnterminatedString { span: Span },

    #[error("{span:?}: invalid integer literal `{text}`")]
    InvalidIntLiteral { span: Span, text: String },

    #[error("{span:?}: invalid float literal `{text}`")]
    InvalidFloatLiteral { span: Span, text: String },

    #[error("{span:?}: too many generic arguments")]
    TooManyGenerics { span: Span },
}

impl ParseError {
    pub fn span(&self) -> Span {
        match self {
            Self::UnexpectedToken { span, .. }    => *span,
            Self::UnexpectedEof   { span, .. }    => *span,
            Self::Custom          { span, .. }    => *span,
            Self::UnterminatedString { span }     => *span,
            Self::InvalidIntLiteral { span, .. }  => *span,
            Self::InvalidFloatLiteral { span, .. } => *span,
            Self::TooManyGenerics { span }        => *span,
        }
    }

    pub fn unexpected(span: Span, found: &TokenKind, expected: &str) -> Self {
        Self::UnexpectedToken {
            span,
            found:    found.describe().to_owned(),
            expected: expected.to_owned(),
        }
    }
}
