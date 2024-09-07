mod logger;

use crate::commands::run::run_command;
use crate::logger::setup_logger;
use clap::{
    builder::{styling, PossibleValuesParser, Styles, TypedValueParser},
    Args, Parser, Subcommand, ValueHint,
};
use std::io;
use std::path::PathBuf;
use termcolor::{BufferWriter, ColorChoice};

pub mod commands {
    pub mod run;
}

#[derive(Debug, Parser)]
#[command(author, version, about, name = "pulse",
styles = Styles::styled()
        .header(styling::AnsiColor::Yellow.on_default())
        .usage(styling::AnsiColor::Yellow.on_default())
        .literal(styling::AnsiColor::Green.on_default()))]
struct Program {
    #[arg(global = true, short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Run {
        #[arg(name = "FILE", value_hint = ValueHint::FilePath)]
        file: Option<PathBuf>,
    },
}

#[tracing::instrument]
fn main() {
    let stderr = BufferWriter::stderr(ColorChoice::Always);
    let program = Program::parse();
    setup_logger(program.verbose);

    let result = match &program.command {
        Commands::Run { file } => run_command(file.clone()),
    };

    match result {
        Ok(_) => {
            tracing::info!("Program finished successfully");
        }
        Err(err) => {
            let mut buffer = stderr.buffer();
            err.log_pretty(&mut buffer);
            stderr.print(&buffer).expect("Final result error writing");
            std::process::exit(1);
        }
    }
}
