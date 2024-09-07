use crate::stream::TokenStream;
use crate::token::{Token, TokenKind};
use pulse_ast::position::Span;
use std::io::{Error, ErrorKind};

mod stream;
mod token;

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

    pub fn lex(&mut self) -> Result<Option<Token>, Error> {
        let mut start = self.stream.pos();
        let Some(mut next_ch) = self.stream.next_char()? else {
            return Ok(None);
        };

        if is_whitespace(next_ch) {
            loop {
                start = self.stream.pos();
                let Some(next) = self.stream.next_char()? else {
                    return Ok(None);
                };
                if !is_whitespace(next) {
                    next_ch = next;
                    break;
                }
            }
        }

        match char::try_from(next_ch) {
            Ok(ch) => Ok(Some(Token::new(
                TokenKind::Identifier(ch.to_string()),
                Span::new(start, self.stream.pos()),
            ))),
            Err(_) => Err(Error::new(
                ErrorKind::InvalidData,
                format!("invalid character: {}", next_ch),
            )),
        }
    }
}

const fn is_whitespace(ch: u32) -> bool {
    matches!(
        ch,
        0x0020 | 0x0009 | 0x000B | 0x000C | 0x00A0 | 0xFEFF | 0x1680 | 0x2000
            ..=0x200A | 0x202F | 0x205F | 0x3000
    )
}
