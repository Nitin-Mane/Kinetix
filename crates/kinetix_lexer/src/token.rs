//! Token types and span representation for the Kinetix lexer.

use std::fmt;

// ---------------------------------------------------------------------------
// Span
// ---------------------------------------------------------------------------

/// A byte-range in the source text [start, end).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end:   usize,
}

impl Span {
    #[inline]
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
}

// ---------------------------------------------------------------------------
// TokenKind
// ---------------------------------------------------------------------------

/// All token types recognized by the Kinetix lexer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // ── Keywords ──────────────────────────────────────────────────────────
    // Control flow
    If,
    Else,
    Elif,
    For,
    While,
    Loop,
    Break,
    Continue,
    Return,
    Match,
    // Declarations
    Fn,
    Let,
    Mut,
    Const,
    Struct,
    Impl,
    Enum,
    Trait,
    Type,
    // Modules
    Import,
    Export,
    Mod,
    // Values
    True,
    False,
    Nil,
    // MATLAB-inspired
    Matrix,
    // Async
    Async,
    Await,
    Spawn,

    // ── Primitive types ───────────────────────────────────────────────────
    TyInt,
    TyFloat,
    TyBool,
    TyString,
    TyNil,
    TyVec,
    TyMap,
    TyMatrix,

    // ── Literals ──────────────────────────────────────────────────────────
    IntLiteral(i64),
    FloatLiteral(OrderedFloat),
    StringLiteral(String),
    CharLiteral(char),

    // ── Identifiers ───────────────────────────────────────────────────────
    Ident(String),

    // ── Operators ─────────────────────────────────────────────────────────
    // Arithmetic
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    StarStar,   // ** (power)
    // Comparison
    EqEq,
    BangEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    // Logical
    AmpAmp,
    PipePipe,
    Bang,
    // Bitwise
    Amp,
    Pipe,
    Caret,
    Tilde,
    LtLt,
    GtGt,
    // Assignment
    Eq,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    // Range
    DotDot,
    DotDotEq,
    // Arrow
    Arrow,       // ->
    FatArrow,    // =>

    // ── Delimiters ────────────────────────────────────────────────────────
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // ── Punctuation ───────────────────────────────────────────────────────
    Comma,
    Semicolon,
    Colon,
    ColonColon, // ::
    Dot,
    At,
    Hash,
    Question,

    // ── Comments ──────────────────────────────────────────────────────────
    LineComment(String),
    BlockComment(String),

    // ── Whitespace ────────────────────────────────────────────────────────
    Newline,
    Whitespace,

    // ── Special ───────────────────────────────────────────────────────────
    /// An unrecognized character — error-resilient recovery token.
    Unknown(char),
    /// End of file.
    Eof,
}

impl TokenKind {
    /// Returns `true` if this token is trivia (whitespace/comment) that the
    /// parser typically skips.
    pub fn is_trivia(&self) -> bool {
        matches!(
            self,
            TokenKind::Whitespace
                | TokenKind::Newline
                | TokenKind::LineComment(_)
                | TokenKind::BlockComment(_)
        )
    }

    /// Human-readable name for display in error messages.
    pub fn describe(&self) -> &'static str {
        match self {
            TokenKind::Fn       => "`fn`",
            TokenKind::Let      => "`let`",
            TokenKind::Mut      => "`mut`",
            TokenKind::If       => "`if`",
            TokenKind::Else     => "`else`",
            TokenKind::For      => "`for`",
            TokenKind::While    => "`while`",
            TokenKind::Loop     => "`loop`",
            TokenKind::Return   => "`return`",
            TokenKind::Match    => "`match`",
            TokenKind::Struct   => "`struct`",
            TokenKind::Impl     => "`impl`",
            TokenKind::True     => "`true`",
            TokenKind::False    => "`false`",
            TokenKind::Nil      => "`nil`",
            TokenKind::Matrix   => "`matrix`",
            TokenKind::Import   => "`import`",
            TokenKind::Export   => "`export`",
            TokenKind::IntLiteral(_)    => "integer literal",
            TokenKind::FloatLiteral(_)  => "float literal",
            TokenKind::StringLiteral(_) => "string literal",
            TokenKind::Ident(_)         => "identifier",
            TokenKind::Plus     => "`+`",
            TokenKind::Minus    => "`-`",
            TokenKind::Star     => "`*`",
            TokenKind::Slash    => "`/`",
            TokenKind::Eq       => "`=`",
            TokenKind::EqEq     => "`==`",
            TokenKind::Arrow    => "`->`",
            TokenKind::LParen   => "`(`",
            TokenKind::RParen   => "`)`",
            TokenKind::LBrace   => "`{`",
            TokenKind::RBrace   => "`}`",
            TokenKind::LBracket => "`[`",
            TokenKind::RBracket => "`]`",
            TokenKind::Comma    => "`,`",
            TokenKind::Semicolon => "`;`",
            TokenKind::Colon    => "`:`",
            TokenKind::ColonColon => "`::`",
            TokenKind::Dot      => "`.`",
            TokenKind::Eof      => "<eof>",
            TokenKind::Unknown(_) => "<unknown>",
            _                   => "token",
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.describe())
    }
}

