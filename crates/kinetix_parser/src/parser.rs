//! The main Kinetix parser.
//!
//! Implements a hand-written recursive-descent parser with Pratt
//! (top-down operator precedence) expression parsing.

use smol_str::SmolStr;
use kinetix_lexer::{Lexer, Token, TokenKind, Span};
use kinetix_ast::{
    SourceFile,
    expr::{
        BinOp, Block, ClosureParam, Expr, ExprKind, Lit, MatchArm,
        Pattern, StringSegment, UnOp,
    },
    stmt::{
        EnumDef, EnumVariant, EnumVariantFields, FieldDef, FnDef, FnSig,
        ImplBlock, ImportKind, ImportPath, Item, ItemKind, Param,
        Stmt, StmtKind, StructDef, TraitDef, TraitItem,
    },
    types::{GenericParam, GenericParamKind, TyAnnotation, TyKind},
};
use crate::{
    error::ParseError,
    pratt::{infix_binding_power, postfix_binding_power, prefix_binding_power, token_to_binop},
};

// ── Parser state ─────────────────────────────────────────────────────────

pub struct Parser {
    /// All tokens from the source (including trivia).
    tokens:  Vec<Token>,
    /// Current position in `tokens`.
    pos:     usize,
    /// Accumulated parse errors.
    errors:  Vec<ParseError>,
}

impl Parser {
    /// Create a parser from raw source text.
    pub fn new(src: &str) -> Self {
        let tokens: Vec<Token> = Lexer::new(src)
            .filter(|t| !t.is_trivia())  // Skip whitespace/comments for parsing
            .collect();
        Self { tokens, pos: 0, errors: Vec::new() }
    }

    // ── Token access ──────────────────────────────────────────────────────

