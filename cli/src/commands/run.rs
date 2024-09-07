use pulse_core::error::Error;
use pulse_core::Result;
use std::path::PathBuf;

pub fn run_command(path: Option<PathBuf>) -> Result<()> {
    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Not implemented").into());

    Ok(())
}
