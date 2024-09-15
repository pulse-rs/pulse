#![feature(let_chains)]

use crate::error::error::Error;

pub mod ast;
pub mod build;
pub mod error;
mod global_context;
mod ir;
pub mod lexer;
mod parser;
mod scopes;
mod semantic;
mod types;

pub type Result<T> = std::result::Result<T, Error>;
