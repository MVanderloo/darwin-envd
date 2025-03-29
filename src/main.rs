mod args;
mod path_utils;
mod processor;
mod shell;
mod variable_expansion;

use args::{Args, ShellType};
use clap::Parser;
use path_utils::{contract_path, expand_tilde};
use processor::process_directory;
use shell::detect_shell;
use std::process;

fn main() {
    let args = Args::parse();
    let debug_level = args.debug;

    let extension = if !args.extension.starts_with('.') {
        format!(".{}", args.extension)
    } else {
        args.extension.clone()
    };

    let shell = match args.shell {
        ShellType::Detect => detect_shell(),
        ShellType::Bash => shell::Shell::Bash,
        ShellType::Zsh => shell::Shell::Zsh,
        ShellType::Fish => shell::Shell::Fish,
    };

    if debug_level >= 1 {
        eprintln!("Parameters:");
        eprintln!("  Shell: {:?}", shell);
        eprintln!("  Extension: {}", extension);

        if args.paths.is_empty() {
            eprintln!("  Path: ~/.config/environment.d (default)");
        } else {
            eprintln!("  Paths:");
            for path in &args.paths {
                eprintln!("    {}", path);
            }
        }
    }

    let paths_to_process = if args.paths.is_empty() {
        vec!["~/.config/environment.d".to_string()]
    } else {
        args.paths.clone()
    };

    for path_str in paths_to_process {
        let path = expand_tilde(&path_str);

        if !path.exists() {
            if debug_level >= 1 {
                eprintln!("Path does not exist, skipping: {}", contract_path(&path));
            }
            continue;
        }

        if !path.is_dir() {
            eprintln!("Error: {} is not a directory", contract_path(&path));
            process::exit(1);
        }

        process_directory(&path, &shell, &extension, debug_level);
    }
}
