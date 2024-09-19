#![feature(let_chains)]
#![allow(warnings, unused)]

use crate::error::error::Error;

pub mod ast;
pub mod build;
mod codegen;
pub mod error;
mod global_context;
pub mod lexer;
mod parser;
mod scopes;
mod semantic;
mod types;

pub type Result<T> = std::result::Result<T, Error>;
