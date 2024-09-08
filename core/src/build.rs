use crate::lexer::token::Token;
use crate::lexer::Lexer;

pub struct BuildProcess {
    pub input: String,
}

impl BuildProcess {
    pub fn with_input(input: String) -> Self {
        Self { input }
    }

    pub fn compile(self) {
        log::debug!("Starting compilation process");
        let mut tokens: Vec<Token> = vec![];
        let mut lexer = Lexer::new(&self.input);

        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        log::debug!("Finished lexical analysis with {} tokens", tokens.len());

        for token in tokens {
            log::debug!("{:?}", token);
        }
    }
}
