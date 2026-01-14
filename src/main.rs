mod cli;
mod config;
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
    let default_editor = config::load_default_editor(editors::is_valid_editor)?;
    let command = cli::parse_args(editors::is_valid_editor, &default_editor)?;

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
        cli::Command::SetDefault { editor } => {
            if !editors::is_valid_editor(&editor) {
                return Err(Box::new(editors::InvalidEditorError { editor }));
            }
            config::set_default_editor(&editor)?;
            println!("Default editor set to {}", editor);
        }
        cli::Command::ResetDefault => {
            config::reset_default_editor()?;
            println!("Default editor reset to vscode");
        }
        cli::Command::Open { editor, path } => {
            let editor_name = editors::get_editor_name(&editor)?;
            opener::open_project(editor_name, &path)?;
        }
    }

    Ok(())
}
