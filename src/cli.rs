use std::path::PathBuf;
use clap::{Parser, Subcommand, value_parser};

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
    
    // /// Path to the directory
    // #[arg(short, long, default_value = "./test")]
    // pub path: PathBuf,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Populate the directory with test files
    Populate {
        /// Directory to populate
        #[arg(short='p', long, value_parser = value_parser!(PathBuf), default_value = "./test")]
        path: PathBuf,
    },
    
    /// Clear all files from the directory
    Clear {
        /// Directory to clear
        #[arg(short='p', long, value_parser = value_parser!(PathBuf), default_value = "./test")]
        path: PathBuf,
    },
    
    /// Sort files in the directory based on criteria
    Sort {
        /// Directory to sort
        #[arg(short='p', long, value_parser = value_parser!(PathBuf), default_value = ".")]
        path: PathBuf,

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
    pub fn execute(&self) -> Result<(), String> {
        match self {
            Commands::Populate { path } => {
                validate_path(&path);
                populate::populate_dir(path);
                Ok(())
            }
            Commands::Clear { path } => {
                validate_path(&path);
                populate::clear_dir(path);
                Ok(())
            }
            Commands::Sort { path, ext, name, recursive } => {
                validate_path(&path);
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

fn validate_path(path: &PathBuf) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("Path does not exist: {}", path.display()));
    }
    if !path.is_dir() {
        return Err(format!("Path is not a directory: {}", path.display()));
    }
    Ok(())
}