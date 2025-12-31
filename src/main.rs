use clap::Parser;
use cli::Cli;
use state::State;
use std::process::exit;

mod cli;
mod populate;
mod sort;
mod state;

fn main() {
    let cli = Cli::parse();

    // Load embeddings
    let mut state = State::new();
    state.load_embeddings();

    // Execute the command
    if let Err(e) = cli.command.execute(&state) {
        eprintln!("Error: {}", e);
        exit(1);
    }
}