// ---------------------------------------------------------------------------
// Float wrapper that allows PartialEq / Eq / Hash on f64
// ---------------------------------------------------------------------------

/// A wrapper around `f64` that provides total ordering, `Eq`, and `Hash`
/// by treating NaN as equal to NaN (for token identity purposes only).
#[derive(Debug, Clone, Copy)]
pub struct OrderedFloat(pub f64);

impl PartialEq for OrderedFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for OrderedFloat {}

impl std::hash::Hash for OrderedFloat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl fmt::Display for OrderedFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// Token
// ---------------------------------------------------------------------------

/// A token produced by the Kinetix lexer, combining a [`TokenKind`] with the
/// [`Span`] in the original source text and the line/column for diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind:   TokenKind,
    pub span:   Span,
    /// 1-indexed line number.
    pub line:   u32,
    /// 1-indexed column number (byte offset on the line).
    pub column: u32,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, line: u32, column: u32) -> Self {
        Self { kind, span, line, column }
    }

    /// Returns `true` if this token is trivia.
    pub fn is_trivia(&self) -> bool {
        self.kind.is_trivia()
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} @ {}:{}", self.kind, self.line, self.column)
    }
}

// ---------------------------------------------------------------------------
// Keyword table
// ---------------------------------------------------------------------------

/// Map a string slice to a keyword token, or return `None` if it is a plain
/// identifier.
pub fn keyword_or_ident(s: &str) -> TokenKind {
    match s {
        // Control flow
        "if"       => TokenKind::If,
        "else"     => TokenKind::Else,
        "elif"     => TokenKind::Elif,
        "for"      => TokenKind::For,
        "while"    => TokenKind::While,
        "loop"     => TokenKind::Loop,
        "break"    => TokenKind::Break,
        "continue" => TokenKind::Continue,
        "return"   => TokenKind::Return,
        "match"    => TokenKind::Match,
        // Declarations
        "fn"       => TokenKind::Fn,
        "let"      => TokenKind::Let,
        "mut"      => TokenKind::Mut,
        "const"    => TokenKind::Const,
        "struct"   => TokenKind::Struct,
        "impl"     => TokenKind::Impl,
        "enum"     => TokenKind::Enum,
        "trait"    => TokenKind::Trait,
        "type"     => TokenKind::Type,
        // Modules
        "import"   => TokenKind::Import,
        "export"   => TokenKind::Export,
        "mod"      => TokenKind::Mod,
        // Async
        "async"    => TokenKind::Async,
        "await"    => TokenKind::Await,
        "spawn"    => TokenKind::Spawn,
        // Values
        "true"     => TokenKind::True,
        "false"    => TokenKind::False,
        "nil"      => TokenKind::Nil,
        // MATLAB-inspired
        "matrix"   => TokenKind::Matrix,
        // Built-in types
        "int"      => TokenKind::TyInt,
        "float"    => TokenKind::TyFloat,
        "bool"     => TokenKind::TyBool,
        "string"   => TokenKind::TyString,
        "vec"      => TokenKind::TyVec,
        "map"      => TokenKind::TyMap,
        // Everything else is an identifier
        other      => TokenKind::Ident(other.to_owned()),
    }
}
