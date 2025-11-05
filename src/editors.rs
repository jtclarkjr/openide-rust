pub const EDITORS: &[(&str, &str)] = &[
    ("xcode", "Xcode"),
    ("vscode", "Visual Studio Code"),
    ("webstorm", "WebStorm"),
    ("pycharm-ce", "PyCharm CE"),
    ("intellij", "IntelliJ IDEA"),
    ("goland", "GoLand"),
    ("cursor", "Cursor"),
    ("neovim", "Neovim"),
];

pub fn is_valid_editor(name: &str) -> bool {
    EDITORS.iter().any(|(key, _)| *key == name)
}

pub fn get_editor_name(editor_key: &str) -> Option<&str> {
    EDITORS
        .iter()
        .find(|(key, _)| *key == editor_key)
        .map(|(_, name)| *name)
}

pub fn list_editors() {
    println!("Available editors:");
    for (key, name) in EDITORS {
        println!("  - {:<12} ({})", key, name);
    }
}
