use crate::ast::span::Span;
use crate::lexer::stream::TokenStream;
use crate::lexer::token::{Keyword, NumericLiteral, Token, TokenKind};

pub mod stream;
pub mod token;

use crate::Result;

#[derive(Debug)]
pub struct Lexer {
    pub stream: TokenStream,
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Self {
            stream: TokenStream::new(&content),
        }
    }

    pub fn lex(&mut self) -> Result<Option<Token>> {
        let mut start = self.stream.pos();
        let Some(mut next_ch) = self.stream.next_char()? else {
            return Ok(None);
        };

        if next_ch.is_whitespace() {
            loop {
                start = self.stream.pos();
                let Some(next) = self.stream.next_char()? else {
                    return Ok(None);
                };
                if !next.is_whitespace() {
                    next_ch = next;
                    break;
                }
            }
        }

        let mut kind: TokenKind = TokenKind::Bad;

        if next_ch.is_digit(10) {
            let value = self.consume_number();

            kind = TokenKind::Numeric(NumericLiteral::Integer(value?));
        } else if Lexer::is_identifier_start(&next_ch) {
            let ident = self.consume_identifier()?;

            kind = match ident.as_str() {
                "let" => TokenKind::Keyword(Keyword::Let),
                "fn" => TokenKind::Keyword(Keyword::Fn),
                "if" => TokenKind::Keyword(Keyword::If),
                "else" => TokenKind::Keyword(Keyword::Else),
                "while" => TokenKind::Keyword(Keyword::While),
                "return" => TokenKind::Keyword(Keyword::Return),
                _ => TokenKind::Identifier(ident),
            };
        } else {
        }

        let end = self.stream.pos();
        let span = Span::new(start, end);

        Ok(Some(Token::new(kind, span)))
    }

    fn is_identifier_start(c: &char) -> bool {
        c.is_alphabetic() || c == &'_'
    }

    fn consume_identifier(&mut self) -> Result<String> {
        let mut ident = String::new();
        while let Ok(ch) = self.stream.peek_char() {
            if let Some(ch) = ch {
                if Lexer::is_identifier_start(&ch) {
                    self.stream.next_char()?;
                    ident.push(ch);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(ident)
    }

    fn consume_number(&mut self) -> Result<i64> {
        let mut value = 0;
        while let Ok(ch) = self.stream.peek_char() {
            if let Some(ch) = ch {
                if ch.is_digit(10) {
                    self.stream.next_char()?;
                    value = value * 10 + ch as i64 - '0' as i64;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(value)
    }
}
