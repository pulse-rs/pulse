mod counter;

use crate::ast::expr::{
    BinOpAssociativity, BinOpKind, BinOperator, ElseBranch, Expr, ExprKind, NumberExpr, UnOpKind,
    UnOperator,
};
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

    pub fn parse_return_statement(&mut self) -> Result<ID> {
        let return_keyword = self.check(TokenKind::Keyword(Keyword::Return))?.clone();
        let expression = if self.current().kind != TokenKind::Separator(Separator::SemiColon) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        Ok(self.ast.return_statement(return_keyword, expression))
    }

    pub fn parse_while_statement(&mut self) -> Result<ID> {
        let while_keyword = self.check(TokenKind::Keyword(Keyword::While))?.clone();
        let condition_expr = self.parse_expression()?;
        let body = self.parse_body()?;
        Ok(self
            .ast
            .while_statement(while_keyword, condition_expr, body))
    }

    pub fn parse_statement(&mut self) -> Result<ID> {
        let id = match self.current().kind {
            TokenKind::Keyword(Keyword::Let) => self.parse_let()?,
            TokenKind::Keyword(Keyword::While) => self.parse_while_statement()?,
            TokenKind::Keyword(Keyword::Return) => self.parse_return_statement()?,
            _ => self.parse_expression_statement()?,
        };

        self.consume_if(TokenKind::Separator(Separator::SemiColon));

        Ok(id)
    }

    fn parse_expression_statement(&mut self) -> Result<ID> {
        let expr = self.parse_expression()?;
        Ok(self.ast.expression_statement(expr))
    }

    fn parse_expression(&mut self) -> Result<ID> {
        self.parse_assignment_expression()
    }

    fn parse_assignment_expression(&mut self) -> Result<ID> {
        if self.current().kind == TokenKind::Identifier
            && self.peek(1).kind == TokenKind::Operator(Operator::Equals)
        {
            let identifier = self.check(TokenKind::Identifier)?.clone();
            let equals = self.check(TokenKind::Operator(Operator::Equals))?.clone();
            let expr = self.parse_expression()?;
            return Ok(self.ast.assignment_expression(identifier, equals, expr).id);
        }
        self.parse_binary_expression()
    }

    pub fn parse_unary_operator(&mut self) -> Option<UnOperator> {
        let token = self.current();
        let kind = match token.kind {
            TokenKind::Operator(Operator::Minus) => Some(UnOpKind::Minus),
            TokenKind::Operator(Operator::Tilde) => Some(UnOpKind::BitwiseNot),
            _ => None,
        };
        kind.map(|kind| UnOperator::new(kind, token.clone()))
    }

    pub fn parse_if_expression(&mut self, if_keyword: Token) -> Result<&Expr> {
        let condition_expr = self.parse_expression()?;
        let then = self.parse_body()?;
        let else_statement = self.parse_optional_else_statement()?;
        Ok(self
            .ast
            .if_expr(if_keyword, condition_expr, then, else_statement))
    }

    pub fn parse_optional_else_statement(&mut self) -> Result<Option<ElseBranch>> {
        if self.current().kind == TokenKind::Keyword(Keyword::Else) {
            let else_keyword = self.check(TokenKind::Keyword(Keyword::Else))?.clone();
            let else_expr = self.parse_body()?;
            return Ok(Some(ElseBranch::new(else_keyword, else_expr)));
        }
        Ok(None)
    }

    pub fn parse_body(&mut self) -> Result<Body> {
        let opening_brace = self
            .check(TokenKind::Separator(Separator::OpenBrace))?
            .clone();
        let mut body = Vec::new();
        while self.current().kind != TokenKind::Separator(Separator::CloseBrace) && !self.is_eof() {
            body.push(self.parse_statement()?);
        }
        let closing_brace = self
            .check(TokenKind::Separator(Separator::CloseBrace))?
            .clone();
        Ok(Body::new(opening_brace, body, closing_brace))
    }

    fn parse_block_expression(&mut self, left_brace: Token) -> Result<&Expr> {
        let mut statements = Vec::new();
        while self.current().kind != TokenKind::Separator(Separator::CloseBrace) && !self.is_eof() {
            statements.push(self.parse_statement()?);
        }
        let right_brace = self
            .check(TokenKind::Separator(Separator::CloseBrace))?
            .clone();
        Ok(self
            .ast
            .block_expression(left_brace, statements, right_brace))
    }

    fn parse_call_expression(&mut self, identifier: Token) -> Result<ID> {
        let left_paren = self
            .check(TokenKind::Separator(Separator::LeftParen))?
            .clone();
        let mut arguments = Vec::new();
        while self.current().kind != TokenKind::Separator(Separator::RightParen) && !self.is_eof() {
            arguments.push(self.parse_expression()?);
            if self.current().kind != TokenKind::Separator(Separator::RightParen) {
                self.check(TokenKind::Separator(Separator::Comma))?;
            }
        }
        let right_paren = self
            .check(TokenKind::Separator(Separator::RightParen))?
            .clone();
        Ok(self
            .ast
            .call_expression(identifier, left_paren, arguments, right_paren)
            .id)
    }

    pub fn parse_primary_expression(&mut self) -> Result<ID> {
        let token = self.consume().clone();
        let id = match token.kind {
            TokenKind::Separator(Separator::OpenBrace) => self.parse_block_expression(token),
            TokenKind::Keyword(Keyword::If) => self.parse_if_expression(token),
            TokenKind::Number(number) => Ok(self.ast.number_expression(token, number)),
            TokenKind::Separator(Separator::LeftParen) => Ok({
                let expr = self.parse_expression()?;
                let left_paren = token;
                let right_paren = self
                    .check(TokenKind::Separator(Separator::LeftParen))?
                    .clone();
                self.ast
                    .parenthesized_expression(left_paren, expr, right_paren)
            }),
            TokenKind::Identifier => {
                if matches!(
                    self.current().kind,
                    TokenKind::Separator(Separator::LeftParen)
                ) {
                    return self.parse_call_expression(token);
                }
                Ok(self.ast.variable_expression(token))
            }
            TokenKind::Keyword(Keyword::True) | TokenKind::Keyword(Keyword::False) => {
                let value = token.kind == TokenKind::Keyword(Keyword::True);

                Ok(self.ast.boolean_expression(token, value))
            }
            _ => {
                //     TODO: handle error
                Err(ParseError(
                    format!("Unexpected token: {}", token.kind.to_string().cyan()),
                    token.span,
                    self.content.clone(),
                ))
            }
        }?
        .id;

        Ok(id)
    }

    pub fn parse_unary_expression(&mut self) -> Result<ID> {
        if let Some(operator) = self.parse_unary_operator() {
            self.consume();
            let operand = self.parse_unary_expression();
            return Ok(self.ast.unary_expr(operator, operand?).id);
        }
        self.parse_primary_expression()
    }

    fn parse_binary_operator(&mut self) -> Option<BinOperator> {
        let token = self.current();
        let kind = match token.kind {
            TokenKind::Operator(Operator::Plus) => Some(BinOpKind::Plus),
            TokenKind::Operator(Operator::Minus) => Some(BinOpKind::Minus),
            TokenKind::Operator(Operator::Asterisk) => Some(BinOpKind::Multiply),
            TokenKind::Operator(Operator::Slash) => Some(BinOpKind::Divide),
            TokenKind::Operator(Operator::Ampersand) => Some(BinOpKind::BitwiseAnd),
            TokenKind::Operator(Operator::Pipe) => Some(BinOpKind::BitwiseOr),
            TokenKind::Operator(Operator::Caret) => Some(BinOpKind::BitwiseXor),
            TokenKind::Operator(Operator::DoubleAsterisk) => Some(BinOpKind::Power),
            TokenKind::Operator(Operator::EqualsEquals) => Some(BinOpKind::Equals),
            TokenKind::Operator(Operator::BangEquals) => Some(BinOpKind::NotEquals),
            TokenKind::Operator(Operator::LessThan) => Some(BinOpKind::LessThan),
            TokenKind::Operator(Operator::LessThanEquals) => Some(BinOpKind::LessThanOrEqual),
            TokenKind::Operator(Operator::GreaterThan) => Some(BinOpKind::GreaterThan),
            TokenKind::Operator(Operator::GreaterThanEquals) => Some(BinOpKind::GreaterThanOrEqual),
            TokenKind::Operator(Operator::Percent) => Some(BinOpKind::Modulo),
            _ => None,
        };
        kind.map(|kind| BinOperator::new(kind, token.clone()))
    }

    pub fn parse_binary_expression_recurse(&mut self, mut left: ID, precedence: u8) -> Result<ID> {
        while let Some(operator) = self.parse_binary_operator() {
            let operator_precedence = operator.precedence();
            if operator_precedence < precedence {
                break;
            }
            self.consume();
            let mut right = self.parse_unary_expression()?;

            while let Some(inner_operator) = self.parse_binary_operator() {
                let greater_precedence = inner_operator.precedence() > operator.precedence();
                let equal_precedence = inner_operator.precedence() == operator.precedence();
                if !(greater_precedence
                    || equal_precedence
                        && inner_operator.associativity() == BinOpAssociativity::Right)
                {
                    break;
                }

                right = self.parse_binary_expression_recurse(
                    right,
                    std::cmp::max(operator.precedence(), inner_operator.precedence()),
                )?;
            }
            left = self.ast.binary_expression(operator, left, right).id;
        }
        Ok(left)
    }

    pub fn parse_binary_expression(&mut self) -> Result<ID> {
        let left = self.parse_unary_expression()?;
        self.parse_binary_expression_recurse(left, 0)
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
