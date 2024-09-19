use pulse_core::{error::error::Error, Result};
use std::fs;
use std::path::PathBuf;

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
        let file_path = path.join(file);
        fs::create_dir_all(file_path.parent().unwrap()).map_err(Error::io)?;
        fs::write(file_path, content).map_err(Error::io)?;
    }
    
    log::info!("Created project at {:?}", path);

    Ok(())
}
