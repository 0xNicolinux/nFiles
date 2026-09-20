use clap::Parser;
use std::io::{self, Write};
use std::path::PathBuf;

/// Cross-platform CLI tool to automatically organize files into categorized directories based on file extensions.
#[derive(Parser, Debug)]
#[command(name = "file-organizer", author, version, about, long_about = None)]
pub struct CliArgs {
    /// Path to the directory to organize
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Preview operations without modifying the filesystem
    #[arg(short, long)]
    pub dry_run: bool,

    /// Recursively process files in subdirectories
    #[arg(short, long)]
    pub recursive: bool,

    /// Display additional technical information
    #[arg(short, long)]
    pub verbose: bool,

    /// Force interactive prompts
    #[arg(short, long)]
    pub interactive: bool,

    /// Use platform-specific user directories
    #[arg(short = 'S', long)]
    pub system_directories: bool,
}

pub struct CliUserInterface;

impl CliUserInterface {
    pub fn prompt_directory() -> io::Result<String> {
        println!("File Organizer\n");
        print!("Enter the directory you want to organize:\n> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    pub fn prompt_confirm(prompt_msg: &str, default_yes: bool) -> io::Result<bool> {
        let default_str = if default_yes { "[Y/n]" } else { "[y/N]" };
        print!("{} {}: \n> ", prompt_msg, default_str);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let trimmed = input.trim().to_lowercase();

        if trimmed.is_empty() {
            Ok(default_yes)
        } else {
            Ok(trimmed == "y" || trimmed == "yes")
        }
    }
}
