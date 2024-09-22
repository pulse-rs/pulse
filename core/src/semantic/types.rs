use crate::ast::expr::{
    combine_call_expr_span, AssignExpr, BinOpKind, BinaryExpr, BlockExpr, BoolExpr, CallExpr, Expr,
    ExprKind, IfExpr, NumberExpr, StringExpr, UnOpKind, UnaryExpr, VarExpr,
};
use crate::ast::function::{get_type_of_last_expr, FunctionDeclaration, TypeAnnotation};
use crate::ast::item::ItemKind;
use crate::ast::span::TextSpan;
use crate::ast::stmt::{LetStmt, ReturnStmt, Stmt, WhileStmt};
use crate::ast::visitor::ASTWalker;
use crate::ast::{Ast, ID};
use crate::error::error::Error::{
    CallToUndeclaredFunction, IllegalReturn, InvalidArguments, MainFunctionParameters, NotFound,
    ReservedName, TypeMismatch,
};
use crate::lexer::token::{Operator, TokenKind};
use crate::scopes::Scopes;
use crate::types::{parse_type, Type};
use crate::Result;
use indexmap::IndexMap;
use lazy_static::lazy_static;
use std::process::id;

pub struct TypeAnalyzer<'a> {
    pub content: String,
    pub scopes: Scopes<'a>,
}

lazy_static! {
    pub static ref STD_RESERVED_WORDS: Vec<&'static str> =
        vec!["print", "println", "eprintln", "eprint"];
    pub static ref STD_MODULES: IndexMap<&'static str, IndexMap<&'static str, Type>> = {
        let mut map = IndexMap::new();
        let mut io = IndexMap::new();
        io.insert("print", Type::Void);
        io.insert("println", Type::Void);
        io.insert("eprint", Type::Void);
        io.insert("eprintln", Type::Void);
        map.insert("io", io);

        let mut math = IndexMap::new();
        math.insert("sqrt", Type::Int);

        map.insert("math", math);
        map
    };
}

impl<'a> ASTWalker for TypeAnalyzer<'a> {
    fn visit_func_decl(
        &mut self,
        ast: &mut Ast,
        func_decl: &FunctionDeclaration,
        item_id: ID,
    ) -> Result<()> {
        log::debug!(
            "TypeAnalyzer::visit_func_decl func with id: {}",
            func_decl.id
        );
        self.scopes.push_scope(Some(func_decl.id));
        let func = self.scopes.global.functions.get(&func_decl.id).unwrap();
        if STD_RESERVED_WORDS.contains(&&*func.name) {
            let item = ast.query_item(item_id);
            let span = match item.kind {
                ItemKind::Function(ref func) => func.identifier.span.clone(),
                _ => unreachable!(),
            };

            return Err(ReservedName(func.name.clone(), span, self.content.clone()));
        }

        for param in &func_decl.parameters {
            self.scopes.local.last_mut().unwrap().add_local(*param);
        }

        self.visit_body(ast, &func_decl.body)?;
        self.scopes.pop_scope();

        Ok(())
    }

    fn visit_return_statement(
        &mut self,
        ast: &mut Ast,
        return_statement: &ReturnStmt,
    ) -> Result<()> {
        log::debug!("TypeAnalyzer::visit_return_statement");
        if let Some(func) = self.scopes.current_function().cloned() {
            if let Some(ret_val) = &return_statement.return_value {
                self.visit_expression(ast, *ret_val)?;
                let ret_val = ast.query_expr(*ret_val).clone();

                expect_type(
                    &ret_val.ty,
                    &func.return_type,
                    &return_statement.return_keyword.span,
                    &self.content,
                )?;

                Ok(())
            } else {
                expect_type(
                    &Type::Void,
                    &func.return_type,
                    &return_statement.return_keyword.span,
                    &self.content,
                )?;

                Ok(())
            }
        } else {
            Err(IllegalReturn(
                return_statement.return_keyword.span.clone(),
                self.content.clone(),
            ))
        }
    }

