use std::fmt;

pub const EDITORS: &[(&str, &str)] = &[
    ("xcode", "Xcode"),
    ("vscode", "Visual Studio Code"),
    ("webstorm", "WebStorm"),
    ("pycharm-ce", "PyCharm CE"),
    ("intellij", "IntelliJ IDEA"),
    ("goland", "GoLand"),
    ("cursor", "Cursor"),
];

#[derive(Debug)]
pub struct InvalidEditorError {
    pub editor: String,
}

impl fmt::Display for InvalidEditorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid IDE '{}'. Use 'openide list' to see available editors.",
            self.editor
        )
    }
}

impl std::error::Error for InvalidEditorError {}

pub fn is_valid_editor(name: &str) -> bool {
    EDITORS.iter().any(|(key, _)| *key == name)
}

pub fn get_editor_name(editor_key: &str) -> Result<&str, InvalidEditorError> {
    EDITORS
        .iter()
        .find(|(key, _)| *key == editor_key)
        .map(|(_, name)| *name)
        .ok_or_else(|| InvalidEditorError {
            editor: editor_key.to_string(),
        })
}

pub fn list_editors() {
    println!("Available editors:");
    for (key, name) in EDITORS {
        println!("  - {:<12} ({})", key, name);
    }
}
