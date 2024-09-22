use crate::ast::expr::{
    BinOpKind, BinOperator, BinaryExpr, BoolExpr, CallExpr, Expr, ExprKind, IfExpr, NumberExpr,
    StringExpr, UnaryExpr, VarExpr,
};
use crate::ast::function::FunctionDeclaration;
use crate::ast::span::TextSpan;
use crate::ast::stmt::{LetStmt, ReturnStmt, Stmt};
use crate::ast::visitor::ASTWalker;
use crate::ast::{Ast, ID};
use crate::codegen::CppCodegen;
use crate::semantic::types::STD_RESERVED_WORDS;
use crate::types::Type;
use crate::Result;
use std::fmt::Write;

fn type_to_str(type_: Type) -> String {
    match type_ {
        Type::Int => "int".to_string(),
        Type::String => "std::string".to_string(),
        Type::Bool => "bool".to_string(),
        Type::Void => "void".to_string(),
        _ => panic!("Invalid type"),
    }
}

fn to_operator(op: BinOperator) -> &'static str {
    match op.kind {
        BinOpKind::Plus => "+",
        BinOpKind::Minus => "-",
        BinOpKind::Multiply => "*",
        BinOpKind::Divide => "/",
        BinOpKind::Equals => "==",
        BinOpKind::NotEquals => "!=",
        BinOpKind::LessThan => "<",
        BinOpKind::GreaterThan => ">",
        BinOpKind::BitwiseAnd => "&",
        BinOpKind::BitwiseOr => "|",
        BinOpKind::BitwiseXor => "^",
        BinOpKind::LessThanOrEqual => "<=",
        BinOpKind::GreaterThanOrEqual => ">=",
        BinOpKind::Modulo => "%",
        BinOpKind::Power => "**",
    }
}

