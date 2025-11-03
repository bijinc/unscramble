use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};

use crate::populate;
use crate::sort;

/// A CLI tool for managing test sandbox directories
#[derive(Parser)]
#[command(name = "unscramble")]
#[command(about = "Semantic file organizer", long_about = None)]
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
    /// Populate the directory with test files
    Populate,
    /// Clear all files from the directory
    Clear,
    /// Sort files in the directory based on criteria
    Sort,
}

/// Options for sorting files
#[derive(Debug, Clone)]
pub struct Options {
    pub by_ext: bool,
    pub by_name: bool,
    pub recursive: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            by_ext: true,
            by_name: false,
            recursive: false,
        }
    }
}

impl Commands {
    /// Execute the command with the given path
    pub fn execute(&self, path: &Path, options: &Options) -> Result<(), String> {
        match self {
            Commands::Populate => {
                populate::populate_dir(path);
                Ok(())
            }
            Commands::Clear => {
                populate::clear_dir(path);
                Ok(())
            }
            Commands::Sort => {
                sort::sort_dir(path, options);
                Ok(())
            }
        }
    }
}