use crate::ast::span::TextSpan;
use crate::error::diagnostics::Diagnostic;
use log::Level;
use std::io::{BufWriter, Stderr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Generic(String, Option<String>),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0} is not implemented")]
    NotImplemented(String),
    #[error("Invalid extension provided: {0}")]
    InvalidExtension(String),
    #[error("Provided file does not exist")]
    FileDoesNotExist,
    #[error("Failed to parse tokens: {0}")]
    ParseError(String, TextSpan, String),
    #[error("Found invalid type: {0}")]
    InvalidType(String, TextSpan, String),
    #[error("Function {0} already exists")]
    FunctionAlreadyExists(String, TextSpan, String),
    #[error("Main function cannot have parameters")]
    MainFunctionParameters,
    #[error("Type mismatch. Attempted to assign {0} to {1}")]
    TypeMismatch(String, String, TextSpan, String),
    #[error("Cannot find {0} in the current scope")]
    NotFound(String, TextSpan, String),
    #[error("Illegal return statement")]
    IllegalReturn(TextSpan, String),
    #[error("Tried to call undeclared function: {0}")]
    CallToUndeclaredFunction(String, TextSpan, String),
    #[error("Invalid arguments provided to call expresion. Expected {0}, got {1}")]
    InvalidArguments(usize, usize, TextSpan, String),
    #[error("Tried to create function with std reserved name: {0}")]
    ReservedName(String, TextSpan, String),
    #[error("Format error: {0}")]
    FormatError(#[from] std::fmt::Error),
    #[error("No C++ compiler found. Looked for: {0}")]
    CompilerNotFound(String),
    #[error("Couldn't find program: {0}")]
    WhichError(#[from] which::Error),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::Generic(s, None)
    }
}

impl Error {
    pub fn io(err: std::io::Error) -> Self {
        Self::Io(err)
    }

    pub fn generic(title: &str, msg: Option<&str>) -> Self {
        Self::Generic(title.to_string(), msg.map(|s| s.to_string()))
    }
}

impl Error {
    pub fn log_pretty(self, buffer: &mut BufWriter<Stderr>) {
        let diagnostic = self.into_diagnostic();

        diagnostic.log_pretty(buffer);
    }

    pub fn into_diagnostic(self) -> Diagnostic {
        let string = self.to_string();

        let (title, text, level, location, hint, content) = match self {
            Self::Generic(title, msg) => (title, msg, Level::Error, None, None, None),
            Self::Io(msg)   => (msg.to_string(), None, Level::Error, None, None, None),
            Self::FormatError(msg) => (msg.to_string(), None, Level::Error, None, None, None),
            Self::NotImplemented(_) => (
                self.to_string(),
                None,
                Level::Error,
                None,
                Some("Check if there is an issue open for it on the GitHub.".to_string()),
                None,
            ),
            Self::InvalidExtension(_) => (
                self.to_string(),
                Some("Expected file extension to be `.pulse`".to_string()),
                Level::Error,
                None,
                None,
                None,
            ),
            Self::FileDoesNotExist => (self.to_string(), None, Level::Error, None, None, None),
            Self::CompilerNotFound(msg) => (
                string,
                None,
                Level::Error,
                None,
                Some("Make sure you have a C++ compiler installed.".to_string()),
                Some(msg),
            ),
            Self::WhichError(msg) => (
                string,
                None,
                Level::Error,
                None,
                Some("Make sure you have the program installed. If it is installed, make sure it is in your PATH.".to_string()),
                Some(msg.to_string()),
            ),
            Self::ParseError(msg, span, content) => {
                (msg, None, Level::Error, Some(span), None, Some(content))
            }
            Self::FunctionAlreadyExists(_, span, content)
            | Self::InvalidType(_, span, content)
            | Self::NotFound(_, span, content)
            | Self::CallToUndeclaredFunction(_, span, content)
            | Self::IllegalReturn(span, content)
            | Self::TypeMismatch(_, _, span, content)
            | Self::InvalidArguments(_, _, span, content)
            | Self::ReservedName(_, span, content) => {
                (string, None, Level::Error, Some(span), None, Some(content))
            }
            Self::MainFunctionParameters => {
                (self.to_string(), None, Level::Error, None, None, None)
            }
        };

        Diagnostic {
            title,
            text,
            level,
            location,
            hint,
            content,
        }
    }
}
