use pulse_ast::position::Position;
use std::io::Write;
use log::Level;
use termcolor::{Buffer, Color, WriteColor};
use crate::colors::ColorHandler;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub title: String,
    pub text: Option<String>,
    pub level: Level,
    pub location: Option<Position>,
    pub hint: Option<String>,
}

impl Diagnostic {
    pub fn log_pretty(&self, buff: &mut Buffer) {
        ColorHandler::set_color(buff, Color::Red, true);
        write!(buff, "{}", self.level.to_string().to_lowercase()).expect("Error writing level");
        ColorHandler::reset_color(buff);

        ColorHandler::set_dimmed_color(buff);
        write!(buff, ": ").expect("Error writing comma");
        ColorHandler::reset_color(buff);

        writeln!(buff, "{}", self.title).expect("Error writing title");

        if let Some(location) = &self.location {
            writeln!(buff, "Location: {location}").expect("Error writing location");
        }
        if let Some(text) = &self.text {
            writeln!(buff, "{}", text).expect("Error writing text");
        }

        self.print_hint(buff);
    }

    pub fn print_hint(&self, buff: &mut Buffer) {
        if let Some(hint) = &self.hint {
            ColorHandler::set_color(buff, Color::Cyan, false);
            writeln!(buff, "Hint: {hint}").expect("Error writing hint");
            ColorHandler::reset_color(buff);
        }
    }
}