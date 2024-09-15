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
    Fn,
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
    Quote,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(i64),
    String(String),
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
            TokenKind::Number(_) => write!(f, "number"),
            TokenKind::String(_) => write!(f, "string"),
            TokenKind::Operator(op) => {
                let op_str = match op {
                    Operator::Plus => "+",
                    Operator::Minus => "-",
                    Operator::Asterisk => "*",
                    Operator::Slash => "/",
                    Operator::Equals => "=",
                    Operator::Ampersand => "&",
                    Operator::Pipe => "|",
                    Operator::Caret => "^",
                    Operator::DoubleAsterisk => "**",
                    Operator::Percent => "%",
                    Operator::Tilde => "~",
                    Operator::GreaterThan => ">",
                    Operator::LessThan => "<",
                    Operator::GreaterThanEquals => ">=",
                    Operator::LessThanEquals => "<=",
                    Operator::EqualsEquals => "==",
                    Operator::BangEquals => "!=",
                };

                write!(f, "{}", op_str)
            }
            TokenKind::Keyword(kw) => {
                let kw_str = match kw {
                    Keyword::Let => "let",
                    Keyword::If => "if",
                    Keyword::Else => "else",
                    Keyword::True => "true",
                    Keyword::False => "false",
                    Keyword::While => "while",
                    Keyword::Fn => "fn",
                    Keyword::Return => "return",
                };

                write!(f, "{}", kw_str)
            }
            TokenKind::Separator(sep) => {
                let sep_str = match sep {
                    Separator::LeftParen => "(",
                    Separator::RightParen => ")",
                    Separator::OpenBrace => "{",
                    Separator::CloseBrace => "}",
                    Separator::Comma => ",",
                    Separator::Colon => ":",
                    Separator::SemiColon => ";",
                    Separator::Arrow => "->",
                    Separator::Quote => "\"",
                };

                write!(f, "{}", sep_str)
            }
            TokenKind::Bad => write!(f, "Bad"),
            TokenKind::Whitespace => write!(f, "whitespace"),
            TokenKind::Identifier => write!(f, "identifier"),
            TokenKind::Eof => write!(f, "EOF"),
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
