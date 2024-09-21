use crate::ast::function::Body;
use crate::ast::span::TextSpan;
use crate::ast::stmt::StmtKind;
use crate::ast::{Ast, ID};
use crate::lexer::token::Token;
use crate::types::Type;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub callee: Token,
    pub left_paren: Token,
    pub arguments: Vec<ID>,
    pub right_paren: Token,
    pub function_idx: ID,
}

impl CallExpr {
    pub fn function_name(&self) -> &str {
        &self.callee.span.literal
    }
}

#[derive(Debug, Clone)]
pub struct BoolExpr {
    pub value: bool,
    pub token: Token,
}

#[derive(Debug, Clone)]
pub struct AssignExpr {
    pub identifier: Token,
    pub equals: Token,
    pub expression: ID,
    pub variable_idx: ID,
}

#[derive(Debug, Copy, Clone)]
pub enum UnOpKind {
    Minus,
    BitwiseNot,
}

impl Display for UnOpKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnOpKind::Minus => write!(f, "-"),
            UnOpKind::BitwiseNot => write!(f, "~"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnOperator {
    pub(crate) kind: UnOpKind,
    token: Token,
}

impl UnOperator {
    pub fn new(kind: UnOpKind, token: Token) -> Self {
        UnOperator { kind, token }
    }
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub operator: UnOperator,
    pub operand: ID,
}

#[derive(Debug, Clone)]
pub struct VarExpr {
    pub identifier: Token,
    pub variable_idx: ID,
}

impl VarExpr {
    pub fn identifier(&self) -> &str {
        &self.identifier.span.literal
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BinOpKind {
    // Arithmetic
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Modulo,
    // Bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    // Relational
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

impl Display for BinOpKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOpKind::Plus => write!(f, "+"),
            BinOpKind::Minus => write!(f, "-"),
            BinOpKind::Multiply => write!(f, "*"),
            BinOpKind::Divide => write!(f, "/"),
            BinOpKind::Power => write!(f, "**"),
            BinOpKind::Modulo => write!(f, "%"),
            BinOpKind::BitwiseAnd => write!(f, "&"),
            BinOpKind::BitwiseOr => write!(f, "|"),
            BinOpKind::BitwiseXor => write!(f, "^"),
            BinOpKind::Equals => write!(f, "=="),
            BinOpKind::NotEquals => write!(f, "!="),
            BinOpKind::LessThan => write!(f, "<"),
            BinOpKind::LessThanOrEqual => write!(f, "<="),
            BinOpKind::GreaterThan => write!(f, ">"),
            BinOpKind::GreaterThanOrEqual => write!(f, ">="),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOpAssociativity {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct BinOperator {
    pub kind: BinOpKind,
    pub token: Token,
}

impl BinOperator {
    pub fn new(kind: BinOpKind, token: Token) -> Self {
        BinOperator { kind, token }
    }

    pub fn precedence(&self) -> u8 {
        match self.kind {
            BinOpKind::Power => 20,
            BinOpKind::Multiply => 19,
            BinOpKind::Divide => 19,
            BinOpKind::Modulo => 19,
            BinOpKind::Plus => 18,
            BinOpKind::Minus => 18,
            BinOpKind::BitwiseAnd => 17,
            BinOpKind::BitwiseXor => 16,
            BinOpKind::BitwiseOr => 15,
            BinOpKind::Equals => 30,
            BinOpKind::NotEquals => 30,
            BinOpKind::LessThan => 29,
            BinOpKind::LessThanOrEqual => 29,
            BinOpKind::GreaterThan => 29,
            BinOpKind::GreaterThanOrEqual => 29,
        }
    }

    pub fn associativity(&self) -> BinOpAssociativity {
        match self.kind {
            BinOpKind::Power => BinOpAssociativity::Right,
            _ => BinOpAssociativity::Left,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: ID,
    pub operator: BinOperator,
    pub right: ID,
}

#[derive(Debug, Clone)]
pub struct NumberExpr {
    pub number: i64,
    pub token: Token,
}

#[derive(Debug, Clone)]
pub struct StringExpr {
    pub string: String,
    pub token: Token,
}

#[derive(Debug, Clone)]
pub struct ParenthesizedExpr {
    pub left_paren: Token,
    pub inner: ID,
    pub right_paren: Token,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Number(NumberExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Parenthesized(ParenthesizedExpr),
    Variable(VarExpr),
    Assignment(AssignExpr),
    Boolean(BoolExpr),
    Call(CallExpr),
    If(IfExpr),
    Block(BlockExpr),
    Error(TextSpan),
    String(StringExpr),
}

impl ExprKind {
    pub fn is_binary(&self) -> bool {
        matches!(self, ExprKind::Binary(_))
    }
}

#[derive(Debug, Clone)]
pub struct BlockExpr {
    pub left_brace: Token,
    pub stmts: Vec<ID>,
    pub right_brace: Token,
}

impl BlockExpr {
    pub fn returning_expression(&self, ast: &Ast) -> Option<ID> {
        if let Some(last_stmt) = self.stmts.last() {
            let stmt = ast.query_stmt(*last_stmt);
            if let StmtKind::Expr(expr_id) = &stmt.kind {
                return Some(*expr_id);
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct IfExpr {
    pub if_keyword: Token,
    pub condition: ID,
    pub then_branch: Body,
    pub else_branch: Option<ElseBranch>,
}

#[derive(Debug, Clone)]
pub struct ElseBranch {
    pub else_keyword: Token,
    pub body: Body,
}

impl ElseBranch {
    pub fn new(else_keyword: Token, body: Body) -> Self {
        ElseBranch { else_keyword, body }
    }
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub id: ID,
    pub ty: Type,
}

impl Expr {
    pub fn new(kind: ExprKind, id: ID, ty: Type) -> Self {
        Expr { kind, id, ty }
    }

    pub fn span(&self, ast: &Ast) -> TextSpan {
        match &self.kind {
            ExprKind::Number(number) => number.token.span.clone(),
            ExprKind::Boolean(boolean) => boolean.token.span.clone(),
            ExprKind::String(string) => string.token.span.clone(),
            ExprKind::Binary(binary) => {
                let left = ast.query_expr(binary.left).span(ast);
                let operator = binary.operator.token.span.clone();
                let right = ast.query_expr(binary.right).span(ast);
                TextSpan::combine(vec![left, operator, right])
            }
            ExprKind::Variable(var) => var.identifier.span.clone(),
            ExprKind::Call(call) => combine_call_expr_span(call),
            ExprKind::If(if_expr) => if_expr.if_keyword.span.clone(),
            _ => {
                log::debug!("No span for {:#?}", self);
                unreachable!()
            }
        }
    }
}

pub fn combine_call_expr_span(call: &CallExpr) -> TextSpan {
    TextSpan::combine(vec![
        call.callee.span.clone(),
        call.left_paren.span.clone(),
        call.right_paren.span.clone(),
    ])
}
