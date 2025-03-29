use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value = "detect")]
    pub shell: ShellType,

    #[arg(long, default_value = ".conf")]
    pub extension: String,

    #[arg(short, long, default_value_t = 0)]
    pub debug: u8,

    #[arg(value_name = "PATHS")]
    pub paths: Vec<String>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ShellType {
    Detect,
    Bash,
    Zsh,
    Fish,
}
