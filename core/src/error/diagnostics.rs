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

        if let Some(location) = &self.location
            && let Some(content) = &self.content
        {
            let to_print = content
                .chars()
                .skip(location.start)
                .take(location.end - location.start);
            let to_print = to_print.collect::<String>();
            let line = content
                .lines()
                .enumerate()
                .find(|(_, line)| line.contains(&to_print))
                .map(|(i, _)| i)
                .unwrap_or(0);
            let decoration =
                " ".repeat(location.start - 1) + &"^".repeat(location.end - location.start);

            let line_number = line + 1;
            let column = location.start + 1;

            writeln!(buff, "{} {}:{}", "--->".cyan(), line_number, column)
                .expect("Error writing line number");

            if line_number > 1 {
                let line_before = format!("{} |", line_number - 1);
                writeln!(buff, "{}", line_before.cyan()).expect("Error writing line number");
            }

            let line_current = format!("{} |", line_number);
            write!(buff, "{}", line_current.cyan()).expect("Error writing line number");
            writeln!(buff, "    {}", content.lines().nth(line).unwrap())
                .expect("Error writing content");
            let padding_left = " ".repeat(column + 4);
            writeln!(buff, "{}{}", padding_left.cyan(), decoration.bright_red())
                .expect("Error writing decoration");

            if line_number > 1 {
                let line_after = format!("{} |", line_number + 1);
                writeln!(buff, "{}", line_after.cyan()).expect("Error writing line number");
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
