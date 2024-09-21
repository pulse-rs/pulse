use crate::ast::visitor::ASTWalker;
use crate::ast::Ast;
use crate::Result;
use std::fmt::Write;
use std::path::PathBuf;

pub mod walker;

use crate::global_context::GlobalContext;
use walker::*;

pub struct CppCodegen<'a> {
    pub ast: &'a mut Ast,
    pub file: PathBuf,
    pub output: String,
    pub ctx: &'a mut GlobalContext,
}

impl<'a> CppCodegen<'a> {
    pub fn new(ast: &'a mut Ast, file: PathBuf, ctx: &'a mut GlobalContext) -> Self {
        Self {
            ast,
            file,
            output: String::new(),
            ctx,
        }
    }

    pub fn generate_code(&mut self) -> Result<String> {
        let ast_ptr: *mut Ast = self.ast as *mut Ast;
        for (id, _) in self.ast.items.clone().iter() {
            unsafe {
                self.visit_item(&mut *ast_ptr, *id)?;
            }
        }

        Ok(self.output.clone())
    }
}
