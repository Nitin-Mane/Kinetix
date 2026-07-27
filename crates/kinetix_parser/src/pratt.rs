//! Pratt (top-down operator precedence) expression parser for Kinetix.
//!
//! Operator precedence table (highest to lowest):
//!
//! | Level | Operators                  | Associativity |
//! |-------|----------------------------|---------------|
//! | 13    | Unary `-`, `!`, `~`        | Prefix        |
//! | 12    | `**` (power)               | Right         |
//! | 11    | `.*`, `./`                 | Left          |
//! | 10    | `*`, `/`, `%`              | Left          |
//! | 9     | `+`, `-`                   | Left          |
//! | 8     | `<<`, `>>`                 | Left          |
//! | 7     | `&`                        | Left          |
//! | 6     | `^`                        | Left          |
//! | 5     | `|`                        | Left          |
//! | 4     | `==`, `!=`, `<`, `<=`, `>`, `>=` | Left    |
//! | 3     | `&&`                       | Left          |
//! | 2     | `||`                       | Left          |
//! | 1     | `..`, `..=`                | Non-assoc     |
//! | 0     | `=`, `+=`, `-=`, etc.      | Right         |

use kinetix_ast::expr::BinOp;
use kinetix_lexer::TokenKind;

/// Binding power for Pratt parsing.  Returns `(left_bp, right_bp)` for
/// infix operators or `None` if the token is not an infix operator.
pub fn infix_binding_power(tok: &TokenKind) -> Option<(u8, u8)> {
    let bp = match tok {
        // Assignment (right-assoc)
        TokenKind::Eq
        | TokenKind::PlusEq
        | TokenKind::MinusEq
        | TokenKind::StarEq
        | TokenKind::SlashEq
        | TokenKind::PercentEq => (1, 2),

        // Range (non-assoc, handled specially)
        TokenKind::DotDot | TokenKind::DotDotEq => (3, 4),

        // Logical OR
        TokenKind::PipePipe => (5, 6),
        // Logical AND
        TokenKind::AmpAmp   => (7, 8),

        // Comparison (non-assoc)
        TokenKind::EqEq
        | TokenKind::BangEq
        | TokenKind::Lt
        | TokenKind::LtEq
        | TokenKind::Gt
        | TokenKind::GtEq    => (9, 10),

        // Bitwise OR
        TokenKind::Pipe      => (11, 12),
        // Bitwise XOR
        TokenKind::Caret     => (13, 14),
        // Bitwise AND
        TokenKind::Amp       => (15, 16),

        // Shifts
        TokenKind::LtLt | TokenKind::GtGt => (17, 18),

        // Additive
        TokenKind::Plus | TokenKind::Minus => (19, 20),

        // Multiplicative
        TokenKind::Star | TokenKind::Slash | TokenKind::Percent => (21, 22),

        // Power (right-assoc)
        TokenKind::StarStar  => (24, 23),

        _ => return None,
    };
    Some(bp)
}

/// Binding power for prefix unary operators.
pub fn prefix_binding_power(tok: &TokenKind) -> Option<u8> {
    match tok {
        TokenKind::Minus | TokenKind::Bang | TokenKind::Tilde => Some(25),
        _ => None,
    }
}

/// Binding power for postfix operators (call, index, field, method, await).
pub fn postfix_binding_power(tok: &TokenKind) -> Option<u8> {
    match tok {
        TokenKind::LParen | TokenKind::LBracket | TokenKind::Dot => Some(27),
        _ => None,
    }
}

/// Map a binary operator token to the AST `BinOp` enum.
pub fn token_to_binop(tok: &TokenKind) -> Option<BinOp> {
    Some(match tok {
        TokenKind::Plus      => BinOp::Add,
        TokenKind::Minus     => BinOp::Sub,
        TokenKind::Star      => BinOp::Mul,
        TokenKind::Slash     => BinOp::Div,
        TokenKind::Percent   => BinOp::Rem,
        TokenKind::StarStar  => BinOp::Pow,
        TokenKind::EqEq      => BinOp::Eq,
        TokenKind::BangEq    => BinOp::Ne,
        TokenKind::Lt        => BinOp::Lt,
        TokenKind::LtEq      => BinOp::Le,
        TokenKind::Gt        => BinOp::Gt,
        TokenKind::GtEq      => BinOp::Ge,
        TokenKind::AmpAmp    => BinOp::And,
        TokenKind::PipePipe  => BinOp::Or,
        TokenKind::Amp       => BinOp::BitAnd,
        TokenKind::Pipe      => BinOp::BitOr,
        TokenKind::Caret     => BinOp::BitXor,
        TokenKind::LtLt      => BinOp::Shl,
        TokenKind::GtGt      => BinOp::Shr,
        TokenKind::DotDot    => BinOp::Range,
        TokenKind::DotDotEq  => BinOp::RangeInc,
        _ => return None,
    })
}
