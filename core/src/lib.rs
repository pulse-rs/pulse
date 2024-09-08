use crate::error::error::Error;

pub mod ast;
pub mod build;
pub mod error;
pub mod lexer;

pub type Result<T> = std::result::Result<T, Error>;
