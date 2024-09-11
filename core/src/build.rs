use crate::ast::Ast;
use crate::error::error::Error::ParseError;
use crate::global_context::GlobalContext;
use crate::lexer::token::{Token, TokenKind};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::Result;

pub struct BuildProcess {
    pub input: String,
    pub ast: &'static mut Ast,
    pub ctx: &'static mut GlobalContext,
}

impl BuildProcess {
    pub fn with_input(input: String) -> Self {
        Self {
            input,
            ast: Box::leak(Box::new(Ast::new())),
            ctx: Box::leak(Box::new(GlobalContext::new())),
        }
    }

    pub fn compile(self) -> Result<()> {
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
            println!("{:?}", token);
            tokens.push(token);
        }
        log::debug!("Finished lexical analysis with {} tokens", tokens.len());

        let mut parser = Parser::new(tokens, self.input, self.ast, self.ctx);

        match parser.parse() {
            Ok(_) => {
                log::debug!("Finished parsing");
                println!("{:?}", self.ast);

                Ok(())
            }
            Err(e) => {
                log::debug!("Error parsing: {}", e);

                Err(e)
            }
        }
    }
}
