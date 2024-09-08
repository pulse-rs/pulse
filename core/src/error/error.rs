use log::Level;
use std::io::{BufWriter, Stderr};
use thiserror::Error;
use crate::error::diagnostics::Diagnostic;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Generic(String, Option<String>),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
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
        match self {
            Self::Generic(title, msg) => Diagnostic {
                title,
                text: msg,
                level: Level::Error,
                location: None,
                hint: Some("This is a generic error".to_string()),
            },
            Self::Io(msg) => Diagnostic {
                title: msg.to_string(),
                text: None,
                level: Level::Error,
                location: None,
                hint: None,
            },
        }
    }
}
