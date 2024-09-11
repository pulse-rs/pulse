mod counter;

use crate::ast::function::{Body, FunctionParameter, FunctionType, TypeAnnotation};
use crate::ast::item::ItemKind;
use crate::ast::stmt::Stmt;
use crate::ast::{item::Item, Ast, ID};
use crate::error::error::Error::ParseError;
use crate::global_context::GlobalContext;
use crate::lexer::token::{Keyword, Operator, Separator, Token, TokenKind};
use crate::parser::counter::Counter;
use crate::types::{parse_type, Type};
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
                let statement_id = self.parse_statement()?;
                self.ast.new_item(ItemKind::Stmt(statement_id));
                self.consume();

                Ok(())
            }
        }
    }

    pub fn parse_function_type(&mut self) -> Result<Option<FunctionType>> {
        if self.current().kind == TokenKind::Separator(Separator::Arrow) {
            let arrow = self.check(TokenKind::Separator(Separator::Arrow))?.clone();
            let return_type = self.check(TokenKind::Identifier)?;

            return Ok(Some(FunctionType {
                arrow,
                type_name: return_type.clone(),
            }));
        }
        Ok(None)
    }

    pub fn parse_type_annotation(&mut self) -> Result<TypeAnnotation> {
        let colon = self.check(TokenKind::Separator(Separator::Colon))?.clone();
        let type_name = self.check(TokenKind::Identifier)?.clone();

        Ok(TypeAnnotation { colon, type_name })
    }

    pub fn parse_optional_type_annotation(&mut self) -> Result<Option<TypeAnnotation>> {
        if self.current().kind == TokenKind::Separator(Separator::Colon) {
            Ok(Some(self.parse_type_annotation()?))
        } else {
            Ok(None)
        }
    }

    pub fn parse_params(&mut self) -> Result<Vec<FunctionParameter>> {
        if self.current().kind != TokenKind::Separator(Separator::LeftParen) {
            return Ok(vec![]);
        }
        self.check(TokenKind::Separator(Separator::LeftParen))?;

        let mut params = vec![];
        while self.current().kind != TokenKind::Separator(Separator::RightParen) && !self.is_eof() {
            params.push(FunctionParameter {
                identifier: self.check(TokenKind::Identifier)?.clone(),
                type_annotation: self.parse_type_annotation()?,
            });

            if self.current().kind == TokenKind::Separator(Separator::Comma) {
                self.check(TokenKind::Separator(Separator::Comma))?;
            }
        }

        self.check(TokenKind::Separator(Separator::RightParen))?;
        Ok(params)
    }

    pub fn parse_statement(&mut self) -> Result<ID> {
        let id = match self.current().kind {
            TokenKind::Keyword(Keyword::Let) => self.parse_let()?,
            _ => 0,
        };

        self.consume_if(TokenKind::Separator(Separator::SemiColon));

        Ok(id)
    }

    fn parse_expression(&mut self) -> Result<ID> {
        unimplemented!()
    }

    pub fn parse_let(&mut self) -> Result<ID> {
        self.check(TokenKind::Keyword(Keyword::Let))?;
        let indent = self.check(TokenKind::Identifier)?.clone();
        let type_annotation = self.parse_optional_type_annotation()?;

        self.check(TokenKind::Operator(Operator::Equals))?;
        let expression = self.parse_expression()?;

        Ok(self.ast.let_stmt(indent, expression, type_annotation))
    }

    pub fn parse_function(&mut self) -> Result<&Item> {
        let func_keyword = self.check(TokenKind::Keyword(Keyword::Fn))?.clone();
        let identifier = self.check(TokenKind::Identifier)?.clone();
        let params = self.parse_params()?;
        let return_type = self.parse_function_type()?;

        let open_brace = self
            .check(TokenKind::Separator(Separator::OpenBrace))?
            .clone();

        let mut body = vec![];

        while self.current().kind != TokenKind::Separator(Separator::CloseBrace) && !self.is_eof() {
            body.push(self.parse_statement()?);
        }

        let close_brace = self
            .check(TokenKind::Separator(Separator::CloseBrace))?
            .clone();

        let mut new_params = vec![];
        for param in &params {
            let new_type = parse_type(&param.type_annotation.type_name, &self.content)?;
            let id = {
                self.global_scope
                    .add_global_variable(param.identifier.span.literal.clone(), new_type.clone())?
            };
            new_params.push(id);
        }

        let body = Body::new(open_brace, body, close_brace);

        let typ = match return_type {
            Some(ref rt) => parse_type(&rt.type_name, &self.content)?,
            None => Type::Void,
        };

        let func = self.global_scope.new_function(
            identifier.clone(),
            body.clone(),
            new_params,
            typ,
            &self.content,
        )?;

        self.ast
            .new_func_item(func_keyword, identifier, params, body, return_type, func)
    }

    fn is_eof(&self) -> bool {
        self.current().kind == TokenKind::Eof
    }

    fn consume(&self) -> &Token {
        self.current.increment();
        self.peek(-1)
    }

    fn consume_if(&self, kind: TokenKind) -> Option<&Token> {
        if self.current().kind == kind {
            Some(self.consume())
        } else {
            None
        }
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
