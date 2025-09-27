//! Lexer for the logik DSL.

use core::fmt;
use std::fmt::Display;
use std::ops::Range;

use logos::Logos;

/// Tokens types for the logik DSL.
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\f]+")]
#[logos(skip r"//[^\n]*")]
pub enum TokenType {
    /// Or logical operator
    #[token("or")]
    Or,

    /// And logical operator
    #[token("and")]
    And,

    /// Not logical operator
    #[token("not")]
    Not,

    /// Boolean true
    #[token("true")]
    True,

    /// Boolean false
    #[token("false")]
    False,

    /// Left parenthesis for grouping
    #[token("(")]
    LParen,

    /// Right parenthesis for grouping
    #[token(")")]
    RParen,

    /// An implication (->)
    #[token("->")]
    Implication,

    /// An equivalent (<->)
    #[token("<->")]
    Equivalent,

    /// An identifier (variable name)
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    /// A new line character
    #[token("\n")]
    NewLine,
}

/// A token with its type and span in the source code.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The type of the token
    pub token_type: TokenType,

    /// The span (start and end indices) of the token in the source code
    pub span: Range<usize>,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Or => write!(f, "Or"),
            TokenType::And => write!(f, "And"),
            TokenType::Not => write!(f, "Not"),
            TokenType::True => write!(f, "True"),
            TokenType::False => write!(f, "False"),
            TokenType::LParen => write!(f, "Left Parenthesis"),
            TokenType::RParen => write!(f, "Right Parenthesis"),
            TokenType::Implication => write!(f, "Implication (->)"),
            TokenType::Equivalent => write!(f, "BiConditional (<->)"),
            TokenType::Identifier(name) => write!(f, "Identifier ({})", name),
            TokenType::NewLine => write!(f, "New line"),
        }
    }
}
