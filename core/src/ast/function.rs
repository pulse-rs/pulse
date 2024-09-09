use crate::ast::ID;
use crate::lexer::token::Token;

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
    pub parameters: Vec<FuncDeclParameter>,
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
pub struct FuncDeclParameter {
    pub identifier: Token,
    pub type_annotation: TypeAnnotation,
}

#[derive(Debug, Clone)]
pub struct Body {
    pub opening_brace: Token,
    pub stmts: Vec<ID>,
    pub closing_brace: Token,
}
