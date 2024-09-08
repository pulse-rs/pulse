use colored::Colorize;
use log::Level;
use pulse_ast::position::Position;
use std::io::{BufWriter, Stderr, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub title: String,
    pub text: Option<String>,
    pub level: Level,
    pub location: Option<Position>,
    pub hint: Option<String>,
}

impl Diagnostic {
    pub fn log_pretty(&self, buff: &mut BufWriter<Stderr>) {
        writeln!(
            buff,
            "{}{}{}",
            self.level.to_string().to_lowercase().bright_red(),
            ": ".dimmed(),
            self.title
        )
        .expect("Error writing level");

        if let Some(location) = &self.location {
            // TODO: implement location printing
        }
        if let Some(text) = &self.text {
            writeln!(buff, "{}", text).expect("Error writing text");
        }

        self.print_hint(buff);
    }

    pub fn print_hint(&self, buff: &mut BufWriter<Stderr>) {
        if let Some(hint) = &self.hint {
            writeln!(buff, "{}{}", "Hint: ".bright_cyan(), hint.bright_cyan())
                .expect("Error writing hint");
        }
    }
}
