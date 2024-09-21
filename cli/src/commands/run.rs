use crate::cpp_compiler::{find_cpp_compiler_for_os, Compiler};
use crate::fs::normalize_path;
use crate::include_files;
use log::{debug, log};
use pulse_core::build::BuildProcess;
use pulse_core::error::error::Error;
use pulse_core::error::error::Error::{FileDoesNotExist, InvalidExtension};
use pulse_core::Result;
use std::fmt::format;
use std::path::PathBuf;
use std::process::Output;
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

    fs::write(full_path, content).map_err(Error::io)?;

    Ok(())
}

pub fn create_without_canonicalize(path: PathBuf, content: &str) -> Result<()> {
    log::debug!("Creating file: {:?}", path);

    fs::write(path, content).map_err(Error::io)?;

    Ok(())
}

pub fn build_dir() -> Result<PathBuf> {
    Ok(env::current_dir().map_err(Error::io)?.join("build"))
}

pub fn setup_build_dir() -> Result<()> {
    log::debug!("Setting up build directory");
    let build_dir = build_dir()?;

    if !build_dir.exists() {
        fs::create_dir(&build_dir).map_err(Error::io)?;
    }

    let std_dir = build_dir.join("std");

    if !std_dir.exists() {
        fs::create_dir(&std_dir).map_err(Error::io)?;
    }

    // TODO: improve this
    let files = include_files!("../../lib/lib.cpp", "../../lib/io.cpp");
    let names = vec!["lib.cpp".to_string(), "io.cpp".to_string()];

    for (index, content) in files.iter().enumerate() {
        let name = names.get(index).unwrap();

        let file = std_dir.join(name);

        fs::write(file, content).map_err(Error::io)?;
    }

    Ok(())
}

pub fn run_command(path: PathBuf) -> Result<()> {
    let (source, full_path) = resolve_file(path.clone())?;
    log::debug!("Running command with source: {}", source);
    setup_build_dir()?;

    let mut build = BuildProcess::new(source, full_path.clone());

    let code = build.compile()?;

    log::debug!("Generated code: {}", code);

    let build_dir = build_dir()?;
    let new_path = build_dir
        .join("source")
        .join(full_path.file_name().unwrap())
        .with_extension("cpp");

    log::debug!("Writing generated code to: {:?}", new_path);

    create_without_canonicalize(new_path.clone(), &code)?;

    let (cpp_compiler, looked_for) = find_cpp_compiler_for_os()?;

    if let Some(cpp_compiler) = cpp_compiler {
        log::debug!("Using C++ compiler: {:?}", cpp_compiler);

        compile_cpp_file(
            cpp_compiler,
            new_path.clone(),
            looked_for,
            build_dir.clone(),
        )?;

        let output = std::process::Command::new(
            build_dir.join(full_path.file_stem().unwrap().to_str().unwrap()),
        )
        .current_dir(build_dir)
        .output()
        .map_err(Error::io)?;

        display_output(output);
    } else {
        log::debug!("No C++ compiler found.");
        return Err(Error::CompilerNotFound(looked_for.to_string()));
    }

    Ok(())
}

pub fn compile_cpp_file(
    compiler_path: PathBuf,
    file: PathBuf,
    compiler: Compiler,
    out_dir: PathBuf,
) -> Result<()> {
    let file_stem = file.clone();
    let file_stem = file_stem.file_stem().unwrap().to_str().unwrap();

    log::debug!("Creating exe with name: {}", file_stem);
    let output = match compiler {
        Compiler::ClangPlus => std::process::Command::new(compiler_path)
            .arg(file)
            .arg("-o")
            .arg(file_stem)
            .current_dir(out_dir)
            .output()
            .map_err(Error::io)?,
        Compiler::Gcc => std::process::Command::new(compiler_path)
            .arg(file)
            .arg("-o")
            .arg(file_stem)
            .current_dir(out_dir)
            .output()
            .map_err(Error::io)?,
        Compiler::Msvc => std::process::Command::new(compiler_path)
            .arg(file)
            .arg(format!("/Fe{}", file_stem))
            .current_dir(out_dir)
            .output()
            .map_err(Error::io)?,
    };

    display_output(output);

    Ok(())
}

pub fn display_output(output: Output) {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    for line in stdout.lines() {
        println!("{}", line);
    }

    for line in stderr.lines() {
        eprintln!("{}", line);
    }
}
