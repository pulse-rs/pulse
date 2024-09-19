use crate::ast::Ast;
use std::path::PathBuf;

pub struct CCodegen<'a> {
    pub ast: &'a mut Ast,
    pub file: PathBuf,
}

impl<'a> CCodegen<'a> {
    pub fn new(ast: &'a mut Ast, file: PathBuf) -> Self {
        Self { ast, file }
    }

    pub fn generate_code(&mut self) {
        println!("for: {}", self.file.display());
        println!("Generating code");
    }
}
