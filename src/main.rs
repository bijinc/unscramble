use clap::Parser;

mod cli;
mod populate;
mod sort;
mod state;

use cli::Cli;
use state::State;

fn main() {
    let cli = Cli::parse();

    // Load embeddings
    let mut state = State::new();
    state.load_embeddings();

    // Execute the command
    if let Err(e) = cli.command.execute(&state) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
