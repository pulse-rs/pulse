use crate::ast::function::{Body, TypeAnnotation};
use crate::ast::ID;
use crate::lexer::token::Token;

#[derive(Debug, Clone)]
pub struct LetStmt {
    pub identifier: Token,
    pub initializer: ID,
    pub type_annotation: Option<TypeAnnotation>,
    pub variable_id: ID,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    Expr(ID),
    Let(LetStmt),
    While(WhileStmt),
    Return(ReturnStmt),
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub while_keyword: Token,
    pub condition: ID,
    pub body: Body,
}

#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub return_keyword: Token,
    pub return_value: Option<ID>,
}

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub id: ID,
}

impl Stmt {
    pub fn new(kind: StmtKind, id: ID) -> Self {
        Stmt { kind, id }
    }
}
