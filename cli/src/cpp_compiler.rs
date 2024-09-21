use log::debug;
use pulse_core::error::error::Error;
use pulse_core::Result;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn check_msvc_available() -> (bool, Option<String>) {
    let msvc_paths = vec![
        r"C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvarsall.bat",
        r"C:\Program Files\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvarsall.bat",
    ];

    for path in msvc_paths {
        if Path::new(path).exists() {
            log::debug!("MSVC found at: {}", path);
            return (true, Some(path.to_string()));
        }
    }

    log::debug!("MSVC not found.");
    (false, None)
}

fn setup_msvc_environment(path_to_msvc: PathBuf) -> Result<HashMap<String, String>> {
    let output = Command::new("cmd")
        .args(&["/C", path_to_msvc.to_str().unwrap(), "x64", "&&", "set"])
        .stdout(Stdio::piped())
        .output()?;

    let mut env_vars = HashMap::new();

    if output.status.success() {
        let output = String::from_utf8_lossy(&output.stdout);

        for line in output.lines() {
            if let Some((key, value)) = line.split_once('=') {
                env_vars.insert(key.to_string(), value.to_string());
            }
        }
    }

    Ok(env_vars)
}

fn apply_env_vars(env_vars: HashMap<String, String>) {
    for (key, value) in env_vars {
        std::env::set_var(key, value);
    }
}

#[derive(Clone)]
pub enum Compiler {
    ClangPlus,
    Gcc,
    Msvc,
}

impl std::fmt::Display for Compiler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Compiler::ClangPlus => write!(f, "clang+="),
            Compiler::Gcc => write!(f, "g++"),
            Compiler::Msvc => write!(f, "cl"),
        }
    }
}

impl AsRef<OsStr> for Compiler {
    fn as_ref(&self) -> &OsStr {
        match self {
            Compiler::ClangPlus => "clang++".as_ref(),
            Compiler::Gcc => "g++".as_ref(),
            Compiler::Msvc => "cl".as_ref(),
        }
    }
}

pub fn looking_for() -> Compiler {
    if cfg!(target_os = "windows") {
        Compiler::Msvc
    } else if cfg!(target_os = "linux") {
        Compiler::Gcc
    } else if cfg!(target_os = "macos") {
        Compiler::ClangPlus
    } else {
        Compiler::Msvc
    }
}

pub fn find_cpp_compiler_for_os() -> Result<(Option<PathBuf>, Compiler)> {
    let looking_for = looking_for();

    if cfg!(target_os = "windows") {
        let (msvc_available, path_to_msvc) = check_msvc_available();
        if msvc_available && let Some(path) = path_to_msvc {
            let msvc_path = PathBuf::from(path);

            let env_vars = setup_msvc_environment(msvc_path)?;
            apply_env_vars(env_vars);

            let path = which::which(looking_for.clone())?;

            return Ok((Some(PathBuf::from(path)), looking_for));
        }
    } else {
        let path = which::which(looking_for.clone())?;

        return Ok((Some(PathBuf::from(path)), looking_for));
    }

    Ok((None, looking_for))
}
