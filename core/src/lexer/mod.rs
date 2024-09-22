use crate::ast::position::Position;
use crate::ast::span::TextSpan;
use crate::lexer::token::{Keyword, Operator, Separator, Token, TokenKind};

pub mod token;

pub struct Lexer<'a> {
    input: &'a str,
    pub pos: Position,
    pub current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0,
            pos: Position::new(0, 0, 0),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos >= self.input.len() {
            self.current_pos += 1;
            return Some(Token::new(
                TokenKind::Eof,
                TextSpan::new(self.pos, self.pos, "\0".to_string()),
            ));
        }

        let c = self.current_char()?;
        let start_pos = self.pos;

        let kind = if Self::is_number_start(&c) {
            TokenKind::Number(self.consume_number())
        } else if Self::is_string_start(&c) {
            self.consume();
            TokenKind::String(self.consume_string())
        } else if Self::is_whitespace(&c) {
            self.consume();
            TokenKind::Whitespace
        } else if Self::is_identifier_start(&c) {
            match self.consume_identifier().as_str() {
                "let" => TokenKind::Keyword(Keyword::Let),
                "if" => TokenKind::Keyword(Keyword::If),
                "else" => TokenKind::Keyword(Keyword::Else),
                "true" => TokenKind::Keyword(Keyword::True),
                "false" => TokenKind::Keyword(Keyword::False),
                "while" => TokenKind::Keyword(Keyword::While),
                "fn" => TokenKind::Keyword(Keyword::Fn),
                "return" => TokenKind::Keyword(Keyword::Return),
                _ => TokenKind::Identifier,
            }
        } else {
            self.consume_punctuation()
        };

        let end_pos = self.pos;
        let literal = self.input[start_pos.index..end_pos.index].to_string();

        Some(Token::new(kind, TextSpan::new(start_pos, end_pos, literal)))
    }

    fn consume_punctuation(&mut self) -> TokenKind {
        let c = self.consume().unwrap();
        match c {
            '+' => TokenKind::Operator(Operator::Plus),
            '-' => self.lex_potential_double_char_operator(
                '>',
                TokenKind::Operator(Operator::Minus),
                TokenKind::Separator(Separator::Arrow),
            ),
            '*' => self.lex_potential_double_char_operator(
                '*',
                TokenKind::Operator(Operator::Asterisk),
                TokenKind::Operator(Operator::DoubleAsterisk),
            ),
            '%' => TokenKind::Operator(Operator::Percent),
            '/' => TokenKind::Operator(Operator::Slash),
            '(' => TokenKind::Separator(Separator::LeftParen),
            ')' => TokenKind::Separator(Separator::RightParen),
            '=' => self.lex_potential_double_char_operator(
                '=',
                TokenKind::Operator(Operator::Equals),
                TokenKind::Operator(Operator::EqualsEquals),
            ),
            '&' => TokenKind::Operator(Operator::Ampersand),
            '|' => TokenKind::Operator(Operator::Pipe),
            '^' => TokenKind::Operator(Operator::Caret),
            '~' => TokenKind::Operator(Operator::Tilde),
            '>' => self.lex_potential_double_char_operator(
                '=',
                TokenKind::Operator(Operator::GreaterThan),
                TokenKind::Operator(Operator::GreaterThanEquals),
            ),
            '<' => self.lex_potential_double_char_operator(
                '=',
                TokenKind::Operator(Operator::LessThan),
                TokenKind::Operator(Operator::LessThanEquals),
            ),
            '!' => self.lex_potential_double_char_operator(
                '=',
                TokenKind::Bad,
                TokenKind::Operator(Operator::BangEquals),
            ),
            '{' => TokenKind::Separator(Separator::OpenBrace),
            '}' => TokenKind::Separator(Separator::CloseBrace),
            ',' => TokenKind::Separator(Separator::Comma),
            ':' => self.lex_potential_double_char_operator(
                ':',
                TokenKind::Separator(Separator::Colon),
                TokenKind::Separator(Separator::Scope),
            ),
            ';' => TokenKind::Separator(Separator::SemiColon),
            '"' => TokenKind::Separator(Separator::Quote),
            _ => TokenKind::Bad,
        }
    }

    fn lex_potential_double_char_operator(
        &mut self,
        expected: char,
        one_char_kind: TokenKind,
        double_char_kind: TokenKind,
    ) -> TokenKind {
        if let Some(next) = self.current_char() {
            if next == expected {
                self.consume();
                double_char_kind
            } else {
                one_char_kind
            }
        } else {
            one_char_kind
        }
    }

    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }

    fn is_string_start(c: &char) -> bool {
        c == &'\"'
    }

    fn consume_string(&mut self) -> String {
        let mut string = String::new();
        while let Some(c) = self.current_char() {
            if c != '\"' {
                self.consume().unwrap();
                string.push(c);
            } else {
                self.consume();
                break;
            }
        }
        string
    }

    fn is_identifier_start(c: &char) -> bool {
        c.is_alphabetic() || c == &'_'
    }

    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    fn consume(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            return None;
        }
        let c = self.current_char();
        self.current_pos += 1;

        self.update_position(c?);

        c
    }

    fn consume_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(c) = self.current_char() {
            if Self::is_identifier_start(&c) {
                self.consume().unwrap();
                identifier.push(c);
            } else {
                break;
            }
        }
        identifier
    }

    fn consume_number(&mut self) -> i64 {
        let mut number: i64 = 0;
        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                self.consume().unwrap();
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            } else {
                break;
            }
        }
        number
    }

    fn update_position(&mut self, c: char) {
        if c == '\n' {
            self.pos.line += 1;
            self.pos.column = 0;
        } else {
            self.pos.column += 1;
        }
        self.pos.index += 1;
    }
}
