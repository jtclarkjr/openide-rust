use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub enum Command {
    Version,
    Help,
    List,
    SetDefault { editor: String },
    ResetDefault,
    Open { editor: String, path: String },
}

#[derive(Debug)]
pub enum ParseError {
    TooManyArgs,
    MissingDefaultEditor,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::TooManyArgs => write!(
                f,
                "Too many arguments. Usage: openide [editor] [path_to_project]"
            ),
            ParseError::MissingDefaultEditor => write!(
                f,
                "Missing editor. Usage: openide default <editor_key>"
            ),
        }
    }
}

impl std::error::Error for ParseError {}

pub fn print_version() {
    println!("openide {}", VERSION);
}

pub fn print_help() {
    println!("openide {}", VERSION);
    println!();
    println!("USAGE:");
    println!("    openide [OPTIONS] [EDITOR] [PATH]");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help       Print help information");
    println!("    -v, --version    Print version information");
    println!("    list             List available editors");
    println!("    default          Set the default editor");
    println!("    reset-default    Reset the default editor to vscode");
    println!();
    println!("ARGS:");
    println!("    [EDITOR]         Editor to use (default: configured)");
    println!("    [PATH]           Path to open (default: current directory)");
    println!();
    println!("EXAMPLES:");
    println!("    openide                    # Open current directory in VS Code");
    println!("    openide cursor             # Open current directory in Cursor");
    println!("    openide /path/to/project   # Open path in VS Code");
    println!("    openide cursor ~/project   # Open path in Cursor");
    println!("    openide default cursor     # Set default editor to Cursor");
    println!("    openide reset-default      # Reset default editor to VS Code");
}

pub fn parse_args(
    is_valid_editor: impl Fn(&str) -> bool,
    default_editor: &str,
) -> Result<Command, ParseError> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        match args[1].as_str() {
            "--version" | "-v" => return Ok(Command::Version),
            "--help" | "-h" => return Ok(Command::Help),
            "list" => return Ok(Command::List),
            "reset-default" => return Ok(Command::ResetDefault),
            _ => {}
        }
    }

    if args.len() > 3 {
        return Err(ParseError::TooManyArgs);
    }

    if args.len() >= 2 && args[1].as_str() == "default" {
        if args.len() < 3 {
            return Err(ParseError::MissingDefaultEditor);
        }
        return Ok(Command::SetDefault {
            editor: args[2].clone(),
        });
    }

    let (editor, path) = match args.len() {
        1 => (default_editor.to_string(), ".".to_string()),
        2 => {
            if is_valid_editor(&args[1]) {
                (args[1].clone(), ".".to_string())
            } else {
                (default_editor.to_string(), args[1].clone())
            }
        }
        _ => (args[1].clone(), args[2].clone()),
    };

    Ok(Command::Open { editor, path })
}
