mod cli;
mod editors;
mod opener;

use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {}", err);
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let command = cli::parse_args(editors::is_valid_editor)?;

    match command {
        cli::Command::Version => {
            cli::print_version();
        }
        cli::Command::Help => {
            cli::print_help();
        }
        cli::Command::List => {
            editors::list_editors();
        }
        cli::Command::Open { editor, path } => {
            let editor_name = editors::get_editor_name(&editor)?;
            opener::open_project(editor_name, &path)?;
        }
    }

    Ok(())
}
