use pulse_core::error::error::Error;
use pulse_core::Result;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

pub fn normalize_path(mut path: PathBuf, root: PathBuf) -> Result<PathBuf> {
    if path.is_relative() {
        path = root.join(path);
    }

    path = match path.canonicalize() {
        Ok(path) => path,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                let parent = path.parent().unwrap();
                log::debug!("Creating parent: {:?}", parent);
                fs::create_dir_all(parent).map_err(Error::io)?;
            }
            return Err(Error::Io(e));
        }
    };
    let root_str = path.to_str().unwrap_or("");
    // On windows, paths can be prefixed with \\?\ to allow longer paths, we need to remove this prefix
    let normalized_root_str = if root_str.starts_with(r"\\?\") {
        root_str.strip_prefix(r"\\?\").unwrap()
    } else {
        root_str
    };
    path = PathBuf::from(normalized_root_str);

    Ok(path)
}
