mod counter;

use crate::ast::item::ItemKind;
use crate::ast::{item::Item, Ast};
use crate::error::error::Error::ParseError;
use crate::global_context::GlobalContext;
use crate::lexer::token::{Keyword, Token, TokenKind};
use crate::parser::counter::Counter;
use crate::Result;
use colored::Colorize;

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: Counter,
    ast: &'a mut Ast,
    global_scope: &'a mut GlobalContext,
    content: String,
}

impl<'a> Parser<'a> {
    pub fn new(
        tokens: Vec<Token>,
        content: String,
        ast: &'a mut Ast,
        global_scope: &'a mut GlobalContext,
    ) -> Self {
        Self {
            tokens,
            current: Counter::new(),
            ast,
            global_scope,
            content,
        }
    }
}

impl Parser<'_> {
    pub fn parse(&mut self) -> Result<()> {
        log::debug!("Starting parsing process");

        while !self.is_eof() {
            self.parse_item()?
        }

        Ok(())
    }

    pub fn parse_item(&mut self) -> Result<()> {
        let kind = &self.current().kind;

        match kind {
            TokenKind::Keyword(Keyword::Fn) => {
                self.parse_function()?;
                Ok(())
            }
            _ => {
                // let statement_id = self.parse_statement();
                // self.ast.new_item(ItemKind::Stmt(statement_id));
                self.consume();

                Ok(())
            }
        }
    }

    pub fn parse_function(&mut self) -> Result<&Item> {
        let func_keyword = self.check(TokenKind::Keyword(Keyword::Fn))?;
        let identifier = self.check(TokenKind::Identifier)?;

        let id = self.ast.new_item(ItemKind::Function);

        Ok(id)
    }

    fn is_eof(&self) -> bool {
        self.current().kind == TokenKind::Eof
    }

    fn consume(&self) -> &Token {
        self.current.increment();
        self.peek(-1)
    }

    fn check(&self, kind: TokenKind) -> Result<&Token> {
        let token = self.consume();

        if token.kind == kind {
            Ok(token)
        } else {
            Err(ParseError(
                format!(
                    "Expected {}, found {}",
                    kind.to_string().cyan(),
                    token.kind.to_string().cyan()
                ),
                token.clone().span,
                self.content.clone(),
            ))
        }
    }

    fn peek(&self, offset: isize) -> &Token {
        let mut index = (self.current.get_value() as isize + offset) as usize;
        if index >= self.tokens.len() {
            index = self.tokens.len() - 1;
        }
        self.tokens.get(index).unwrap()
    }

    fn current(&self) -> &Token {
        self.peek(0)
    }
}
