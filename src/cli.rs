use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
    println!();
    println!("ARGS:");
    println!("    [EDITOR]         Editor to use (default: vscode)");
    println!("    [PATH]           Path to open (default: current directory)");
    println!();
    println!("EXAMPLES:");
    println!("    openide                    # Open current directory in VS Code");
    println!("    openide cursor             # Open current directory in Cursor");
    println!("    openide /path/to/project   # Open path in VS Code");
    println!("    openide cursor ~/project   # Open path in Cursor");
}

pub fn get_args() -> Vec<String> {
    env::args().collect()
}
