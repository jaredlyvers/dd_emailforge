mod emit;
mod model;
mod paths;
mod storage;
mod tui;
mod validate;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

use storage::{load_template, resolve_template_path, LoadError};
use validate::validate_template_with_root;

#[derive(Debug, Parser)]
#[command(
    name = "dd_emailforge",
    version,
    about = "Terminal-UI email template builder"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Open the TUI on a template.json or a folder containing one.
    Tui { path: Option<PathBuf> },
    /// Structural + image validation. Non-zero exit on errors. Warnings on stderr.
    Validate { path: PathBuf },
    /// Pretty-print the loaded template JSON to stdout.
    Show { path: PathBuf },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Command::Tui { path } => match tui::run_tui(path) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("{e:#}");
                ExitCode::from(1)
            }
        },
        Command::Validate { path } => cmd_validate(&path),
        Command::Show { path } => cmd_show(&path),
    }
}

fn cmd_validate(path: &PathBuf) -> ExitCode {
    let json_path = match resolve_template_path(path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{e:#}");
            return ExitCode::from(1);
        }
    };
    let template = match load_cli(&json_path) {
        Ok(t) => t,
        Err(code) => return code,
    };
    let root = storage::template_root(&json_path);
    let report = validate_template_with_root(&template, Some(&root));
    for err in &report.errors {
        eprintln!("- {err}");
    }
    for warn in &report.warnings {
        eprintln!("warning: {warn}");
    }
    if report.ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn cmd_show(path: &PathBuf) -> ExitCode {
    let json_path = match resolve_template_path(path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{e:#}");
            return ExitCode::from(1);
        }
    };
    let template = match load_cli(&json_path) {
        Ok(t) => t,
        Err(code) => return code,
    };
    match serde_json::to_string_pretty(&template) {
        Ok(mut s) => {
            if !s.ends_with('\n') {
                s.push('\n');
            }
            print!("{s}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("failed to serialize template: {e}");
            ExitCode::from(1)
        }
    }
}

fn load_cli(path: &std::path::Path) -> Result<crate::model::Template, ExitCode> {
    match load_template(path) {
        Ok(t) => Ok(t),
        Err(LoadError::MissingVersion) => {
            eprintln!("missing template.json version (expected 1)");
            Err(ExitCode::from(2))
        }
        Err(LoadError::UnsupportedVersion(n)) => {
            eprintln!("unsupported template.json version {n} (expected 1)");
            Err(ExitCode::from(2))
        }
        Err(LoadError::Parse(msg)) => {
            eprintln!("failed to parse template.json: {msg}");
            Err(ExitCode::from(2))
        }
        Err(LoadError::Io(e)) => {
            eprintln!("failed to read template.json: {e}");
            Err(ExitCode::from(2))
        }
    }
}
