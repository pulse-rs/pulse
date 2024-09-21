#![feature(let_chains)]
#![allow(warnings, unused)]

pub mod cpp_compiler;
mod fs;
mod logger;
mod macros;
mod panic_handler;
use crate::commands::init::init_command;
use crate::commands::run::run_command;
use crate::logger::setup_logger;
use clap::{
    builder::{styling, PossibleValuesParser, Styles, TypedValueParser},
    Args, Parser, Subcommand, ValueHint,
};
use pulse_core::error::error::Error::NotImplemented;
use std::io;
use std::io::{stderr, BufWriter, Write};
use std::path::PathBuf;

pub mod commands {
    pub mod init;
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
    #[command(about = "Run a pulse file")]
    Run {
        #[arg(name = "FILE", value_hint = ValueHint::FilePath)]
        file: Option<PathBuf>,
    },

    #[command(about = "Initialize a new project")]
    Init {
        #[arg(name = "NAME")]
        name: Option<String>,
    },
}

fn main() {
    panic_handler::setup_panic_handler();
    let mut stderr = BufWriter::new(stderr());
    let program = Program::parse();
    setup_logger(program.verbose);

    let result = match &program.command {
        Commands::Run { file } => {
            if let Some(file) = file {
                run_command(file.clone())
            } else {
                // TODO: repl
                Err(NotImplemented("REPL".to_string()))
            }
        }
        Commands::Init { name } => init_command(name.clone()),
    };

    match result {
        Ok(_) => {
            log::debug!("Program finished successfully");
        }
        Err(err) => {
            err.log_pretty(&mut stderr);
            stderr.flush().expect("Final result error writing");
            std::process::exit(1);
        }
    }
}