    fn visit_while_statement(&mut self, ast: &mut Ast, while_statement: &WhileStmt) -> Result<()> {
        log::debug!("TypeAnalyzer::visit_while_statement");

        self.visit_expression(ast, while_statement.condition)?;
        let condition = ast.query_expr(while_statement.condition);
        expect_type(
            &condition.ty,
            &Type::Bool,
            &condition.span(ast),
            &self.content,
        )?;
        self.visit_body(ast, &while_statement.body)?;

        Ok(())
    }

    fn visit_block_expr(
        &mut self,
        ast: &mut Ast,
        block_expr: &BlockExpr,
        _expr: &Expr,
    ) -> Result<()> {
        log::debug!("TypeAnalyzer::visit_block_expr");

        self.scopes.push_scope(None);

        for stmt in &block_expr.stmts {
            self.visit_statement(ast, *stmt)?;
        }

        self.scopes.pop_scope();
        let _type = get_type_of_last_expr(block_expr.stmts.clone(), ast).unwrap_or(Type::Void);

        ast.update_type(_expr.id, _type);
        Ok(())
    }

    fn visit_if_expression(&mut self, ast: &mut Ast, if_expr: &IfExpr, expr: &Expr) -> Result<()> {
        log::debug!("TypeAnalyzer::visit_if_expression");

        self.scopes.push_scope(None);
        self.visit_expression(ast, if_expr.condition)?;

        let condition = ast.query_expr(if_expr.condition);
        expect_type(
            &condition.ty,
            &Type::Bool,
            &condition.span(ast),
            &self.content,
        )?;

        self.visit_body(ast, &if_expr.then_branch)?;
        let mut type_ = Type::Void;
        if let Some(else_branch) = &if_expr.else_branch {
            self.scopes.push_scope(None);
            self.visit_body(ast, &else_branch.body)?;
            self.scopes.pop_scope();

            let then_type = if_expr.then_branch.ty(ast).unwrap_or(Type::Void);
            let else_type = else_branch.body.ty(ast).unwrap_or(Type::Void);

            log::debug!(
                "TypeAnalyzer::visit_if_expression then_type: {:?}, else_type: {:?}",
                then_type,
                else_type
            );
            type_ = expect_type(
                &then_type,
                &else_type,
                &if_expr.if_keyword.span,
                &self.content,
            )?;
            self.scopes.pop_scope();
        }

        ast.update_type(expr.id, type_);

        Ok(())
    }

    fn visit_let_statement(
        &mut self,
        ast: &mut Ast,
        let_statement: &LetStmt,
        stmt: &Stmt,
    ) -> Result<()> {
        log::debug!("TypeAnalyzer::visit_let_statement");

        self.visit_expression(ast, let_statement.initializer)?;
        let init = ast.query_expr(let_statement.initializer).clone();

        let typ = if let Some(ann) = &let_statement.type_annotation {
            let typ = parse_type(&ann.type_name, &self.content)?;
            let init_type = init.clone().ty;

            expect_type(&init_type, &typ, &ann.type_name.clone().span, &self.content)?;

            typ
        } else {
            init.ty.clone()
        };

        let ident = let_statement.identifier.span.literal.clone();

        let var = self.scopes.new_var(ident.clone(), typ);
        ast.set_var_stmt(&stmt.id, var);

        Ok(())
    }

