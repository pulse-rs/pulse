use crate::ast::span::TextSpan;
use colored::Colorize;
use log::Level;
use std::io::{BufWriter, Stderr, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub title: String,
    pub text: Option<String>,
    pub level: Level,
    pub location: Option<TextSpan>,
    pub hint: Option<String>,
    pub content: Option<String>,
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
            if let Some(content) = &self.content {
                let line = content
                    .lines()
                    .nth(location.start.line as usize)
                    .expect("Error getting line");
                let line_number = location.start.line;
                let column = location.start.column;
                let line_content = line.trim_end();
                let decoration =
                    "^".repeat(location.end.column as usize - location.start.column as usize);

                writeln!(buff, "{} {}:{}", "--->".cyan(), line_number, column)
                    .expect("Error writing line number");

                if line_number > 1 {
                    let line_before = format!("{} |", line_number - 1);
                    writeln!(buff, "{}", line_before.cyan()).expect("Error writing line number");
                }

                let line_current = format!("{} |", line_number);
                write!(buff, "{}", line_current.cyan()).expect("Error writing line number");
                writeln!(buff, "    {}", line_content).expect("Error writing content");

                let padding_left =
                    " ".repeat((column + 6 + line_number.to_string().len() as u32) as usize);
                writeln!(buff, "{}{}", padding_left, decoration.bright_red())
                    .expect("Error writing decoration");

                if line_number > 1 {
                    let line_after = format!("{} |", line_number + 1);
                    writeln!(buff, "{}", line_after.cyan()).expect("Error writing line number");
                }
            }
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
