// use crate::ast::Ast;
// use crate::global_context::GlobalContext;
// use crate::Result;
// use inkwell::builder::Builder;
// use inkwell::context::Context;
// use inkwell::module::Module;
// use inkwell::values::IntValue;
//
// pub struct IRCompiler {
//     pub ast: &'static mut Ast,
//     pub ctx: &'static mut GlobalContext,
// }
//
// enum Expr {
//     Number(i32),
//     Add(Box<Expr>, Box<Expr>),
// }
//
// fn compile_expr<'ctx>(expr: &Expr, context: &'ctx Context, builder: &inkwell::builder::Builder<'ctx>, module: &Module) -> IntValue<'ctx> {
//     let i32_type = context.i32_type();
//
//     match expr {
//         Expr::Number(n) => i32_type.const_int(*n as u64, false),
//         Expr::Add(lhs, rhs) => {
//             let lhs_val = compile_expr(lhs, context, builder, module);
//             let rhs_val = compile_expr(rhs, context, builder, module);
//             builder.build_int_add(lhs_val, rhs_val, "sum").unwrap()
//         }
//     }
// }
//
// impl IRCompiler {
//     pub fn new(ast: &'static mut Ast, ctx: &'static mut GlobalContext) -> Self {
//         Self { ast, ctx }
//     }
//
//     pub fn compile(&mut self) -> Result<()> {
//         log::debug!("Starting IR compilation process");
//
//         let context = Context::create();
//         let module = context.create_module("main");
//         let builder = context.create_builder();
//         let ast = Expr::Add(Box::new(Expr::Number(1)), Box::new(Expr::Number(2)));
//
//         let i32_type = context.i32_type();
//         let fn_type = i32_type.fn_type(&[], false);
//         let function = module.add_function("main", fn_type, None);
//         let entry = context.append_basic_block(function, "entry");
//         builder.position_at_end(entry);
//
//         let result = compile_expr(&ast, &context, &builder, &module);
//         builder.build_return(Some(&result))?;
//
//         module.print_to_stderr();
//
//         Ok(())
//     }
// }
