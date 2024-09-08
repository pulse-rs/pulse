use crate::ast::span::TextSpan;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Let,
    If,
    Else,
    True,
    False,
    While,
    Func,
    Return,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equals,
    Ampersand,
    Pipe,
    Caret,
    DoubleAsterisk,
    Percent,
    Tilde,
    GreaterThan,
    LessThan,
    GreaterThanEquals,
    LessThanEquals,
    EqualsEquals,
    BangEquals,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Separator {
    LeftParen,
    RightParen,
    OpenBrace,
    CloseBrace,
    Comma,
    Colon,
    SemiColon,
    Arrow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(i64),
    Operator(Operator),
    Keyword(Keyword),
    Separator(Separator),
    Bad,
    Whitespace,
    Identifier,
    Eof,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Number(_) => write!(f, "Number"),
            TokenKind::Operator(op) => write!(f, "{:?}", op),
            TokenKind::Keyword(kw) => write!(f, "{:?}", kw),
            TokenKind::Separator(sep) => write!(f, "{:?}", sep),
            TokenKind::Bad => write!(f, "Bad"),
            TokenKind::Whitespace => write!(f, "Whitespace"),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::Eof => write!(f, "Eof"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}
