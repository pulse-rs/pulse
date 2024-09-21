use crate::cpp_compiler::{find_cpp_compiler_for_os, Compiler};
use crate::fs::normalize_path;
use crate::include_files;
use crate::time::format_time;
use colored::Colorize;
use log::debug;
use pulse_core::build::BuildProcess;
use pulse_core::error::error::Error;
use pulse_core::Result;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::{env, fs};

pub fn resolve_file(path: PathBuf) -> Result<(String, PathBuf)> {
    debug!("Resolving file: {:?}", path);
    let full_path = normalize_path(path, env::current_dir().map_err(Error::io)?)?;

    let extension = full_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    if extension != "pulse" {
        return Err(Error::InvalidExtension(extension.into()));
    }

    if !full_path.exists() {
        return Err(Error::FileDoesNotExist);
    }

    let content = fs::read_to_string(&full_path).map_err(Error::io)?;
    Ok((content, full_path))
}

pub fn create_file(path: PathBuf, content: &str) -> Result<()> {
    debug!("Creating file: {:?}", path);
    let full_path = normalize_path(path, env::current_dir().map_err(Error::io)?)?;
    fs::write(full_path, content).map_err(Error::io)
}

pub fn create_without_canonicalize(path: PathBuf, content: &str) -> Result<()> {
    debug!("Creating file without canonicalization: {:?}", path);
    fs::write(path, content).map_err(Error::io)
}

pub fn build_dir() -> Result<PathBuf> {
    env::current_dir()
        .map_err(Error::io)
        .map(|dir| dir.join("build"))
}

pub fn setup_build_dir() -> Result<()> {
    debug!("Setting up build directory");
    let build_dir = build_dir()?;

    fs::create_dir_all(build_dir.join("std")).map_err(Error::io)?;

    // Include and write files
    let files = include_files!("../../lib/lib.cpp", "../../lib/io.cpp");
    let names = vec!["lib.cpp", "io.cpp"];

    for (name, content) in names.into_iter().zip(files) {
        let file_path = build_dir.join("std").join(name);
        fs::write(file_path, content).map_err(Error::io)?;
    }

    Ok(())
}

pub fn run_command(path: PathBuf) -> Result<()> {
    let (source, full_path) = resolve_file(path)?;
    debug!("Running command with source: {}", source);
    setup_build_dir()?;

    let transpile_start = Instant::now();
    let mut build = BuildProcess::new(source, full_path.clone());
    let code = build.compile()?;

    println!(
        "  {} {}",
        "Transpiled in".bright_cyan(),
        format_time(transpile_start)
    );
    debug!("Generated code: {}", code);

    let new_path = build_dir()?
        .join("source")
        .join(full_path.file_name().unwrap())
        .with_extension("cpp");

    create_without_canonicalize(new_path.clone(), &code)?;

    let (cpp_compiler, looked_for) = find_cpp_compiler_for_os()?;
    if let Some(cpp_compiler) = cpp_compiler {
        debug!("Using C++ compiler: {:?}", cpp_compiler);

        let compile_start = Instant::now();
        compile_cpp_file(cpp_compiler, new_path.clone(), looked_for, build_dir()?)?;
        println!(
            "    {} {}",
            "Compiled in".bright_cyan(),
            format_time(compile_start)
        );

        let exe_path = build_dir()?.join(
            full_path
                .with_extension("exe")
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        );
        println!("        {} {}", "Running".bright_cyan(), exe_path.display());

        let output = std::process::Command::new(exe_path)
            .current_dir(build_dir()?)
            .output()
            .map_err(Error::io)?;

        display_output(output);
    } else {
        debug!("No C++ compiler found.");
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
    debug!("Creating exe with name: {}", file_stem);

    let output = match compiler {
        Compiler::ClangPlus | Compiler::Gcc => std::process::Command::new(compiler_path)
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

pub fn display_output(output: std::process::Output) {
    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
}
