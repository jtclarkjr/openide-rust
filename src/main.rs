mod cli;
mod editors;
mod opener;

use std::process::exit;

fn main() {
    let args = cli::get_args();

    if args.len() == 2 {
        match args[1].as_str() {
            "--version" | "-v" => {
                cli::print_version();
                exit(0);
            }
            "--help" | "-h" => {
                cli::print_help();
                exit(0);
            }
            "list" => {
                editors::list_editors();
                exit(0);
            }
            _ => {}
        }
    }

    if args.len() > 3 {
        eprintln!("Usage: {} [editor] [path_to_project]", args[0]);
        eprintln!("Or use: {} list - to see available editors", args[0]);
        eprintln!("Default editor is vscode if not specified");
        eprintln!("Default path is current directory if not specified");
        exit(1);
    }

    let (editor, project_path) = if args.len() == 1 {
        ("vscode", ".")
    } else if args.len() == 2 {
        if editors::is_valid_editor(&args[1]) {
            (args[1].as_str(), ".")
        } else {
            ("vscode", args[1].as_str())
        }
    } else {
        (args[1].as_str(), args[2].as_str())
    };

    let editor_name = editors::get_editor_name(editor).unwrap_or_else(|| {
        eprintln!(
            "Invalid IDE. Use '{} list' to see available editors.",
            args[0]
        );
        exit(1);
    });

    if let Err(err) = opener::open_project(editor_name, project_path) {
        eprintln!("Failed to open project: {}", err);
        exit(2);
    }
}
