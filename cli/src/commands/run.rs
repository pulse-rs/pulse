use pulse_core::build::BuildProcess;
use pulse_core::error::error::Error;
use pulse_core::error::error::Error::{FileDoesNotExist, InvalidExtension};
use pulse_core::Result;
use std::path::PathBuf;
use std::{env, fs};
use crate::fs::normalize_path;

pub fn resolve_file(path: PathBuf) -> Result<(String, PathBuf)> {
    log::debug!("Resolving file: {:?}", path);
    let full_path = normalize_path(path, env::current_dir().map_err(Error::io)?)?;
    let extension = full_path.extension().unwrap();

    if extension != "pulse" {
        return Err(InvalidExtension(extension.to_str().unwrap().into()));
    }
    if !full_path.exists() {
        return Err(FileDoesNotExist);
    }

    let content = std::fs::read_to_string(&full_path)
        .map_err(Error::io)?
        .to_string();
    Ok((content, full_path))
}

pub fn setup_build_dir() -> Result<()> {
    let build_dir = env::current_dir().map_err(Error::io)?.join("build");

    if !build_dir.exists() {
        fs::create_dir(&build_dir).map_err(Error::io)?;
    }

    Ok(())
}

pub fn run_command(path: PathBuf) -> Result<()> {
    let (source, full_path) = resolve_file(path.clone())?;
    log::debug!("Running command with source: {}", source);

    let mut build = BuildProcess::new(source, full_path);

    build.compile()?;

    Ok(())
}
