use std::fs;
use clap::Parser;

mod populate;
mod cli;

use cli::Cli;

fn main() {
    let cli = Cli::parse();
    
    // Setup sandbox directory
    if cli.path.exists() {
        println!("Sandbox path set: {}", cli.path.display());
    } else {
        match fs::create_dir_all(&cli.path) {
            Ok(_) => println!("Sandbox directory created: {}", cli.path.display()),
            Err(e) => {
                eprintln!("Failed to create sandbox directory: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Execute the command
    if let Err(e) = cli.command.execute(&cli.path) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}