    fn visit_call_expression(
        &mut self,
        ast: &mut Ast,
        call_expression: &CallExpr,
        expr: &Expr,
    ) -> Result<()> {
        let func = self
            .scopes
            .global
            .lookup_function(&call_expression.callee.span.literal);

        log::debug!("TypeAnalyzer::visit_call_expression func: {:?}", func);

        if let Some(func) = func {
            let func = self.scopes.global.functions.get(&func).unwrap();
            let actual_args = call_expression.arguments.len();
            let expected_args = func.parameters.len();

            if actual_args != expected_args {
                return Err(InvalidArguments(
                    expected_args,
                    actual_args,
                    combine_call_expr_span(call_expression),
                    self.content.clone(),
                ));
            }

            let return_type = func.return_type.clone();
            for (argument, param) in call_expression
                .arguments
                .iter()
                .zip(func.parameters.clone().iter())
            {
                self.visit_expression(ast, *argument)?;
                let argument_expression = ast.query_expr(*argument);
                let param = self.scopes.global.variables.get(param);

                expect_type(
                    &argument_expression.ty,
                    &param.unwrap().type_,
                    &argument_expression.span(ast),
                    &self.content,
                )?;
            }

            ast.update_type(expr.id, return_type);
            Ok(())
        } else if let Some(scope) = call_expression.scope {
            let scope = ast.query_expr(scope);

            let expr = match &scope.kind {
                ExprKind::ScopedIdentifier { path } => {
                    if path.first().unwrap().span.literal == "std" {
                        println!("{}", &path[1].span.literal[..]);
                        let module = STD_MODULES
                            .get(&path[1].span.literal[..])
                            .expect("Module not found");
                        let function = module.get(&call_expression.callee.span.literal[..]);

                        if let Some(function) = function {
                            let return_type = function.clone();
                            for argument in &call_expression.arguments {
                                self.visit_expression(ast, *argument)?;
                            }

                            ast.update_type(expr.id, return_type);
                        } else {
                            return Err(CallToUndeclaredFunction(
                                call_expression.callee.span.literal.clone(),
                                call_expression.callee.span.clone(),
                                self.content.clone(),
                            ));
                        }
                    }
                }
                _ => unreachable!("Invalid scope"),
            };

            Ok(())
        } else if STD_RESERVED_WORDS.contains(&&call_expression.callee.span.literal[..]) {
            let return_type = match &call_expression.callee.span.literal[..] {
                "print" => Type::Void,
                "println" => Type::Void,
                "eprint" => Type::Void,
                "eprintln" => Type::Void,
                _ => unreachable!(),
            };

            for argument in &call_expression.arguments {
                self.visit_expression(ast, *argument)?;
            }

            ast.update_type(expr.id, return_type);
            Ok(())
        } else {
            log::debug!("TypeAnalyzer::visit_call_expression Call to undeclared function");

            Err(CallToUndeclaredFunction(
                call_expression.callee.span.literal.clone(),
                call_expression.callee.span.clone(),
                self.content.clone(),
            ))
        }
    }

    fn visit_assignment_expression(
        &mut self,
        ast: &mut Ast,
        assignment_expression: &AssignExpr,
        expr: &Expr,
    ) -> Result<()> {
        log::debug!("TypeAnalyzer::visit_assignment_expression");

        self.visit_expression(ast, assignment_expression.expression)?;
        let ident = assignment_expression.identifier.span.literal.clone();

        let var = self.scopes.lookup_var(&ident);

        if let Some(var) = var {
            ast.set_variable(expr.id, var);
            let var = self.scopes.global.variables.get(&var).unwrap();
            let expr = ast.query_expr(assignment_expression.expression).clone();

            expect_type(&expr.ty, &var.type_, &expr.span(ast), &self.content)?;
            ast.update_type(expr.id, var.type_.clone());
        } else {
            return Err(NotFound(
                ident,
                assignment_expression.identifier.span.clone(),
                self.content.clone(),
            ));
        }

        Ok(())
    }

    fn visit_variable_expression(
        &mut self,
        ast: &mut Ast,
        variable_expression: &VarExpr,
        expr: &Expr,
    ) -> Result<()> {
        log::debug!("TypeAnalyzer::visit_variable_expression");

        let name = variable_expression.identifier.span.literal.clone();

        if !STD_RESERVED_WORDS.contains(&&*name) {
            match self.scopes.lookup_var(&name) {
                Some(id) => {
                    let var = self.scopes.global.variables.get(&id).unwrap();
                    ast.update_type(expr.id, var.type_.clone());
                    ast.set_variable(expr.id, id);
                }
                None => {
                    return Err(NotFound(name, expr.span(ast), self.content.clone()));
                }
            }
        }

        Ok(())
    }

    fn visit_number_expression(
        &mut self,
        ast: &mut Ast,
        _: &NumberExpr,
        expr: &Expr,
    ) -> Result<()> {
        log::debug!("TypeAnalyzer::visit_number_expression");

        ast.update_type(expr.id, Type::Int);

        Ok(())
    }

