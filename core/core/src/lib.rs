mod colors;
mod diagnostics;
pub mod error;

pub type Result<T> = std::result::Result<T, error::Error>;