impl ASTWalker for CppCodegen<'_> {
    fn visit_func_decl(
        &mut self,
        ast: &mut Ast,
        func_decl: &FunctionDeclaration,
        item_id: ID,
    ) -> Result<()> {
        let func = self.ctx.functions.get(&item_id).unwrap();

        let type_name = func.return_type.to_string();
        let name = func.name.to_string();

        write!(self.output, "{} {}(", type_name, name)?;

        for (i, param) in func.parameters.iter().enumerate() {
            if i != 0 {
                write!(self.output, ", ")?;
            }

            let param = self.ctx.variables.get(param).unwrap();
            let param_type = type_to_str(param.type_.clone());

            write!(self.output, "{} {}", param_type, param.name)?;
        }

        write!(self.output, ") {{\n")?;
        let body = func.body.stmts.clone();

        for stmt in body {
            self.visit_statement(ast, stmt)?;
        }

        write!(self.output, "\n}}\n")?;

        Ok(())
    }

    // TODO: Allow last exprs in the body to be returned
    fn visit_return_statement(
        &mut self,
        ast: &mut Ast,
        return_statement: &ReturnStmt,
    ) -> Result<()> {
        if let Some(return_value) = &return_statement.return_value {
            write!(self.output, "return ")?;
            self.visit_expression(ast, *return_value)?;
        } else {
            write!(self.output, "return")?;
        }

        Ok(())
    }

    fn visit_if_expression(&mut self, ast: &mut Ast, if_expr: &IfExpr, expr: &Expr) -> Result<()> {
        write!(self.output, "if (")?;
        self.visit_expression(ast, if_expr.condition)?;

        write!(self.output, ") {{\n")?;
        match if_expr.else_branch.as_ref() {
            None => {
                for stmt in if_expr.then_branch.stmts.clone() {
                    self.visit_statement(ast, stmt)?;
                }

                write!(self.output, "}}\n")?;
            }
            Some(else_branch) => {
                for stmt in if_expr.then_branch.stmts.clone() {
                    self.visit_statement(ast, stmt)?;
                }

                write!(self.output, "}} else {{\n")?;

                for stmt in else_branch.body.stmts.clone() {
                    self.visit_statement(ast, stmt)?;
                }

                write!(self.output, "}}\n")?;
            }
            _ => unreachable!("Invalid if expression"),
        }

        Ok(())
    }

    fn visit_let_statement(
        &mut self,
        ast: &mut Ast,
        let_statement: &LetStmt,
        stmt: &Stmt,
    ) -> Result<()> {
        if let Some(var) = self.ctx.lookup_var(let_statement.variable_id) {
            let type_name = type_to_str(var.type_.clone());

            write!(self.output, "{} {} = ", type_name, var.name)?;

            self.visit_expression(ast, let_statement.initializer)?;

            write!(self.output, "")?;
        }

        Ok(())
    }

    fn visit_statement(&mut self, ast: &mut Ast, statement: ID) -> Result<()> {
        self.do_visit_statement(ast, statement)?;
        write!(self.output, ";\n")?;

        Ok(())
    }

    fn visit_call_expression(
        &mut self,
        ast: &mut Ast,
        call_expression: &CallExpr,
        _expr: &Expr,
    ) -> Result<()> {
        if let Some(scope) = call_expression.scope {
            let scope = ast.query_expr(scope);

            let expr = match &scope.kind {
                ExprKind::ScopedIdentifier { path } => {
                    for token in path {
                        if token.span.literal == "std" {
                            continue;
                        }
                        write!(self.output, "{}::", token.span.literal.clone())?;
                    }
                }
                _ => unreachable!("Invalid scope"),
            };

            write!(self.output, "{}(", call_expression.callee.span.literal)?;

            for (i, arg) in call_expression.arguments.iter().enumerate() {
                if i != 0 {
                    write!(self.output, ", ")?;
                }

                self.visit_expression(ast, *arg)?;
            }

            write!(self.output, ")")?;

            return Ok(());
        }

        if STD_RESERVED_WORDS.contains(&call_expression.callee.span.literal.as_str()) {
            write!(self.output, "{} (", call_expression.callee.span.literal)?;

            for (i, arg) in call_expression.arguments.iter().enumerate() {
                if i != 0 {
                    write!(self.output, ", ")?;
                }

                self.visit_expression(ast, *arg)?;
            }

            write!(self.output, ")")?;

            return Ok(());
        }

        let func = self
            .ctx
            .lookup_function(&call_expression.callee.span.literal);

        log::debug!("TypeAnalyzer::visit_call_expression func: {:?}", func);

        if let Some(func) = func {
            let func = self.ctx.functions.get(&func).unwrap();
            write!(self.output, "{}(", func.name)?;

            for (i, arg) in call_expression.arguments.iter().enumerate() {
                if i != 0 {
                    write!(self.output, ", ")?;
                }

                self.visit_expression(ast, *arg)?;
            }

            write!(self.output, ")")?;
        }

        Ok(())
    }

    fn visit_variable_expression(
        &mut self,
        ast: &mut Ast,
        variable_expression: &VarExpr,
        expr: &Expr,
    ) -> Result<()> {
        if let Some(var) = self.ctx.variables.get(&variable_expression.variable_idx) {
            write!(self.output, "{}", var.name)?;
        }

        Ok(())
    }

    fn visit_number_expression(
        &mut self,
        ast: &mut Ast,
        number: &NumberExpr,
        expr: &Expr,
    ) -> Result<()> {
        write!(self.output, "{}", number.number)?;

        Ok(())
    }

    fn visit_string_expression(
        &mut self,
        ast: &mut Ast,
        string: &StringExpr,
        expr: &Expr,
    ) -> Result<()> {
        write!(self.output, "\"{}\"", string.string)?;

        Ok(())
    }

    fn visit_boolean_expression(
        &mut self,
        ast: &mut Ast,
        boolean: &BoolExpr,
        expr: &Expr,
    ) -> Result<()> {
        write!(self.output, "{}", boolean.value)?;

        Ok(())
    }

    fn visit_error(&mut self, ast: &mut Ast, span: &TextSpan) -> Result<()> {
        Ok(())
    }

    fn visit_unary_expression(
        &mut self,
        ast: &mut Ast,
        unary_expression: &UnaryExpr,
        expr: &Expr,
    ) -> Result<()> {
        todo!()
    }

    fn visit_binary_expression(
        &mut self,
        ast: &mut Ast,
        binary_expression: &BinaryExpr,
        _expr: &Expr,
    ) -> Result<()> {
        let str_op = to_operator(binary_expression.operator.clone());

        self.visit_expression(ast, binary_expression.left)?;
        write!(self.output, " {} ", str_op)?;
        self.visit_expression(ast, binary_expression.right)?;

        Ok(())
    }
}
