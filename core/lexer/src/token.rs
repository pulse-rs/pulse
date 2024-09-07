use pulse_ast::position::Span;

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
pub enum Keyword {
    Let,
    Const,
    If,
    Else,
    While,
    For,
    Function,
    Return,
    Break,
    Continue,
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Keyword(String),
    NumericLiteral(NumericLiteral),
    StringLiteral(String),
    BooleanLiteral(bool),
    Operator(String),
    Delimiter(char),
    Comment(String),
    Whitespace,
    Newline,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumericLiteral {
    Integer(i64),
    Float(f64),
}
