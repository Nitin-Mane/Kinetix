//! Error-resilient lexer for the Kinetix language.

use crate::token::{keyword_or_ident, OrderedFloat, Span, Token, TokenKind};

// ---------------------------------------------------------------------------
// Lexer
// ---------------------------------------------------------------------------

/// An iterator-based, error-resilient lexer for Kinetix source code.
///
/// The lexer consumes a source string and produces a stream of [`Token`]s.
/// On encountering an unrecognized character it emits [`TokenKind::Unknown`]
/// instead of panicking, so the IDE can continue syntax-highlighting even
/// on partially broken files.
pub struct Lexer<'src> {
    src:    &'src str,
    /// Byte offset of the current position.
    pos:    usize,
    /// Current 1-indexed line.
    line:   u32,
    /// Byte offset of the start of the current line (for column calculation).
    line_start: usize,
    finished: bool,
}

impl<'src> Lexer<'src> {
    /// Create a new lexer over `src`.
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            pos: 0,
            line: 1,
            line_start: 0,
            finished: false,
        }
    }

    // ── Source access helpers ──────────────────────────────────────────────

    /// Return the byte at `self.pos` without advancing.
    fn current(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    /// Return the byte at `self.pos + 1` without advancing.
    fn peek_next(&self) -> Option<char> {
        let mut chars = self.src[self.pos..].chars();
        chars.next(); // skip current
        chars.next()
    }

    /// Advance by one character and return it.
    fn advance(&mut self) -> Option<char> {
        let ch = self.current()?;
        self.pos += ch.len_utf8();
        if ch == '\n' {
            self.line += 1;
            self.line_start = self.pos;
        }
        Some(ch)
    }

    /// Return the current 1-indexed column.
    fn column(&self) -> u32 {
        (self.pos - self.line_start + 1) as u32
    }

    /// Return `true` and advance if the next char equals `expected`.
    fn eat_if(&mut self, expected: char) -> bool {
        if self.current() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Build a [`Token`] from a start position.
    fn make_token(&self, kind: TokenKind, start: usize, start_line: u32, start_col: u32) -> Token {
        Token::new(kind, Span::new(start, self.pos), start_line, start_col)
    }

    // ── Skip helpers ──────────────────────────────────────────────────────

    fn skip_while<F: Fn(char) -> bool>(&mut self, pred: F) {
        while self.current().map(&pred).unwrap_or(false) {
            self.advance();
        }
    }

    // ── Lex helpers ───────────────────────────────────────────────────────

    fn lex_whitespace(&mut self, start: usize, sl: u32, sc: u32) -> Token {
        self.skip_while(|c| c == ' ' || c == '\t' || c == '\r');
        self.make_token(TokenKind::Whitespace, start, sl, sc)
    }

    fn lex_newline(&mut self, start: usize, sl: u32, sc: u32) -> Token {
        self.make_token(TokenKind::Newline, start, sl, sc)
    }

    fn lex_line_comment(&mut self, start: usize, sl: u32, sc: u32) -> Token {
        let text_start = self.pos;
        while let Some(c) = self.current() {
            if c == '\n' { break; }
            self.advance();
        }
        let text = self.src[text_start..self.pos].to_owned();
        self.make_token(TokenKind::LineComment(text), start, sl, sc)
    }

    fn lex_block_comment(&mut self, start: usize, sl: u32, sc: u32) -> Token {
        let text_start = self.pos;
        let mut depth = 1usize;
        loop {
            match (self.current(), self.peek_next()) {
                (Some('/'), Some('*')) => {
                    self.advance(); self.advance();
                    depth += 1;
                }
                (Some('*'), Some('/')) => {
                    self.advance(); self.advance();
                    depth -= 1;
                    if depth == 0 { break; }
                }
                (None, _) => break, // unterminated — error resilient
                _ => { self.advance(); }
            }
        }
        let text = self.src[text_start..self.pos].to_owned();
        self.make_token(TokenKind::BlockComment(text), start, sl, sc)
    }

    fn lex_number(&mut self, start: usize, sl: u32, sc: u32) -> Token {
        self.skip_while(|c| c.is_ascii_digit() || c == '_');
        let is_float = self.current() == Some('.') && self.peek_next().map(|c| c.is_ascii_digit()).unwrap_or(false);
        if is_float {
            self.advance(); // consume '.'
            self.skip_while(|c| c.is_ascii_digit() || c == '_');
            // optional exponent
            if let Some('e') | Some('E') = self.current() {
                self.advance();
                if let Some('+') | Some('-') = self.current() {
                    self.advance();
                }
                self.skip_while(|c| c.is_ascii_digit());
            }
            let text = self.src[start..self.pos].replace('_', "");
            let val: f64 = text.parse().unwrap_or(f64::NAN);
            self.make_token(TokenKind::FloatLiteral(OrderedFloat(val)), start, sl, sc)
        } else {
            let text = self.src[start..self.pos].replace('_', "");
            let val: i64 = text.parse().unwrap_or(0);
            self.make_token(TokenKind::IntLiteral(val), start, sl, sc)
        }
    }

    fn lex_hex_number(&mut self, start: usize, sl: u32, sc: u32) -> Token {
        // already consumed "0x"
        self.skip_while(|c| c.is_ascii_hexdigit() || c == '_');
        let text = self.src[start..self.pos].replace('_', "");
        // strip "0x" prefix for parsing
        let val = i64::from_str_radix(text.trim_start_matches("0x").trim_start_matches("0X"), 16)
            .unwrap_or(0);
        self.make_token(TokenKind::IntLiteral(val), start, sl, sc)
    }

    fn lex_string(&mut self, start: usize, sl: u32, sc: u32) -> Token {
        let mut s = String::new();
        loop {
            match self.advance() {
                None | Some('"') => break,
                Some('\\') => {
                    match self.advance() {
                        Some('n')  => s.push('\n'),
                        Some('t')  => s.push('\t'),
                        Some('r')  => s.push('\r'),
                        Some('"')  => s.push('"'),
                        Some('\\') => s.push('\\'),
                        Some('0')  => s.push('\0'),
                        Some(c)    => { s.push('\\'); s.push(c); }
                        None       => break,
                    }
                }
                Some(c) => s.push(c),
            }
        }
        self.make_token(TokenKind::StringLiteral(s), start, sl, sc)
    }

    fn lex_char(&mut self, start: usize, sl: u32, sc: u32) -> Token {
        let ch = match self.advance() {
            None => ' ',
            Some('\\') => match self.advance() {
                Some('n')  => '\n',
                Some('t')  => '\t',
                Some('\'') => '\'',
                Some('\\') => '\\',
                Some(c)    => c,
                None       => ' ',
            },
            Some(c) => c,
        };
        self.eat_if('\''); // closing quote
        self.make_token(TokenKind::CharLiteral(ch), start, sl, sc)
    }

    fn lex_ident(&mut self, start: usize, sl: u32, sc: u32) -> Token {
        self.skip_while(|c| c.is_alphanumeric() || c == '_');
        let word = &self.src[start..self.pos];
        let kind = keyword_or_ident(word);
        self.make_token(kind, start, sl, sc)
    }

    // ── Main lex routine ──────────────────────────────────────────────────

    fn next_token(&mut self) -> Token {
        let start  = self.pos;
        let sl     = self.line;
        let sc     = self.column();

        let ch = match self.advance() {
            None => return self.make_token(TokenKind::Eof, start, sl, sc),
            Some(c) => c,
        };

        match ch {
            // Whitespace
            ' ' | '\t' | '\r' => self.lex_whitespace(start, sl, sc),
            '\n'               => self.lex_newline(start, sl, sc),

            // Comments or division
            '/' => {
                if self.eat_if('/') {
                    self.lex_line_comment(start, sl, sc)
                } else if self.eat_if('*') {
                    self.lex_block_comment(start, sl, sc)
                } else if self.eat_if('=') {
                    self.make_token(TokenKind::SlashEq, start, sl, sc)
                } else {
                    self.make_token(TokenKind::Slash, start, sl, sc)
                }
            }

            // Numbers
            '0' if self.current() == Some('x') || self.current() == Some('X') => {
                self.advance(); // 'x'
                self.lex_hex_number(start, sl, sc)
            }
            c if c.is_ascii_digit() => self.lex_number(start, sl, sc),

            // Strings
            '"'  => self.lex_string(start, sl, sc),
            '\'' => self.lex_char(start, sl, sc),

            // Identifiers / keywords
            c if c.is_alphabetic() || c == '_' => self.lex_ident(start, sl, sc),

            // Operators and punctuation
            '+' => if self.eat_if('=') { self.make_token(TokenKind::PlusEq,  start, sl, sc) }
                   else                { self.make_token(TokenKind::Plus,     start, sl, sc) },
            '-' => if self.eat_if('>') { self.make_token(TokenKind::Arrow,   start, sl, sc) }
                   else if self.eat_if('=') { self.make_token(TokenKind::MinusEq, start, sl, sc) }
                   else                { self.make_token(TokenKind::Minus,    start, sl, sc) },
            '*' => if self.eat_if('*') { self.make_token(TokenKind::StarStar, start, sl, sc) }
                   else if self.eat_if('=') { self.make_token(TokenKind::StarEq,  start, sl, sc) }
                   else                { self.make_token(TokenKind::Star,     start, sl, sc) },
            '%' => if self.eat_if('=') { self.make_token(TokenKind::PercentEq, start, sl, sc) }
                   else                { self.make_token(TokenKind::Percent,  start, sl, sc) },
            '=' => if self.eat_if('=') { self.make_token(TokenKind::EqEq,    start, sl, sc) }
                   else if self.eat_if('>') { self.make_token(TokenKind::FatArrow, start, sl, sc) }
                   else                { self.make_token(TokenKind::Eq,       start, sl, sc) },
            '!' => if self.eat_if('=') { self.make_token(TokenKind::BangEq,  start, sl, sc) }
                   else                { self.make_token(TokenKind::Bang,     start, sl, sc) },
            '<' => if self.eat_if('=') { self.make_token(TokenKind::LtEq,    start, sl, sc) }
                   else if self.eat_if('<') { self.make_token(TokenKind::LtLt, start, sl, sc) }
                   else                { self.make_token(TokenKind::Lt,       start, sl, sc) },
            '>' => if self.eat_if('=') { self.make_token(TokenKind::GtEq,    start, sl, sc) }
                   else if self.eat_if('>') { self.make_token(TokenKind::GtGt, start, sl, sc) }
                   else                { self.make_token(TokenKind::Gt,       start, sl, sc) },
            '&' => if self.eat_if('&') { self.make_token(TokenKind::AmpAmp,  start, sl, sc) }
                   else                { self.make_token(TokenKind::Amp,      start, sl, sc) },
            '|' => if self.eat_if('|') { self.make_token(TokenKind::PipePipe, start, sl, sc) }
                   else                { self.make_token(TokenKind::Pipe,     start, sl, sc) },
            '^' => self.make_token(TokenKind::Caret,    start, sl, sc),
            '~' => self.make_token(TokenKind::Tilde,    start, sl, sc),
            '(' => self.make_token(TokenKind::LParen,   start, sl, sc),
            ')' => self.make_token(TokenKind::RParen,   start, sl, sc),
            '{' => self.make_token(TokenKind::LBrace,   start, sl, sc),
            '}' => self.make_token(TokenKind::RBrace,   start, sl, sc),
            '[' => self.make_token(TokenKind::LBracket, start, sl, sc),
            ']' => self.make_token(TokenKind::RBracket, start, sl, sc),
            ',' => self.make_token(TokenKind::Comma,    start, sl, sc),
            ';' => self.make_token(TokenKind::Semicolon, start, sl, sc),
            ':' => if self.eat_if(':') { self.make_token(TokenKind::ColonColon, start, sl, sc) }
                   else                { self.make_token(TokenKind::Colon,    start, sl, sc) },
            '.' => if self.eat_if('.') {
                        if self.eat_if('=') { self.make_token(TokenKind::DotDotEq, start, sl, sc) }
                        else               { self.make_token(TokenKind::DotDot,    start, sl, sc) }
                   } else {
                       self.make_token(TokenKind::Dot, start, sl, sc)
                   },
            '@' => self.make_token(TokenKind::At,       start, sl, sc),
            '#' => self.make_token(TokenKind::Hash,     start, sl, sc),
            '?' => self.make_token(TokenKind::Question, start, sl, sc),

            // Unknown — error-resilient recovery
            c => self.make_token(TokenKind::Unknown(c), start, sl, sc),
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.finished {
            return None;
        }
        let tok = self.next_token();
        if tok.kind == TokenKind::Eof {
            self.finished = true;
        }
        Some(tok)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenKind;

    fn lex(src: &str) -> Vec<TokenKind> {
        Lexer::new(src).map(|t| t.kind).collect()
    }

    fn lex_no_trivia(src: &str) -> Vec<TokenKind> {
        Lexer::new(src)
            .filter(|t| !t.is_trivia())
            .map(|t| t.kind)
            .collect()
    }

    #[test]
    fn test_keywords() {
        let kinds = lex_no_trivia("fn let mut if else return");
        assert_eq!(kinds, vec![
            TokenKind::Fn,
            TokenKind::Let,
            TokenKind::Mut,
            TokenKind::If,
            TokenKind::Else,
            TokenKind::Return,
            TokenKind::Eof,
        ]);
    }

    #[test]
    fn test_integer_literal() {
        let kinds = lex_no_trivia("42 0xFF 1_000_000");
        assert_eq!(kinds, vec![
            TokenKind::IntLiteral(42),
            TokenKind::IntLiteral(255),
            TokenKind::IntLiteral(1_000_000),
            TokenKind::Eof,
        ]);
    }

    #[test]
    fn test_float_literal() {
        let kinds = lex_no_trivia("3.14 2.0e10");
        assert!(matches!(kinds[0], TokenKind::FloatLiteral(_)));
        assert!(matches!(kinds[1], TokenKind::FloatLiteral(_)));
    }

    #[test]
    fn test_string_literal() {
        let kinds = lex_no_trivia(r#""hello\nworld""#);
        assert_eq!(kinds, vec![
            TokenKind::StringLiteral("hello\nworld".into()),
            TokenKind::Eof,
        ]);
    }

    #[test]
    fn test_operators() {
        let kinds = lex_no_trivia("+ - * / == != <= >= -> =>");
        assert_eq!(kinds, vec![
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Star,
            TokenKind::Slash,
            TokenKind::EqEq,
            TokenKind::BangEq,
            TokenKind::LtEq,
            TokenKind::GtEq,
            TokenKind::Arrow,
            TokenKind::FatArrow,
            TokenKind::Eof,
        ]);
    }

    #[test]
    fn test_line_comment() {
        let toks: Vec<_> = Lexer::new("// hello\nlet").collect();
        assert!(matches!(toks[0].kind, TokenKind::LineComment(_)));
        assert_eq!(toks[1].kind, TokenKind::Newline);
        assert_eq!(toks[2].kind, TokenKind::Let);
    }

    #[test]
    fn test_block_comment() {
        let toks: Vec<_> = Lexer::new("/* block\n comment */let").collect();
        assert!(matches!(toks[0].kind, TokenKind::BlockComment(_)));
        assert_eq!(toks[1].kind, TokenKind::Let);
    }

    #[test]
    fn test_unknown_resilience() {
        let kinds = lex_no_trivia("let $ x");
        assert_eq!(kinds[0], TokenKind::Let);
        assert!(matches!(kinds[1], TokenKind::Unknown('$')));
        assert!(matches!(kinds[2], TokenKind::Ident(_)));
    }

    #[test]
    fn test_span_positions() {
        let toks: Vec<_> = Lexer::new("let x").collect();
        assert_eq!(toks[0].span, Span::new(0, 3)); // "let"
        // toks[1] is whitespace
        assert_eq!(toks[2].span, Span::new(4, 5)); // "x"
    }

    #[test]
    fn test_full_function() {
        let src = r#"
fn add(a: int, b: int) -> int {
    return a + b
}
"#;
        let toks: Vec<_> = Lexer::new(src)
            .filter(|t| !t.is_trivia())
            .collect();
        assert!(toks.iter().any(|t| t.kind == TokenKind::Fn));
        assert!(toks.iter().any(|t| t.kind == TokenKind::Return));
        assert!(toks.iter().any(|t| t.kind == TokenKind::Arrow));
    }
}
