use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

const DEFAULT_EDITOR: &str = "vscode";

fn config_dir() -> Result<PathBuf, io::Error> {
    if let Ok(dir) = env::var("XDG_CONFIG_HOME") {
        return Ok(PathBuf::from(dir));
    }

    let home = env::var("HOME").map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "HOME environment variable not set",
        )
    })?;

    Ok(PathBuf::from(home).join(".config"))
}

fn default_editor_path() -> Result<PathBuf, io::Error> {
    Ok(config_dir()?.join("openide").join("default-editor"))
}

pub fn load_default_editor(is_valid_editor: impl Fn(&str) -> bool) -> Result<String, io::Error> {
    let path = default_editor_path()?;
    match fs::read_to_string(&path) {
        Ok(contents) => {
            let editor = contents.trim();
            if !editor.is_empty() && is_valid_editor(editor) {
                Ok(editor.to_string())
            } else {
                Ok(DEFAULT_EDITOR.to_string())
            }
        }
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(DEFAULT_EDITOR.to_string()),
        Err(err) => Err(err),
    }
}

pub fn set_default_editor(editor: &str) -> Result<(), io::Error> {
    let path = default_editor_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, format!("{editor}\n"))?;
    Ok(())
}

pub fn reset_default_editor() -> Result<(), io::Error> {
    let path = default_editor_path()?;
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err),
    }
}
