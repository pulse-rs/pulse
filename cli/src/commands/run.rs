use crate::fs::normalize_path;
use crate::include_files;
use pulse_core::build::BuildProcess;
use pulse_core::error::error::Error;
use pulse_core::error::error::Error::{FileDoesNotExist, InvalidExtension};
use pulse_core::Result;
use std::path::PathBuf;
use std::{env, fs};
// TODO: If no file was provided. Look recursively for a pulse.toml file and resolve the main file from there

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

pub fn create_file(path: PathBuf, content: &str) -> Result<()> {
    log::debug!("Creating file: {:?}", path);
    let full_path = normalize_path(path, env::current_dir().map_err(Error::io)?)?;

    fs::create_dir_all(full_path.parent().unwrap()).map_err(Error::io)?;
    fs::write(full_path, content).map_err(Error::io)?;

    Ok(())
}

pub fn setup_build_dir() -> Result<()> {
    log::debug!("Setting up build directory");
    let build_dir = env::current_dir().map_err(Error::io)?.join("build");

    if !build_dir.exists() {
        fs::create_dir(&build_dir).map_err(Error::io)?;
    }

    let std_dir = build_dir.join("std");

    if !std_dir.exists() {
        fs::create_dir(&std_dir).map_err(Error::io)?;
    }

    // TODO: maybe simplify this
    let files = include_files!("../../lib/main.cpp");
    let names = vec!["main.cpp".to_string()];

    for content in files.iter() {
        for name in names.iter() {
            let file = std_dir.join(name);

            fs::write(file, content).map_err(Error::io)?;
        }
    }

    Ok(())
}

pub fn run_command(path: PathBuf) -> Result<()> {
    let (source, full_path) = resolve_file(path.clone())?;
    log::debug!("Running command with source: {}", source);
    setup_build_dir()?;

    let mut build = BuildProcess::new(source, full_path);

    build.compile()?;

    Ok(())
}
