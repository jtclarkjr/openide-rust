use std::env;
use std::process::{exit, Command};

const EDITORS: &[(&str, &str)] = &[
    ("xcode", "Xcode"),
    ("vscode", "Visual Studio Code"),
    ("webstorm", "WebStorm"),
    ("pycharm-ce", "PyCharm CE"),
    ("intellij", "IntelliJ IDEA"),
    ("goland", "GoLand"),
    ("cursor", "Cursor"),
    ("neovim", "Neovim"),
];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "list" {
        println!("Available editors:");
        for (key, name) in EDITORS {
            println!("  - {:<12} ({})", key, name);
        }
        exit(0);
    }

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} [editor] <path_to_project>", args[0]);
        eprintln!("Or use: {} list - to see available editors", args[0]);
        eprintln!("Default editor is vscode if not specified");
        exit(1);
    }

    // Check if arg is a valid editor
    let is_valid_editor = |name: &str| -> bool { EDITORS.iter().any(|(key, _)| *key == name) };

    let (editor, project_path) = if args.len() == 2 {
        // Check if the single arg is an editor name without a path
        if is_valid_editor(&args[1]) {
            eprintln!("Error: Missing project path");
            eprintln!("Usage: {} [editor] <path_to_project>", args[0]);
            exit(1);
        }
        ("vscode", &args[1])
    } else {
        (args[1].as_str(), &args[2])
    };

    let editor_name = EDITORS
        .iter()
        .find(|(key, _)| *key == editor)
        .map(|(_, name)| *name)
        .unwrap_or_else(|| {
            eprintln!(
                "Invalid IDE. Use '{} list' to see available editors.",
                args[0]
            );
            exit(1);
        });

    if let Err(err) = open_project(editor_name, project_path) {
        eprintln!("Failed to open project: {}", err);
        exit(2);
    }
}

fn open_project(editor_name: &str, path: &str) -> Result<(), std::io::Error> {
    Command::new("open")
        .arg("-a")
        .arg(editor_name)
        .arg(path)
        .status()?;
    Ok(())
}
