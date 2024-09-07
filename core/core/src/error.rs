use crate::diagnostics::Diagnostic;
use termcolor::Buffer;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("{0}")]
    Generic(String, Option<String>),
    #[error("IO error: {0}")]
    Io(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::Generic(s, None)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}

impl Error {
    pub fn log_pretty(self, buffer: &mut Buffer) {
        let diagnostic = self.clone().into_diagnostic();

        diagnostic.log_pretty(buffer);
    }

    pub fn into_diagnostic(self) -> Diagnostic {
        match self.clone() {
            Self::Generic(title, msg) => Diagnostic {
                title,
                text: msg,
                level: tracing::Level::ERROR,
                location: None,
                hint: Some("This is a generic error".to_string()),
            },
            Self::Io(msg) => Diagnostic {
                title: msg,
                text: None,
                level: tracing::Level::ERROR,
                location: None,
                hint: None,
            },
        }
    }
}
