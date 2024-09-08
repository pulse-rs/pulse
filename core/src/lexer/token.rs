use crate::ast::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    Identifier(String),
    Keyword(Keyword),
    Numeric(NumericLiteral),
    StringLiteral(String),
    BooleanLiteral(bool),
    Operator(Separator),
    Separator(char),
    Comment(String),
    Whitespace,
    Newline,
    EOF,
    Bad,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumericLiteral {
    Integer(i64),
    Float(f64),
}
