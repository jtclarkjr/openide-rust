use std::env;
use std::process::{exit, Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "list" {
        println!("Available editors:");
        println!("  - xcode      (Xcode)");
        println!("  - vscode     (Visual Studio Code)");
        println!("  - webstorm   (WebStorm)");
        println!("  - pycharm    (PyCharm)");
        println!("  - pycharm-ce (PyCharm CE)");
        println!("  - intellij   (IntelliJ IDEA)");
        println!("  - goland     (GoLand)");
        println!("  - cursor     (Cursor)");
        println!("  - neovim     (Neovim)");
        exit(0);
    }

    if args.len() != 3 {
        eprintln!("Usage: {} <editor> <path_to_project>", args[0]);
        eprintln!("Or use: {} list - to see available editors", args[0]);
        exit(1);
    }

    let editor = &args[1];
    let project_path = &args[2];

    let editor_name = match editor.as_str() {
        "xcode" => "Xcode",
        "vscode" => "Visual Studio Code",
        "webstorm" => "WebStorm",
        "pycharm" => "PyCharm",
        "pycharm-ce" => "PyCharm CE",
        "intellij" => "IntelliJ IDEA",
        "goland" => "GoLand",
        "cursor" => "Cursor",
        "neovim" => "Neovim",
        _ => {
            eprintln!("Invalid IDE. Use '{} list' to see available editors.", args[0]);
            exit(1);
        }
    };

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
