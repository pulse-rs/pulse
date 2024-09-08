mod logger;
mod panic_handler;

use crate::commands::run::run_command;
use crate::logger::setup_logger;
use clap::{
    builder::{styling, PossibleValuesParser, Styles, TypedValueParser},
    Args, Parser, Subcommand, ValueHint,
};
use std::io;
use std::io::{stderr, BufWriter, Write};
use std::path::PathBuf;

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

fn main() {
    panic_handler::setup_panic_handler();
    let mut stderr = BufWriter::new(stderr());
    let program = Program::parse();
    setup_logger(program.verbose);

    let result = match &program.command {
        Commands::Run { file } => run_command(file.clone()),
    };

    match result {
        Ok(_) => {
            log::info!("Program finished successfully");
            log::error!("Program finished successfully");
            log::trace!("Program finished successfully");
            log::debug!("Program finished successfully");
            log::warn!("Program finished successfully");
        }
        Err(err) => {
            err.log_pretty(&mut stderr);
            stderr.flush().expect("Final result error writing");
            std::process::exit(1);
        }
    }
}
