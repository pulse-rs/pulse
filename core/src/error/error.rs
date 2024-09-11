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

        match self {
            Self::Generic(title, msg) => Diagnostic {
                title,
                text: msg,
                level: Level::Error,
                location: None,
                hint: Some("This is a generic error".to_string()),
                content: None,
            },
            Self::Io(msg) => Diagnostic {
                title: msg.to_string(),
                text: None,
                level: Level::Error,
                location: None,
                hint: None,
                content: None,
            },
            Self::NotImplemented(_) => Diagnostic {
                title: string,
                text: None,
                level: Level::Error,
                location: None,
                hint: Some("Check if there is an issue open for it on the GitHub.".to_string()),
                content: None,
            },
            Self::InvalidExtension(_) => Diagnostic {
                title: string,
                text: Some("Expected file extension to be `.pulse`".to_string()),
                level: Level::Error,
                location: None,
                hint: None,
                content: None,
            },
            Self::FileDoesNotExist => Diagnostic {
                title: string,
                text: None,
                level: Level::Error,
                location: None,
                hint: None,
                content: None,
            },
            Self::ParseError(msg, span, content) => Diagnostic {
                title: msg,
                text: None,
                level: Level::Error,
                location: Some(span),
                hint: None,
                content: Some(content),
            },
            Self::FunctionAlreadyExists(msg, span, content)
            | Self::InvalidType(msg, span, content) => Diagnostic {
                title: string,
                text: None,
                level: Level::Error,
                location: Some(span),
                hint: None,
                content: Some(content),
            },
        }
    }
}