    fn visit_string_expression(
        &mut self,
        ast: &mut Ast,
        _: &StringExpr,
        expr: &Expr,
    ) -> Result<()> {
        log::debug!("TypeAnalyzer::visit_string_expression");

        ast.update_type(expr.id, Type::String);

        Ok(())
    }

    fn visit_boolean_expression(&mut self, ast: &mut Ast, _: &BoolExpr, expr: &Expr) -> Result<()> {
        log::debug!("TypeAnalyzer::visit_boolean_expression");

        ast.update_type(expr.id, Type::Bool);

        Ok(())
    }

    fn visit_error(&mut self, ast: &mut Ast, span: &TextSpan) -> Result<()> {
        log::debug!("TypeAnalyzer::visit_error");

        Ok(())
    }

    fn visit_unary_expression(
        &mut self,
        ast: &mut Ast,
        unary_expression: &UnaryExpr,
        expr: &Expr,
    ) -> Result<()> {
        log::debug!("TypeAnalyzer::visit_unary_expression");
        self.visit_expression(ast, unary_expression.operand)?;
        let op = ast.query_expr(unary_expression.operand).clone();
        let type_ = match unary_expression.operator.kind {
            UnOpKind::Minus => (Type::Int, Type::Int),
            UnOpKind::BitwiseNot => (Type::Int, Type::Int),
        };

        expect_type(&op.ty, &type_.0, &op.span(ast), &self.content)?;

        ast.update_type(expr.id, type_.1);

        Ok(())
    }

    fn visit_binary_expression(
        &mut self,
        ast: &mut Ast,
        binary_expression: &BinaryExpr,
        expr: &Expr,
    ) -> Result<()> {
        log::debug!("TypeAnalyzer::visit_binary_expression");

        self.visit_expression(ast, binary_expression.left)?;
        self.visit_expression(ast, binary_expression.right)?;

        let left = ast.query_expr(binary_expression.left).clone();
        let right = ast.query_expr(binary_expression.right).clone();

        let operator = &binary_expression.operator.kind;
        let result: (Type, Type, Type) = match operator {
            BinOpKind::Plus => (Type::Int, Type::Int, Type::Int),
            BinOpKind::Minus => (Type::Int, Type::Int, Type::Int),
            BinOpKind::Multiply => (Type::Int, Type::Int, Type::Int),
            BinOpKind::Divide => (Type::Int, Type::Int, Type::Int),
            BinOpKind::Power => (Type::Int, Type::Int, Type::Int),
            BinOpKind::BitwiseAnd => (Type::Int, Type::Int, Type::Int),
            BinOpKind::BitwiseOr => (Type::Int, Type::Int, Type::Int),
            BinOpKind::BitwiseXor => (Type::Int, Type::Int, Type::Int),
            BinOpKind::Equals => (Type::Int, Type::Int, Type::Bool),
            BinOpKind::NotEquals => (Type::Int, Type::Int, Type::Bool),
            BinOpKind::LessThan => (Type::Int, Type::Int, Type::Bool),
            BinOpKind::LessThanOrEqual => (Type::Int, Type::Int, Type::Bool),
            BinOpKind::GreaterThan => (Type::Int, Type::Int, Type::Bool),
            BinOpKind::GreaterThanOrEqual => (Type::Int, Type::Int, Type::Bool),
            BinOpKind::Modulo => (Type::Int, Type::Int, Type::Int),
        };

        expect_type(&left.ty, &result.0, &left.span(ast), &self.content)?;
        expect_type(&right.ty, &result.1, &right.span(ast), &self.content)?;

        ast.update_type(expr.id, result.2);

        Ok(())
    }
}

pub fn expect_type(type1: &Type, type2: &Type, span: &TextSpan, content: &String) -> Result<Type> {
    if !Type::is_assignable_to(type1, type2) {
        return Err(TypeMismatch(
            type1.to_str(),
            type2.to_str(),
            span.clone(),
            content.clone(),
        ));
    }

    Ok(type2.clone())
}
