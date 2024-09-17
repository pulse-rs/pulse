pub mod interner;
pub mod passes;

use crate::ast::expr::{
    BinOpKind, BinOperator, BoolExpr, Expr, ExprKind, NumberExpr, StringExpr, UnaryExpr, VarExpr,
};
use crate::ast::function::{Body, FunctionDeclaration};
use crate::ast::span::TextSpan;
use crate::ast::stmt::{LetStmt, Stmt, StmtKind};
use crate::ast::visitor::ASTWalker;
use crate::ast::{Ast, ID};
use crate::error::error::Error;
use crate::error::error::Error::{ExecutionEngineError, StrError};
use crate::global_context::GlobalContext;
use crate::ir::interner::StrId;
use crate::scopes::Scopes;
use crate::Result;
use indexmap::IndexMap;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::targets::{InitializationConfig, Target};
use inkwell::values::{
    BasicMetadataValueEnum, BasicValueEnum, FloatValue, FunctionValue, IntValue, PointerValue,
};
use inkwell::FloatPredicate;
use rustc_hash::FxHashMap;

pub struct IRCompiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    pub scopes: &'a Scopes,
    pub variables: IndexMap<String, PointerValue<'ctx>>,
    pub fn_val: Option<FunctionValue<'ctx>>,
}
// TODO: improve error handling

impl<'a, 'ctx> IRCompiler<'a, 'ctx> {
    #[inline]
    pub fn fn_value(&self) -> FunctionValue<'ctx> {
        self.fn_val.unwrap()
    }

    pub fn compile(&mut self, ast: &'a mut Ast) -> Result<()> {
        for (id, _) in ast.items.clone().iter() {
            self.visit_item(ast, *id)?
        }

        Ok(())
    }

    pub fn create_entry_alloca(&self, name: &str) -> PointerValue<'ctx> {
        let builder = self.context.create_builder();
        let entry = self.fn_value().get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry),
        }

        builder.build_alloca(self.context.i64_type(), name).unwrap()
    }

    pub fn parse_body(&mut self, ast: &mut Ast, body: Body) -> Result<Vec<FloatValue>> {
        let mut list = vec![];
        for stmt in body.stmts {
            let expr = ast.query_stmt(stmt).clone();

            match expr.kind {
                StmtKind::Expr(expr) => {
                    let expr = ast.query_expr(expr);

                    let pt = self
                        .compile_expr(ast, expr.clone())
                        .map_err(|err| StrError(err))?;

                    list.push(pt);
                }
                StmtKind::Let(let_stmt) => {
                    let name = let_stmt.identifier.span.literal;

                    let expr = ast.query_expr(let_stmt.initializer);
                    let val = self
                        .compile_expr(ast, expr.clone())
                        .map_err(|err| StrError(err))?;
                    let alloca = self.create_entry_alloca(&name);
                    self.builder.build_store(alloca, val);
                    self.variables.insert(name.clone(), alloca);
                }
                StmtKind::Return(_) => {}
                StmtKind::While(_) => {}
            };
        }

        Ok(list)
    }

    pub fn compile_expr(
        &mut self,
        ast: &mut Ast,
        expr: Expr,
    ) -> std::result::Result<FloatValue<'ctx>, &'static str> {
        match expr.kind {
            ExprKind::Number(number) => {
                Ok(self.context.f64_type().const_float(number.number as f64))
            }
            ExprKind::Variable(var) => {
                let name = var.identifier.span.literal;
                let ptr = self.variables.get(&name).unwrap();
                Ok(self.build_load(*ptr, &name).into_float_value())
            }
            ExprKind::Binary(bin) => {
                let lhs = self.compile_expr(ast, ast.query_expr(bin.left).clone())?;
                let rhs = self.compile_expr(ast, ast.query_expr(bin.right).clone())?;
                let op = bin.operator;

                match op.kind {
                    BinOpKind::Plus => Ok(self
                        .builder
                        .build_float_add(lhs, rhs, "add")
                        .map_err(|_| "Error adding floats")?),
                    BinOpKind::Minus => Ok(self
                        .builder
                        .build_float_sub(lhs, rhs, "sub")
                        .map_err(|_| "Error subtracting floats")?),
                    BinOpKind::Multiply => Ok(self
                        .builder
                        .build_float_mul(lhs, rhs, "mul")
                        .map_err(|_| "Error multiplying floats")?),
                    BinOpKind::Divide => Ok(self
                        .builder
                        .build_float_div(lhs, rhs, "div")
                        .map_err(|_| "Error dividing floats")?),
                    _ => todo!("Unsupported binary operator"),
                }
            }
            ExprKind::Call(call) => {
                if let Some(func) = self.module.get_function(&call.callee.span.literal) {
                    let mut compiled_args = Vec::with_capacity(call.arguments.len());

                    for arg in call.arguments {
                        compiled_args.push(self.compile_expr(ast, ast.query_expr(arg).clone())?);
                    }

                    let argsv: Vec<BasicMetadataValueEnum> = compiled_args
                        .iter()
                        .by_ref()
                        .map(|&val| val.into())
                        .collect();

                    match self
                        .builder
                        .build_call(func, argsv.as_slice(), "tmp")
                        .unwrap()
                        .try_as_basic_value()
                        .left()
                    {
                        Some(value) => Ok(value.into_float_value()),
                        None => Err("Invalid call produced."),
                    }
                } else {
                    Err("Function not found")
                }
            }
            // ExprKind::If(if_expr) => {
            //     let parent = self.fn_value();
            //     let zero_const = self.context.f64_type().const_float(0.0);
            //
            //     let expr = ast.query_expr(if_expr.condition).clone();
            //     let cond = self.compile_expr(ast, expr)?;
            //     let cond = self
            //         .builder
            //         .build_float_compare(FloatPredicate::ONE, cond, zero_const, "ifcond")
            //         .map_err(|_| "Error comparing floats")?;
            //
            //     let then_bb = self.context.append_basic_block(parent, "then");
            //     let else_bb = self.context.append_basic_block(parent, "else");
            //     let merge_bb = self.context.append_basic_block(parent, "ifcont");
            //
            //     self.builder
            //         .build_conditional_branch(cond, then_bb, else_bb)
            //         .map_err(|_| "Error building conditional branch")?;
            //
            //     self.builder.position_at_end(then_bb);
            //     let then_val = self
            //         .parse_body(ast, if_expr.then_branch.clone())
            //         .map_err(|_| "Error parsing then branch")?;
            //     let then_val = then_val.last().unwrap();
            //     self.builder
            //         .build_unconditional_branch(merge_bb)
            //         .map_err(|_| "Error building unconditional branch")?;
            //
            //     let then_bb = self.builder.get_insert_block().unwrap();
            //
            //     self.builder.position_at_end(else_bb);
            //     let else_val = self
            //         .parse_body(ast, if_expr.else_branch.unwrap().body)
            //         .map_err(|_| "Error parsing else branch")?;
            //     let else_val = else_val.last().unwrap();
            //     self.builder
            //         .build_unconditional_branch(merge_bb)
            //         .map_err(|_| "Error building unconditional branch")?;
            //     let else_bb = self.builder.get_insert_block().unwrap();
            //
            //     self.builder.position_at_end(merge_bb);
            //
            //     let phi = self
            //         .builder
            //         .build_phi(self.context.f64_type(), "iftmp")
            //         .map_err(|_| "Error building phi")?;
            //
            //     phi.add_incoming(&[(then_val, then_bb), (else_val, else_bb)]);
            //
            //     Ok(phi.as_basic_value().into_float_value())
            // }
            _ => {
                println!("{:?}", expr);
                todo!()
            }
        }
    }

    pub fn build_load(&self, ptr: PointerValue<'ctx>, name: &str) -> BasicValueEnum<'ctx> {
        self.builder
            .build_load(self.context.f64_type(), ptr, name)
            .unwrap()
    }
}

