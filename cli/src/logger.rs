use tracing_subscriber::prelude::*;

pub fn setup_logger(verbose: bool) {
    tracing_subscriber::fmt()
        .with_target(verbose)
        .with_ansi(true)
        .with_line_number(verbose)
        .with_file(verbose)
        .without_time()
        .with_env_filter(tracing_subscriber::EnvFilter::from(if verbose {
            "trace"
        } else {
            "info"
        }))
        .pretty()
        .init();
}
