use std::process::Command;

pub fn open_project(editor_name: &str, path: &str) -> Result<(), std::io::Error> {
    Command::new("open")
        .arg("-a")
        .arg(editor_name)
        .arg(path)
        .status()?;
    Ok(())
}
