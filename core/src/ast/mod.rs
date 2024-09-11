use crate::ast::expr::{
    AssignExpr, BinOperator, BinaryExpr, BlockExpr, BoolExpr, CallExpr, ElseBranch, Expr, ExprKind,
    IfExpr, NumberExpr, ParenthesizedExpr, UnOperator, UnaryExpr, VarExpr,
};
use crate::ast::function::{Body, FunctionParameter, FunctionType, TypeAnnotation};
use crate::ast::item::{Item, ItemKind};
use crate::ast::stmt::{LetStmt, ReturnStmt, Stmt, StmtKind, WhileStmt};
use crate::lexer::token::Token;
use crate::types::Type;
use crate::Result;
use indexmap::IndexMap;
use std::fmt::Debug;

pub mod expr;
pub mod function;
pub mod item;
pub mod position;
pub mod span;
pub mod stmt;

pub type ID = u32;

pub fn new_id(mut last_id: ID) -> ID {
    last_id += 1;
    last_id
}

#[derive(Clone)]
pub struct Ast {
    pub items: IndexMap<ID, Item>,
    pub stmts: IndexMap<ID, Stmt>,
    pub exprs: IndexMap<ID, Expr>,
}

impl Debug for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ast")
            .field("items", &self.items)
            .field("stmts", &self.stmts)
            .field("exprs", &self.exprs)
            .finish()
    }
}

impl Ast {
    pub fn new() -> Self {
        Self {
            items: IndexMap::new(),
            stmts: IndexMap::new(),
            exprs: IndexMap::new(),
        }
    }

    pub fn new_item(&mut self, kind: ItemKind) -> &Item {
        let id = new_id(self.items.len() as u32);
        let item = Item::new(kind, id);
        self.items.insert(id, item);

        self.items.get(&id).unwrap()
    }

    pub fn new_stmt(&mut self, kind: StmtKind) -> ID {
        let id = new_id(self.items.len() as u32);
        let stmt = Stmt::new(kind, id);
        self.stmts.insert(stmt.id, stmt);

        id
    }

    pub fn let_stmt(
        &mut self,
        identifier: Token,
        initializer: u32,
        type_annotation: Option<TypeAnnotation>,
    ) -> ID {
        let var_id = new_id(self.stmts.len() as u32);

        let let_stmt = StmtKind::Let(LetStmt {
            identifier,
            initializer,
            type_annotation,
            variable_id: var_id,
        });

        self.new_stmt(let_stmt)
    }

    pub fn new_expr(&mut self, kind: ExprKind) -> &Expr {
        let expr = Expr::new(kind, new_id(self.exprs.len() as u32), Type::Unresolved);
        let id = expr.id;
        self.exprs.insert(id, expr);

        self.exprs.get(&id).unwrap()
    }

    pub fn while_statement(&mut self, while_keyword: Token, condition: ID, body: Body) -> ID {
        self.new_stmt(StmtKind::While(WhileStmt {
            while_keyword,
            condition,
            body,
        }))
    }

    pub fn new_func_item(
        &mut self,
        func_keyword: Token,
        identifier: Token,
        parameters: Vec<FunctionParameter>,
        body: Body,
        return_type: Option<FunctionType>,
        function_id: ID,
    ) -> Result<&Item> {
        let func_decl = ItemKind::Function(function::FunctionDeclaration {
            func_keyword,
            identifier,
            parameters,
            body,
            return_type,
            id: function_id,
        });

        Ok(self.new_item(func_decl))
    }

    pub fn expression_statement(&mut self, expr_id: ID) -> ID {
        self.new_stmt(StmtKind::Expr(expr_id))
    }

    pub fn return_statement(&mut self, return_keyword: Token, return_value: Option<ID>) -> ID {
        self.new_stmt(StmtKind::Return(ReturnStmt {
            return_keyword,
            return_value,
        }))
    }

    pub fn number_expression(&mut self, token: Token, number: i64) -> &Expr {
        self.new_expr(ExprKind::Number(NumberExpr { number, token }))
    }

    pub fn boolean_expression(&mut self, token: Token, value: bool) -> &Expr {
        self.new_expr(ExprKind::Boolean(BoolExpr { token, value }))
    }

    pub fn parenthesized_expression(
        &mut self,
        left_paren: Token,
        expression: ID,
        right_paren: Token,
    ) -> &Expr {
        self.new_expr(ExprKind::Parenthesized(ParenthesizedExpr {
            inner: expression,
            left_paren,
            right_paren,
        }))
    }

    pub fn if_expr(
        &mut self,
        if_keyword: Token,
        condition: ID,
        then: Body,
        else_statement: Option<ElseBranch>,
    ) -> &Expr {
        self.new_expr(ExprKind::If(IfExpr {
            if_keyword,
            condition,
            then_branch: then,
            else_branch: else_statement,
        }))
    }

    pub fn variable_expression(&mut self, identifier: Token) -> &Expr {
        let id = new_id(self.exprs.len() as u32);

        self.new_expr(ExprKind::Variable(VarExpr {
            identifier,
            variable_idx: id,
        }))
    }

    pub fn block_expression(
        &mut self,
        left_brace: Token,
        statements: Vec<ID>,
        right_brace: Token,
    ) -> &Expr {
        self.new_expr(ExprKind::Block(BlockExpr {
            left_brace,
            stmts: statements,
            right_brace,
        }))
    }

    pub fn assignment_expression(
        &mut self,
        identifier: Token,
        equals: Token,
        expression: ID,
    ) -> &Expr {
        let new_id = new_id(self.exprs.len() as u32);

        self.new_expr(ExprKind::Assignment(AssignExpr {
            identifier,
            equals,
            expression,
            variable_idx: new_id,
        }))
    }

    pub fn unary_expr(&mut self, operator: UnOperator, operand: ID) -> &Expr {
        self.new_expr(ExprKind::Unary(UnaryExpr { operator, operand }))
    }

    pub fn call_expression(
        &mut self,
        callee: Token,
        left_paren: Token,
        arguments: Vec<ID>,
        right_paren: Token,
    ) -> &Expr {
        self.new_expr(ExprKind::Call(CallExpr {
            callee,
            arguments,
            left_paren,
            right_paren,
            function_idx: u32::MAX,
        }))
    }

    pub fn binary_expression(&mut self, operator: BinOperator, left: ID, right: ID) -> &Expr {
        self.new_expr(ExprKind::Binary(BinaryExpr {
            operator,
            left,
            right,
        }))
    }

    pub fn query_stmt(&self, id: ID) -> &Stmt {
        self.stmts.get(&id).unwrap()
    }
}