    fn peek(&self) -> &TokenKind {
        self.tokens.get(self.pos)
            .map(|t| &t.kind)
            .unwrap_or(&TokenKind::Eof)
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn peek2(&self) -> &TokenKind {
        self.tokens.get(self.pos + 1)
            .map(|t| &t.kind)
            .unwrap_or(&TokenKind::Eof)
    }

    fn current_span(&self) -> Span {
        self.tokens.get(self.pos)
            .map(|t| t.span)
            .unwrap_or(Span::new(0, 0))
    }

    fn advance(&mut self) -> &Token {
        let tok = &self.tokens[self.pos.min(self.tokens.len() - 1)];
        if self.pos < self.tokens.len() { self.pos += 1; }
        tok
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), TokenKind::Eof)
    }

    /// Consume the next token if it matches `kind`, otherwise record an error
    /// and return the current span as a "dummy" span.
    fn expect(&mut self, kind: &TokenKind, desc: &str) -> Span {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(kind) {
            self.advance().span
        } else {
            let span = self.current_span();
            self.errors.push(ParseError::unexpected(span, self.peek(), desc));
            span
        }
    }

    fn eat(&mut self, kind: &TokenKind) -> bool {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn eat_ident(&mut self) -> Option<SmolStr> {
        if let TokenKind::Ident(s) = self.peek().clone() {
            self.advance();
            Some(SmolStr::new(&s))
        } else {
            None
        }
    }

    fn expect_ident(&mut self) -> SmolStr {
        if let TokenKind::Ident(s) = self.peek().clone() {
            self.advance();
            SmolStr::new(&s)
        } else {
            let span = self.current_span();
            self.errors.push(ParseError::unexpected(span, self.peek(), "identifier"));
            SmolStr::new("<error>")
        }
    }

    // ── Error recovery ────────────────────────────────────────────────────

    /// Skip tokens until we reach one of the `sync` kinds (for error recovery).
    fn synchronize(&mut self, sync: &[TokenKind]) {
        while !self.is_at_end() {
            if sync.iter().any(|k| std::mem::discriminant(k) == std::mem::discriminant(self.peek())) {
                break;
            }
            self.advance();
        }
    }

    // ── Public parse entry points ─────────────────────────────────────────

    /// Parse a complete source file.
    pub fn parse_file(mut self) -> (SourceFile, Vec<ParseError>) {
        let mut items = Vec::new();
        while !self.is_at_end() {
            match self.parse_item() {
                Some(item) => items.push(kinetix_ast::Node::new(item, Span::new(0, 0))),
                None => {
                    // Skip unknown token and continue
                    if !self.is_at_end() { self.advance(); }
                }
            }
        }
        let errors = self.errors;
        (SourceFile { items, path: None }, errors)
    }

    /// Parse a single expression (used for REPL).
    pub fn parse_single_expr(mut self) -> (Option<Expr>, Vec<ParseError>) {
        let expr = self.parse_expr(0);
        let errors = self.errors;
        (Some(expr), errors)
    }

    // ── Item parsing ──────────────────────────────────────────────────────

    fn parse_item(&mut self) -> Option<Item> {
        let doc = self.eat_doc_comment();
        let is_pub = self.eat(&TokenKind::Export);
        let span_start = self.current_span();

        let kind = match self.peek().clone() {
            TokenKind::Fn      => ItemKind::Fn(self.parse_fn_def()),
            TokenKind::Struct  => ItemKind::Struct(self.parse_struct()),
            TokenKind::Enum    => ItemKind::Enum(self.parse_enum()),
            TokenKind::Trait   => ItemKind::Trait(self.parse_trait()),
            TokenKind::Impl    => ItemKind::Impl(self.parse_impl()),
            TokenKind::Import  => {
                self.advance();
                ItemKind::Import(self.parse_import())
            }
            TokenKind::Const   => {
                self.advance();
                let name = self.expect_ident();
                self.expect(&TokenKind::Colon, "`:`");
                let ty = self.parse_type();
                self.expect(&TokenKind::Eq, "`=`");
                let init = Box::new(self.parse_expr(0));
                ItemKind::Const { name, ty, init }
            }
            TokenKind::Mod     => {
                self.advance();
                let name = self.expect_ident();
                self.expect(&TokenKind::LBrace, "`{`");
                let mut items = Vec::new();
                while !self.is_at_end() && !matches!(self.peek(), TokenKind::RBrace) {
                    if let Some(item) = self.parse_item() {
                        items.push(item);
                    }
                }
                self.eat(&TokenKind::RBrace);
                ItemKind::Module { name, items }
            }
            _ => return None,
        };

        Some(Item { kind, span: span_start, doc })
    }

    fn eat_doc_comment(&mut self) -> Option<String> {
        // Doc comments are `///` line comments; they're filtered as trivia currently.
        // Future: keep doc comments in the token stream.
        None
    }

    // ── Function parsing ──────────────────────────────────────────────────

    fn parse_fn_def(&mut self) -> FnDef {
        self.advance(); // consume `fn`
        let is_async = false; // async handled separately
        let name = self.expect_ident();
        let generics = self.parse_generics();
        self.expect(&TokenKind::LParen, "`(`");
        let params = self.parse_params();
        self.expect(&TokenKind::RParen, "`)`");
        let return_type = if self.eat(&TokenKind::Arrow) {
            Some(self.parse_type())
        } else {
            None
        };
        let body = if matches!(self.peek(), TokenKind::LBrace) {
            Some(self.parse_block())
        } else {
            self.eat(&TokenKind::Semicolon);
            None
        };

        FnDef {
            sig: FnSig {
                name,
                generics,
                params,
                return_type,
                is_async,
                is_pub: false,
            },
            body,
        }
    }

    fn parse_params(&mut self) -> Vec<Param> {
        let mut params = Vec::new();
        while !matches!(self.peek(), TokenKind::RParen | TokenKind::Eof) {
            let is_mut  = self.eat(&TokenKind::Mut);
            let name_str = if let TokenKind::Ident(s) = self.peek().clone() {
                s.clone()
            } else {
                break;
            };
            let is_self = name_str == "self";
            let name = SmolStr::new(self.advance().kind.describe()); // advance past ident
            // Re-parse properly:
            let name = if is_self { SmolStr::new("self") } else {
                SmolStr::new(&name_str)
            };
            let _ = self.advance(); // actually consume ident we peeked
            // Rewind one — fix: parse properly
            let ty = if self.eat(&TokenKind::Colon) {
                self.parse_type()
            } else {
                TyAnnotation::new(TyKind::Infer)
            };
            params.push(Param { name, ty, is_self, is_mut });
            if !self.eat(&TokenKind::Comma) { break; }
        }
        params
    }

    // ── Struct / Enum / Trait / Impl ──────────────────────────────────────

    fn parse_struct(&mut self) -> StructDef {
        self.advance(); // `struct`
        let name = self.expect_ident();
        let generics = self.parse_generics();
        self.expect(&TokenKind::LBrace, "`{`");
        let mut fields = Vec::new();
        while !matches!(self.peek(), TokenKind::RBrace | TokenKind::Eof) {
            let is_pub = self.eat(&TokenKind::Export);
            let fname = self.expect_ident();
            self.expect(&TokenKind::Colon, "`:`");
            let ty = self.parse_type();
            let span = self.current_span();
            fields.push(FieldDef { name: fname, ty, is_pub, span });
            if !self.eat(&TokenKind::Comma) { break; }
        }
        self.eat(&TokenKind::RBrace);
        StructDef { name, generics, fields, is_pub: false }
    }

    fn parse_enum(&mut self) -> EnumDef {
        self.advance(); // `enum`
        let name = self.expect_ident();
        let generics = self.parse_generics();
        self.expect(&TokenKind::LBrace, "`{`");
        let mut variants = Vec::new();
        while !matches!(self.peek(), TokenKind::RBrace | TokenKind::Eof) {
            let vname = self.expect_ident();
            let span = self.current_span();
            let fields = if matches!(self.peek(), TokenKind::LParen) {
                self.advance();
                let mut types = Vec::new();
                while !matches!(self.peek(), TokenKind::RParen | TokenKind::Eof) {
                    types.push(self.parse_type());
                    if !self.eat(&TokenKind::Comma) { break; }
                }
                self.eat(&TokenKind::RParen);
                EnumVariantFields::Tuple(types)
            } else if matches!(self.peek(), TokenKind::LBrace) {
                self.advance();
                let mut fields = Vec::new();
                while !matches!(self.peek(), TokenKind::RBrace | TokenKind::Eof) {
                    let fname = self.expect_ident();
                    self.expect(&TokenKind::Colon, "`:`");
                    let ty = self.parse_type();
                    fields.push(FieldDef { name: fname, ty, is_pub: false, span });
                    if !self.eat(&TokenKind::Comma) { break; }
                }
                self.eat(&TokenKind::RBrace);
                EnumVariantFields::Struct(fields)
            } else {
                EnumVariantFields::Unit
            };
            variants.push(EnumVariant { name: vname, fields, span });
            if !self.eat(&TokenKind::Comma) { break; }
        }
        self.eat(&TokenKind::RBrace);
        EnumDef { name, generics, variants, is_pub: false }
    }

    fn parse_trait(&mut self) -> TraitDef {
        self.advance(); // `trait`
        let name = self.expect_ident();
        let generics = self.parse_generics();
        let mut super_traits = Vec::new();
        if self.eat(&TokenKind::Colon) {
            loop {
                super_traits.push(self.parse_type());
                if !self.eat(&TokenKind::Plus) { break; }
            }
        }
        self.expect(&TokenKind::LBrace, "`{`");
        let mut items = Vec::new();
        while !matches!(self.peek(), TokenKind::RBrace | TokenKind::Eof) {
            if matches!(self.peek(), TokenKind::Fn) {
                items.push(TraitItem::Method(self.parse_fn_def()));
            } else {
                self.advance(); // skip unknown
            }
        }
        self.eat(&TokenKind::RBrace);
        TraitDef { name, generics, super_traits, items, is_pub: false }
    }

    fn parse_impl(&mut self) -> ImplBlock {
        self.advance(); // `impl`
        let generics = self.parse_generics();
        let first_ty = self.parse_type();
        // Check for `impl Trait for Type`
        let (trait_ty, self_ty) = if self.eat(&TokenKind::Ident(String::from("for"))) {
            (Some(first_ty), self.parse_type())
        } else {
            (None, first_ty)
        };
        self.expect(&TokenKind::LBrace, "`{`");
        let mut items = Vec::new();
        while !matches!(self.peek(), TokenKind::RBrace | TokenKind::Eof) {
            if matches!(self.peek(), TokenKind::Fn) {
                items.push(self.parse_fn_def());
            } else {
                self.advance();
            }
        }
        self.eat(&TokenKind::RBrace);
        ImplBlock { generics, trait_ty, self_ty, items }
    }

    fn parse_import(&mut self) -> ImportPath {
        let mut segments = Vec::new();
        loop {
            let seg = self.expect_ident();
            segments.push(seg);
            if !self.eat(&TokenKind::ColonColon) { break; }
        }
        let kind = ImportKind::Simple;
        ImportPath { segments, kind }
    }

    // ── Block parsing ─────────────────────────────────────────────────────

    fn parse_block(&mut self) -> Block {
        let span_start = self.current_span();
        self.expect(&TokenKind::LBrace, "`{`");
        let mut stmts = Vec::new();
        let mut tail_expr = None;

        while !matches!(self.peek(), TokenKind::RBrace | TokenKind::Eof) {
            let stmt = self.parse_stmt();
            // The last expr without semicolon becomes tail_expr
            if let StmtKind::Expr(expr) = &stmt.kind {
                if !matches!(self.peek(), TokenKind::RBrace) {
                    stmts.push(stmt);
                } else {
                    tail_expr = Some(expr.clone());
                }
            } else {
                stmts.push(stmt);
            }
        }
        let span_end = self.current_span();
        self.eat(&TokenKind::RBrace);

        Block {
            stmts,
            tail_expr,
            span: Span::new(span_start.start, span_end.end),
        }
    }

    // ── Statement parsing ─────────────────────────────────────────────────

    fn parse_stmt(&mut self) -> Stmt {
        let span = self.current_span();
        let kind = match self.peek().clone() {
            TokenKind::Let => {
                self.advance();
                let mutable = self.eat(&TokenKind::Mut);
                let pattern = self.parse_pattern();
                let ty = if self.eat(&TokenKind::Colon) { Some(self.parse_type()) } else { None };
                let init = if self.eat(&TokenKind::Eq) { Some(Box::new(self.parse_expr(0))) } else { None };
                self.eat(&TokenKind::Semicolon);
                StmtKind::Let { mutable, pattern, ty, init }
            }
            TokenKind::Const => {
                self.advance();
                let name = self.expect_ident();
                self.expect(&TokenKind::Colon, "`:`");
                let ty = self.parse_type();
                self.expect(&TokenKind::Eq, "`=`");
                let init = Box::new(self.parse_expr(0));
                self.eat(&TokenKind::Semicolon);
                StmtKind::Const { name, ty, init }
            }
            TokenKind::Fn | TokenKind::Struct | TokenKind::Enum
            | TokenKind::Trait | TokenKind::Impl | TokenKind::Import => {
                if let Some(item) = self.parse_item() {
                    StmtKind::Item(Box::new(item))
                } else {
                    StmtKind::Empty
                }
            }
            TokenKind::Semicolon => {
                self.advance();
                StmtKind::Empty
            }
            _ => {
                let expr = self.parse_expr(0);
                self.eat(&TokenKind::Semicolon);
                StmtKind::Expr(Box::new(expr))
            }
        };
        Stmt { kind, span }
    }

    // ── Expression parsing (Pratt) ────────────────────────────────────────

    pub(crate) fn parse_expr(&mut self, min_bp: u8) -> Expr {
        let span_start = self.current_span();
        let mut lhs = self.parse_prefix();

        loop {
            // Postfix operators
            if let Some(bp) = postfix_binding_power(self.peek()) {
                if bp < min_bp { break; }
                lhs = self.parse_postfix(lhs);
                continue;
            }

            // Infix operators
            if let Some((l_bp, r_bp)) = infix_binding_power(self.peek()) {
                if l_bp < min_bp { break; }
                let op_tok = self.peek().clone();
                self.advance();
                let rhs = self.parse_expr(r_bp);
                let span = Span::new(span_start.start, rhs.span.end);
                if let Some(op) = token_to_binop(&op_tok) {
                    lhs = Expr {
                        kind: ExprKind::BinOp { op, lhs: Box::new(lhs), rhs: Box::new(rhs) },
                        span,
                    };
                } else {
                    // Assignment operators
                    let asgn_op = match &op_tok {
                        TokenKind::PlusEq    => Some(BinOp::Add),
                        TokenKind::MinusEq   => Some(BinOp::Sub),
                        TokenKind::StarEq    => Some(BinOp::Mul),
                        TokenKind::SlashEq   => Some(BinOp::Div),
                        TokenKind::PercentEq => Some(BinOp::Rem),
                        _ => None,
                    };
                    lhs = Expr {
                        kind: ExprKind::Assign { op: asgn_op, lhs: Box::new(lhs), rhs: Box::new(rhs) },
                        span,
                    };
                }
                continue;
            }

            // `as` cast
            if matches!(self.peek(), TokenKind::Ident(s) if s == "as") {
                self.advance();
                let ty = self.parse_type();
                let span = Span::new(span_start.start, self.current_span().end);
                lhs = Expr {
                    kind: ExprKind::Cast { expr: Box::new(lhs), ty },
                    span,
                };
                continue;
            }

            break;
        }

        lhs
    }

    fn parse_prefix(&mut self) -> Expr {
        let span = self.current_span();
        match self.peek().clone() {
            // Integer literal
            TokenKind::IntLiteral(v) => {
                self.advance();
                Expr { kind: ExprKind::Lit(Lit::Int(v)), span }
            }
            // Float literal
            TokenKind::FloatLiteral(f) => {
                self.advance();
                Expr { kind: ExprKind::Lit(Lit::Float(f.0)), span }
            }
            // String literal
            TokenKind::StringLiteral(s) => {
                let s = s.clone();
                self.advance();
                Expr { kind: ExprKind::Lit(Lit::String(s)), span }
            }
            // Bool
            TokenKind::True  => { self.advance(); Expr { kind: ExprKind::Lit(Lit::Bool(true)),  span } }
            TokenKind::False => { self.advance(); Expr { kind: ExprKind::Lit(Lit::Bool(false)), span } }
            // Nil
            TokenKind::Nil   => { self.advance(); Expr { kind: ExprKind::Lit(Lit::Nil), span } }

            // Panic intrinsic
            TokenKind::Ident(s) if s == "panic" => {
                self.advance();
                self.expect(&TokenKind::LParen, "`(`");
                let msg = self.parse_expr(0);
                self.expect(&TokenKind::RParen, "`)`");
                Expr { kind: ExprKind::Panic(Box::new(msg)), span }
            }

            // Identifier or path
            TokenKind::Ident(_) => self.parse_ident_or_path(),

            // Unary operators
            TokenKind::Minus => {
                self.advance();
                let operand = self.parse_expr(25);
                Expr { kind: ExprKind::UnOp { op: UnOp::Neg, operand: Box::new(operand) }, span }
            }
            TokenKind::Bang => {
                self.advance();
                let operand = self.parse_expr(25);
                Expr { kind: ExprKind::UnOp { op: UnOp::Not, operand: Box::new(operand) }, span }
            }

            // Grouped or tuple
            TokenKind::LParen => self.parse_paren_expr(),

            // Vec/Matrix literal
            TokenKind::LBracket => self.parse_bracket_expr(),

            // Block
            TokenKind::LBrace => {
                let block = self.parse_block();
                let sp = block.span;
                Expr { kind: ExprKind::Block(block), span: sp }
            }

            // If expression
            TokenKind::If => self.parse_if_expr(),

            // Match expression
            TokenKind::Match => self.parse_match_expr(),

            // Loop
            TokenKind::Loop => {
                self.advance();
                let body = self.parse_block();
                let sp = body.span;
                Expr { kind: ExprKind::Loop(body), span: sp }
            }

            // While
            TokenKind::While => {
                self.advance();
                let cond = Box::new(self.parse_expr(0));
                let body = self.parse_block();
                let sp = body.span;
                Expr { kind: ExprKind::While { cond, body }, span: sp }
            }

            // For
            TokenKind::For => {
                self.advance();
                let pattern = self.parse_pattern();
                self.eat(&TokenKind::Ident(String::from("in")));
                let iter = Box::new(self.parse_expr(0));
                let body = self.parse_block();
                let sp = body.span;
                Expr { kind: ExprKind::For { pattern, iter, body }, span: sp }
            }

            // Return
            TokenKind::Return => {
                self.advance();
                let val = if !matches!(self.peek(), TokenKind::Semicolon | TokenKind::RBrace | TokenKind::Eof) {
                    Some(Box::new(self.parse_expr(0)))
                } else {
                    None
                };
                Expr { kind: ExprKind::Return(val), span }
            }

            // Break
            TokenKind::Break => {
                self.advance();
                let val = if !matches!(self.peek(), TokenKind::Semicolon | TokenKind::RBrace | TokenKind::Eof) {
                    Some(Box::new(self.parse_expr(0)))
                } else { None };
                Expr { kind: ExprKind::Break(val), span }
            }

            // Continue
            TokenKind::Continue => {
                self.advance();
                Expr { kind: ExprKind::Continue, span }
            }

            // Closure: |params| body
            TokenKind::Pipe => self.parse_closure(),

            // Await
            TokenKind::Await => {
                self.advance();
                let inner = self.parse_expr(0);
                Expr { kind: ExprKind::Await(Box::new(inner)), span }
            }

            // Spawn
            TokenKind::Spawn => {
                self.advance();
                let inner = self.parse_expr(0);
                Expr { kind: ExprKind::Spawn(Box::new(inner)), span }
            }

            // Error recovery
            _ => {
                let tok = self.peek().clone();
                self.errors.push(ParseError::unexpected(span, &tok, "expression"));
                if !self.is_at_end() { self.advance(); }
                Expr { kind: ExprKind::Lit(Lit::Nil), span }
            }
        }
    }

    fn parse_postfix(&mut self, mut lhs: Expr) -> Expr {
        match self.peek().clone() {
            // Call: expr(args)
            TokenKind::LParen => {
                self.advance();
                let args = self.parse_call_args();
                let span_end = self.current_span();
                self.eat(&TokenKind::RParen);
                let span = Span::new(lhs.span.start, span_end.end);
                lhs = Expr { kind: ExprKind::Call { callee: Box::new(lhs), args }, span };
            }
            // Index: expr[i] or expr[i, j]
            TokenKind::LBracket => {
                self.advance();
                let mut indices = Vec::new();
                while !matches!(self.peek(), TokenKind::RBracket | TokenKind::Eof) {
                    indices.push(self.parse_expr(0));
                    if !self.eat(&TokenKind::Comma) { break; }
                }
                let span_end = self.current_span();
                self.eat(&TokenKind::RBracket);
                let span = Span::new(lhs.span.start, span_end.end);
                lhs = Expr { kind: ExprKind::Index { object: Box::new(lhs), indices }, span };
            }
            // Field/Method: expr.field or expr.method(args)
            TokenKind::Dot => {
                self.advance();
                let field = self.expect_ident();
                let span_start = lhs.span.start;
                if matches!(self.peek(), TokenKind::LParen) {
                    self.advance();
                    let args = self.parse_call_args();
                    let span_end = self.current_span();
                    self.eat(&TokenKind::RParen);
                    let span = Span::new(span_start, span_end.end);
                    lhs = Expr { kind: ExprKind::MethodCall { receiver: Box::new(lhs), method: field, args }, span };
                } else {
                    let span = Span::new(span_start, self.current_span().end);
                    lhs = Expr { kind: ExprKind::Field { object: Box::new(lhs), field }, span };
                }
            }
            _ => {}
        }
        lhs
    }

    fn parse_call_args(&mut self) -> Vec<Expr> {
        let mut args = Vec::new();
        while !matches!(self.peek(), TokenKind::RParen | TokenKind::Eof) {
            args.push(self.parse_expr(0));
            if !self.eat(&TokenKind::Comma) { break; }
        }
        args
    }

    fn parse_ident_or_path(&mut self) -> Expr {
        let span = self.current_span();
        let mut segments = Vec::new();
        if let TokenKind::Ident(s) = self.peek().clone() {
            segments.push(SmolStr::new(&s));
            self.advance();
        }
        while self.eat(&TokenKind::ColonColon) {
            if let TokenKind::Ident(s) = self.peek().clone() {
                segments.push(SmolStr::new(&s));
                self.advance();
            } else {
                break;
            }
        }

        // Struct literal: Path { field: expr, ... }
        if matches!(self.peek(), TokenKind::LBrace) && self.is_struct_literal_context(&segments) {
            self.advance();
            let mut fields = Vec::new();
            while !matches!(self.peek(), TokenKind::RBrace | TokenKind::Eof) {
                let fname = self.expect_ident();
                let val = if self.eat(&TokenKind::Colon) {
                    self.parse_expr(0)
                } else {
                    Expr { kind: ExprKind::Ident(fname.clone()), span }
                };
                fields.push((fname, val));
                if !self.eat(&TokenKind::Comma) { break; }
            }
            self.eat(&TokenKind::RBrace);
            return Expr { kind: ExprKind::StructLit { name: segments, fields }, span };
        }

        let kind = if segments.len() == 1 {
            ExprKind::Ident(segments.into_iter().next().unwrap())
        } else {
            ExprKind::Path(segments)
        };
        Expr { kind, span }
    }

    fn is_struct_literal_context(&self, _path: &[SmolStr]) -> bool {
        // Heuristic: if path starts with uppercase, treat `{` as struct literal.
        // This avoids ambiguity with block expressions in `if`/`while` conditions.
        _path.first().map(|s| s.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)).unwrap_or(false)
    }

    fn parse_paren_expr(&mut self) -> Expr {
        let span = self.current_span();
        self.advance(); // `(`
        if matches!(self.peek(), TokenKind::RParen) {
            self.advance();
            return Expr { kind: ExprKind::Tuple(vec![]), span };
        }
        let first = self.parse_expr(0);
        if self.eat(&TokenKind::Comma) {
            let mut elems = vec![first];
            while !matches!(self.peek(), TokenKind::RParen | TokenKind::Eof) {
                elems.push(self.parse_expr(0));
                if !self.eat(&TokenKind::Comma) { break; }
            }
            self.eat(&TokenKind::RParen);
            Expr { kind: ExprKind::Tuple(elems), span }
        } else {
            self.eat(&TokenKind::RParen);
            first
        }
    }

    fn parse_bracket_expr(&mut self) -> Expr {
        let span = self.current_span();
        self.advance(); // `[`
        if matches!(self.peek(), TokenKind::RBracket) {
            self.advance();
            return Expr { kind: ExprKind::VecLit(vec![]), span };
        }
        // Check for matrix: `[[row]; [row]]`
        if matches!(self.peek(), TokenKind::LBracket) {
            return self.parse_matrix_lit(span);
        }
        let mut elems = Vec::new();
        while !matches!(self.peek(), TokenKind::RBracket | TokenKind::Eof) {
            elems.push(self.parse_expr(0));
            if !self.eat(&TokenKind::Comma) { break; }
        }
        self.eat(&TokenKind::RBracket);
        Expr { kind: ExprKind::VecLit(elems), span }
    }

    fn parse_matrix_lit(&mut self, span: Span) -> Expr {
        let mut rows = Vec::new();
        while matches!(self.peek(), TokenKind::LBracket) {
            self.advance();
            let mut row = Vec::new();
            while !matches!(self.peek(), TokenKind::RBracket | TokenKind::Eof) {
                row.push(self.parse_expr(0));
                if !self.eat(&TokenKind::Comma) { break; }
            }
            self.eat(&TokenKind::RBracket);
            rows.push(row);
            if !self.eat(&TokenKind::Semicolon) { break; }
        }
        self.eat(&TokenKind::RBracket);
        Expr { kind: ExprKind::MatrixLit(rows), span }
    }

    fn parse_if_expr(&mut self) -> Expr {
        let span = self.current_span();
        self.advance(); // `if`
        let mut branches = Vec::new();
        let cond = self.parse_expr(0);
        let body = self.parse_block();
        branches.push((cond, body));
        let mut else_block = None;
        loop {
            if self.eat(&TokenKind::Elif) {
                let cond = self.parse_expr(0);
                let body = self.parse_block();
                branches.push((cond, body));
            } else if self.eat(&TokenKind::Else) {
                else_block = Some(self.parse_block());
                break;
            } else {
                break;
            }
        }
        Expr { kind: ExprKind::If { branches, else_block }, span }
    }

    fn parse_match_expr(&mut self) -> Expr {
        let span = self.current_span();
        self.advance(); // `match`
        let scrutinee = Box::new(self.parse_expr(0));
        self.expect(&TokenKind::LBrace, "`{`");
        let mut arms = Vec::new();
        while !matches!(self.peek(), TokenKind::RBrace | TokenKind::Eof) {
            let arm_span = self.current_span();
            let pattern = self.parse_pattern();
            let guard = if self.eat(&TokenKind::Ident(String::from("if"))) {
                Some(Box::new(self.parse_expr(0)))
            } else { None };
            self.expect(&TokenKind::FatArrow, "`=>`");
            let body = Box::new(self.parse_expr(0));
            self.eat(&TokenKind::Comma);
            arms.push(MatchArm { pattern, guard, body, span: arm_span });
        }
        self.eat(&TokenKind::RBrace);
        Expr { kind: ExprKind::Match { scrutinee, arms }, span }
    }

    fn parse_closure(&mut self) -> Expr {
        let span = self.current_span();
        self.advance(); // first `|`
        let mut params = Vec::new();
        while !matches!(self.peek(), TokenKind::Pipe | TokenKind::Eof) {
            let name = self.expect_ident();
            let ty = if self.eat(&TokenKind::Colon) { Some(self.parse_type()) } else { None };
            params.push(ClosureParam { name, ty });
            if !self.eat(&TokenKind::Comma) { break; }
        }
        self.eat(&TokenKind::Pipe);
        let return_type = if self.eat(&TokenKind::Arrow) { Some(self.parse_type()) } else { None };
        let body = Box::new(self.parse_expr(0));
        Expr { kind: ExprKind::Closure { params, return_type, body }, span }
    }

    // ── Pattern parsing ───────────────────────────────────────────────────

    fn parse_pattern(&mut self) -> Pattern {
        match self.peek().clone() {
            TokenKind::Ident(s) if s == "_" => { self.advance(); Pattern::Wildcard }
            TokenKind::Ident(_) => {
                let name = self.expect_ident();
                if matches!(self.peek(), TokenKind::LBrace) {
                    // Struct pattern
                    self.advance();
                    let mut fields = Vec::new();
                    while !matches!(self.peek(), TokenKind::RBrace | TokenKind::Eof) {
                        let fname = self.expect_ident();
                        let pat = if self.eat(&TokenKind::Colon) {
                            self.parse_pattern()
                        } else {
                            Pattern::Binding(fname.clone())
                        };
                        fields.push((fname, pat));
                        if !self.eat(&TokenKind::Comma) { break; }
                    }
                    self.eat(&TokenKind::RBrace);
                    Pattern::Struct { name: vec![name], fields }
                } else if matches!(self.peek(), TokenKind::LParen) {
                    self.advance();
                    let mut inner = Vec::new();
                    while !matches!(self.peek(), TokenKind::RParen | TokenKind::Eof) {
                        inner.push(self.parse_pattern());
                        if !self.eat(&TokenKind::Comma) { break; }
                    }
                    self.eat(&TokenKind::RParen);
                    Pattern::TupleVariant { path: vec![name], inner }
                } else {
                    Pattern::Binding(name)
                }
            }
            TokenKind::IntLiteral(v) => { let v = v; self.advance(); Pattern::Lit(Lit::Int(v)) }
            TokenKind::StringLiteral(s) => { let s = s.clone(); self.advance(); Pattern::Lit(Lit::String(s)) }
            TokenKind::True  => { self.advance(); Pattern::Lit(Lit::Bool(true))  }
            TokenKind::False => { self.advance(); Pattern::Lit(Lit::Bool(false)) }
            TokenKind::Nil   => { self.advance(); Pattern::Lit(Lit::Nil) }
            _ => { Pattern::Wildcard }
        }
    }

    // ── Type parsing ──────────────────────────────────────────────────────

    fn parse_type(&mut self) -> TyAnnotation {
        match self.peek().clone() {
            TokenKind::TyInt    => { self.advance(); TyAnnotation::new(TyKind::Int) }
            TokenKind::TyFloat  => { self.advance(); TyAnnotation::new(TyKind::Float) }
            TokenKind::TyBool   => { self.advance(); TyAnnotation::new(TyKind::Bool) }
            TokenKind::TyString => { self.advance(); TyAnnotation::new(TyKind::String) }
            TokenKind::TyNil    => { self.advance(); TyAnnotation::new(TyKind::Nil) }
            TokenKind::TyVec    => {
                self.advance();
                let inner = if self.eat(&TokenKind::Lt) {
                    let t = self.parse_type();
                    self.eat(&TokenKind::Gt);
                    t
                } else { TyAnnotation::new(TyKind::Infer) };
                TyAnnotation::new(TyKind::Vec(Box::new(inner)))
            }
            TokenKind::TyMap    => {
                self.advance();
                if self.eat(&TokenKind::Lt) {
                    let k = self.parse_type();
                    self.eat(&TokenKind::Comma);
                    let v = self.parse_type();
                    self.eat(&TokenKind::Gt);
                    TyAnnotation::new(TyKind::Map(Box::new(k), Box::new(v)))
                } else {
                    TyAnnotation::new(TyKind::Map(
                        Box::new(TyAnnotation::new(TyKind::Infer)),
                        Box::new(TyAnnotation::new(TyKind::Infer)),
                    ))
                }
            }
            TokenKind::TyMatrix => {
                self.advance();
                let inner = if self.eat(&TokenKind::Lt) {
                    let t = self.parse_type();
                    self.eat(&TokenKind::Gt);
                    t
                } else { TyAnnotation::new(TyKind::Float) };
                TyAnnotation::new(TyKind::Matrix(Box::new(inner)))
            }
            TokenKind::Question => {
                self.advance();
                TyAnnotation::new(TyKind::Optional(Box::new(self.parse_type())))
            }
            TokenKind::Fn => {
                self.advance();
                let is_async = false;
                self.eat(&TokenKind::LParen);
                let mut params = Vec::new();
                while !matches!(self.peek(), TokenKind::RParen | TokenKind::Eof) {
                    params.push(self.parse_type());
                    if !self.eat(&TokenKind::Comma) { break; }
                }
                self.eat(&TokenKind::RParen);
                self.eat(&TokenKind::Arrow);
                let ret = Box::new(self.parse_type());
                TyAnnotation::new(TyKind::Fn { params, ret, is_async })
            }
            TokenKind::LParen => {
                self.advance();
                let mut tys = Vec::new();
                while !matches!(self.peek(), TokenKind::RParen | TokenKind::Eof) {
                    tys.push(self.parse_type());
                    if !self.eat(&TokenKind::Comma) { break; }
                }
                self.eat(&TokenKind::RParen);
                TyAnnotation::new(TyKind::Tuple(tys))
            }
            TokenKind::Ident(name) => {
                let name = SmolStr::new(&name);
                self.advance();
                // Check for generics
                let generics = if matches!(self.peek(), TokenKind::Lt) {
                    self.advance();
                    let mut gs = Vec::new();
                    while !matches!(self.peek(), TokenKind::Gt | TokenKind::Eof) {
                        gs.push(self.parse_type());
                        if !self.eat(&TokenKind::Comma) { break; }
                    }
                    self.eat(&TokenKind::Gt);
                    gs
                } else { vec![] };
                TyAnnotation::new(TyKind::Named { name, generics })
            }
            _ => TyAnnotation::new(TyKind::Infer),
        }
    }

    fn parse_generics(&mut self) -> Vec<GenericParam> {
        if !self.eat(&TokenKind::Lt) { return vec![]; }
        let mut params = Vec::new();
        while !matches!(self.peek(), TokenKind::Gt | TokenKind::Eof) {
            let name = self.expect_ident();
            let mut bounds = Vec::new();
            if self.eat(&TokenKind::Colon) {
                loop {
                    bounds.push(self.parse_type());
                    if !self.eat(&TokenKind::Plus) { break; }
                }
            }
            params.push(GenericParam {
                name,
                bounds,
                kind: GenericParamKind::Type,
            });
            if !self.eat(&TokenKind::Comma) { break; }
        }
        self.eat(&TokenKind::Gt);
        params
    }
}
