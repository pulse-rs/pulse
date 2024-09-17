use crate::ast::visitor::ASTWalker;
use crate::ast::Ast;
use crate::error::error::Error::{MainFunctionParameters, ParseError};
use crate::global_context::GlobalContext;
use crate::ir::interner::Interner;
use crate::ir::passes::run_passes_on;
use crate::ir::IRCompiler;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use log::debug;
use std::path::PathBuf;
// use crate::ir::IRCompiler;
use crate::lexer::token::{Token, TokenKind};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::scopes::Scopes;
use crate::semantic::types::TypeAnalyzer;
use crate::Result;

pub struct BuildProcess {
    pub input: String,
    pub ast: &'static mut Ast,
    pub ctx: &'static mut GlobalContext,
    pub file: PathBuf,
}

impl BuildProcess {
    pub fn new(input: String, file: PathBuf) -> Self {
        Self {
            input,
            ast: Box::leak(Box::new(Ast::new())),
            ctx: Box::leak(Box::new(GlobalContext::new())),
            file,
        }
    }

    pub fn compile(&mut self) -> Result<()> {
        log::debug!("Starting compilation process");
        let mut tokens: Vec<Token> = vec![];
        let mut lexer = Lexer::new(&self.input);

        while let Some(token) = lexer.next_token() {
            if token.kind == TokenKind::Whitespace {
                continue;
            }
            if token.kind == TokenKind::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        log::debug!("Finished lexical analysis with {} tokens", tokens.len());

        let mut parser = Parser::new(tokens, self.input.clone(), self.ast, self.ctx);

        match parser.parse() {
            Ok(_) => {
                log::debug!("Finished parsing");
                let scopes = Scopes::new(self.ctx.clone());
                let mut type_analyzer = TypeAnalyzer {
                    content: self.input.clone(),
                    scopes: scopes.clone(),
                };

                for (id, _) in self.ast.items.clone().iter() {
                    type_analyzer.visit_item(self.ast, *id)?;
                }
                let mut interner = Interner::new();
                log::debug!("Initialized string interner");
                let file_id = interner.intern(&self.file.to_string_lossy());
                log::debug!("Interned main file id: {:?}", file_id);

                let context = Context::create();
                let builder = context.create_builder();
                let module = context.create_module("main");
                let mut compiler = IRCompiler {
                    context: &context,
                    builder: &builder,
                    module: &module,
                    scopes: &scopes,
                    variables: Default::default(),
                    fn_val: None,
                };

                compiler.compile(self.ast)?;

                run_passes_on(&module);

                log::debug!(
                    "Compiled to IR: \n====== \n {}\n======",
                    module.print_to_string().to_string()
                );

                let jit = module
                    .create_jit_execution_engine(OptimizationLevel::None)
                    .unwrap();

                let main = unsafe {
                    jit.get_function::<unsafe extern "C" fn() -> f64>("main")
                        .unwrap()
                };

                unsafe {
                    let result = main.call();
                    println!("Result: {}", result);
                }

                Ok(())
            }
            Err(e) => {
                log::debug!("Error parsing: {}", e);

                Err(e)
            }
        }
    }
}
