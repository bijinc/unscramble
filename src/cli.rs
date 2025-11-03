use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};

use crate::populate;

/// A CLI tool for managing test sandbox directories
#[derive(Parser)]
#[command(name = "unscramble")]
#[command(about = "Manage test sandbox directories", long_about = None)]
pub struct Cli {
    /// The command to execute
    #[command(subcommand)]
    pub command: Commands,
    
    /// Path to the sandbox directory
    #[arg(short, long, default_value = "./test")]
    pub path: PathBuf,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Populate the sandbox directory with test files
    Populate,
    /// Clear all files from the sandbox directory
    Clear,
}

impl Commands {
    /// Execute the command with the given sandbox path
    pub fn execute(&self, sandbox_path: &Path) -> Result<(), String> {
        match self {
            Commands::Populate => {
                populate::populate_dir(sandbox_path);
                Ok(())
            }
            Commands::Clear => {
                populate::clear_dir(sandbox_path);
                Ok(())
            }
        }
    }
}