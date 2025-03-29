use std::env;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl Shell {
    pub fn get_command_executor(&self) -> &str {
        match self {
            Shell::Bash => "bash",
            Shell::Zsh => "zsh",
            Shell::Fish => "fish",
        }
    }

    pub fn format_export(&self, key: &str, value: &str) -> String {
        // Remove any surrounding quotes if they exist
        let clean_value = value.trim_matches(|c| c == '\'' || c == '"');

        match self {
            Shell::Bash | Shell::Zsh => format!("export {}='{}'", key, clean_value),
            Shell::Fish => format!("set -gx {} '{}'", key, clean_value),
        }
    }
}

pub fn detect_shell() -> Shell {
    if let Ok(shell) = env::var("SHELL") {
        if shell.contains("bash") {
            return Shell::Bash;
        } else if shell.contains("zsh") {
            return Shell::Zsh;
        } else if shell.contains("fish") {
            return Shell::Fish;
        }
    }
    Shell::Bash
}

pub fn execute_command(cmd: &str, shell: &Shell, debug_level: u8) -> String {
    let shell_executor = shell.get_command_executor();

    match Command::new(shell_executor)
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
    {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(e) => {
            if debug_level >= 1 {
                eprintln!("Error executing command '{}': {}", cmd, e);
            }
            String::new()
        }
    }
}