impl ASTWalker for IRCompiler<'_, '_> {
    fn visit_func_decl(
        &mut self,
        ast: &mut Ast,
        func_decl: &FunctionDeclaration,
        item_id: ID,
    ) -> Result<()> {
        let return_type = self.context.f64_type();
        let args = std::iter::repeat(return_type)
            .take(func_decl.parameters.len())
            .map(|t| t.into())
            .collect::<Vec<_>>();

        let fn_type = self.context.f64_type().fn_type(&args, false);
        let fn_value = self
            .module
            .add_function(&func_decl.identifier.span.literal, fn_type, None);

        for (i, arg) in fn_value.get_param_iter().enumerate() {
            let var = self.scopes.global.lookup_var(func_decl.parameters[i]);

            if let Some(var) = var {
                arg.set_name(&var.name);
            }
        }

        let entry = self.context.append_basic_block(fn_value, "entry");

        self.builder.position_at_end(entry);

        self.fn_val = Some(fn_value);
        self.variables.reserve(func_decl.parameters.len());

        for (i, arg) in fn_value.get_param_iter().enumerate() {
            let var = self.scopes.global.lookup_var(func_decl.parameters[i]);

            if let Some(var) = var {
                let alloca = self.create_entry_alloca(&var.name);
                self.builder.build_store(alloca, arg);
                self.variables.insert(var.name.clone(), alloca);
            }
        }

        let list = self.parse_body(ast, func_decl.body.clone())?;
        let return_value = list
            .last()
            .expect("Expected at least one expression in the function body");
        self.builder.build_return(Some(return_value));

        fn_value.verify(true);

        Ok(())
    }

    fn visit_let_statement(
        &mut self,
        ast: &mut Ast,
        let_statement: &LetStmt,
        stmt: &Stmt,
    ) -> Result<()> {
        Ok(())
    }

    fn visit_variable_expression(
        &mut self,
        ast: &mut Ast,
        variable_expression: &VarExpr,
        expr: &Expr,
    ) -> Result<()> {
        Ok(())
    }

    fn visit_number_expression(
        &mut self,
        ast: &mut Ast,
        number: &NumberExpr,
        expr: &Expr,
    ) -> Result<()> {
        Ok(())
    }

    fn visit_string_expression(
        &mut self,
        ast: &mut Ast,
        string: &StringExpr,
        expr: &Expr,
    ) -> Result<()> {
        Ok(())
    }

    fn visit_boolean_expression(
        &mut self,
        ast: &mut Ast,
        boolean: &BoolExpr,
        expr: &Expr,
    ) -> Result<()> {
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
        Ok(())
    }
}
