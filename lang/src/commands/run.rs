use pulse_core::build::BuildProcess;
use pulse_core::error::error::Error;
use pulse_core::error::error::Error::{FileDoesNotExist, InvalidExtension};
use pulse_core::Result;
use std::path::PathBuf;

pub fn resolve_file(path: PathBuf) -> Result<String> {
    log::debug!("Resolving file: {:?}", path);
    let extension = path.extension().unwrap();

    if extension != "pulse" {
        return Err(InvalidExtension(extension.to_str().unwrap().into()));
    }
    if !path.exists() {
        return Err(FileDoesNotExist);
    }

    Ok(std::fs::read_to_string(path)
        .map_err(|err| Error::io(err))?
        .to_string())
}

pub fn run_command(path: PathBuf) -> Result<()> {
    let source = resolve_file(path)?;
    log::debug!("Running command with source: {}", source);

    let build = BuildProcess::with_input(source);

    build.compile();

    Ok(())
}
