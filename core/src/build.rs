use crate::lexer::Lexer;

pub struct BuildProcess;

impl BuildProcess {
    pub fn compile(input: &str) {
        log::debug!("Starting compilation process");
        let mut lexer = Lexer::new(input.to_string());
    }
}
