use pulse_core::{error::error::Error, Result};
use std::fs;
use std::path::PathBuf;
use crate::commands::run::create_file;

pub fn init_command(name: Option<String>) -> Result<()> {
    let name = name.unwrap_or_else(|| "pulse_project".to_string());
    let path = PathBuf::from(name);

    fs::create_dir(&path).map_err(Error::io)?;
    let files: Vec<(&str, &str)> = vec![
        (
            "src/main.pulse",
            "fn main() {\n    println(\"Hello, World!\");\n}\n",
        ),
        (
            "pulse.toml",
            "[project]\nname = \"pulse_project\"\nversion = \"0.1.0\"\n",
        ),
        (".gitignore", "build/\n"),
    ];
    
    for (file, content) in files {
        create_file(path.join(file), content)?;
    }
    
    log::info!("Created project at {:?}", path);

    Ok(())
}
