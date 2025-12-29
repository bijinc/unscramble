use std::fs;
use clap::Parser;

mod populate;
mod sort;
mod cli;

use cli::Cli;

fn main() {
    let cli = Cli::parse();

    // Execute the command
    if let Err(e) = cli.command.execute() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
