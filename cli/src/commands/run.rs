use pulse_core::build::BuildProcess;
use pulse_core::error::error::Error;
use pulse_core::error::error::Error::{FileDoesNotExist, InvalidExtension};
use pulse_core::Result;
use std::path::PathBuf;
use std::{env, fs};

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
        .map_err(Error::io)?
        .to_string())
}

pub fn setup_build_dir() -> Result<()> {
    let build_dir = env::current_dir().map_err(Error::io)?.join("build");

    if !build_dir.exists() {
        fs::create_dir(&build_dir).map_err(Error::io)?;
    }

    Ok(())
}

pub fn run_command(path: PathBuf) -> Result<()> {
    let source = resolve_file(path.clone())?;
    log::debug!("Running command with source: {}", source);

    let mut build = BuildProcess::new(source, path);

    build.compile()?;

    Ok(())
}
