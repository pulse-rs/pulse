use crate::ast::stmt::StmtKind;
use crate::ast::{Ast, ID};
use crate::lexer::token::Token;
use crate::types::Type;

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<ID>,
    pub name: String,
    pub body: Body,
    pub return_type: Type,
}

#[derive(Debug, Clone)]
pub struct FunctionType {
    pub arrow: Token,
    pub type_name: Token,
}

impl FunctionType {
    pub fn new(arrow: Token, type_name: Token) -> Self {
        Self { arrow, type_name }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub func_keyword: Token,
    pub identifier: Token,
    pub parameters: Vec<ID>,
    pub body: Body,
    pub return_type: Option<FunctionType>,
    pub id: ID,
}

#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    pub colon: Token,
    pub type_name: Token,
}

#[derive(Debug, Clone)]
pub struct FunctionParameter {
    pub identifier: Token,
    pub type_annotation: TypeAnnotation,
}

#[derive(Debug, Clone)]
pub struct Body {
    pub opening_brace: Token,
    pub stmts: Vec<ID>,
    pub closing_brace: Token,
}

impl Body {
    pub fn new(opening_brace: Token, stmts: Vec<ID>, closing_brace: Token) -> Self {
        Self {
            opening_brace,
            stmts,
            closing_brace,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &ID> {
        self.stmts.iter()
    }

    pub fn ty(&self, ast: &Ast) -> Option<Type> {
        get_type_of_last_expr(self.stmts.clone(), ast)
    }
}

pub fn get_type_of_last_expr(body: Vec<ID>, ast: &Ast) -> Option<Type> {
    body.last().and_then(|stmt| {
        let stmt = ast.query_stmt(*stmt);
        if let StmtKind::Expr(expr_id) = &stmt.kind {
            Some(ast.query_expr(*expr_id).ty.clone())
        } else {
            None
        }
    })
}
