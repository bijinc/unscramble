use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};

use crate::populate;
use crate::sort;

/// A CLI tool for organizing files in a directory
#[derive(Parser)]
#[command(name = "unscramble")]
#[command(about = "Semantic file organizer", long_about = None)]
pub struct Cli {
    /// The command to execute
    #[command(subcommand)]
    pub command: Commands,
    
    /// Path to the directory
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
    Sort {
        /// Sort files by extension
        #[arg(short='e', long, default_value = "false")]
        ext: bool,
        
        /// Sort files by name
        #[arg(short='n', long, default_value = "false")]
        name: bool,
        
        /// Recursively sort subdirectories
        #[arg(short='r', long, default_value = "false")]
        recursive: bool,
    },
}

/// Options for sorting files
#[derive(Debug, Clone)]
pub struct SortOptions {
    pub ext: bool,
    pub name: bool,
    pub recursive: bool,
}

impl Commands {
    /// Execute the command with the given path
    pub fn execute(&self, path: &Path) -> Result<(), String> {
        match self {
            Commands::Populate => {
                populate::populate_dir(path);
                Ok(())
            }
            Commands::Clear => {
                populate::clear_dir(path);
                Ok(())
            }
            Commands::Sort { ext, name, recursive } => {
                let options = SortOptions {
                    ext: *ext,
                    name: *name,
                    recursive: *recursive,
                };
                sort::sort_dir(path, &options);
                Ok(())
            }
        }
    }
}