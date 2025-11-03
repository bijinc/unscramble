use std::path::Path;
use std::fs;
use std::env;

mod populate;

fn main() {
    // get command line args
    let args: Vec<String> = env::args().collect();
    let cmd = handle_args(&args);

    const POPULATE_CMD: &str = "populate";
    const CLEAR_CMD: &str = "clear";

    // define params
    let sandbox_path = Path::new("./test");

    if sandbox_path.exists() {
        println!("Sandbox path set.");
    } else {
        match fs::create_dir(sandbox_path) {
            Ok(_) => println!("Sandbox directory created."),
            Err(e) => eprintln!("Failed to create sandbox directory: {}", e),
        }
    }

    match cmd.as_deref() {
        Some(POPULATE_CMD) => {
            populate::populate_dir(sandbox_path);
        },
        Some(CLEAR_CMD) => {
            populate::clear_dir(sandbox_path);
        },
        Some(unknown) => {
            println!("Unknown command: {}", unknown);
        },
        None => {
            // No command provided
        }
    }
}

fn handle_args(args: &[String]) -> Option<String> {
    if args.len() < 2 {
        println!("No command provided.");
        return None;
    }
    Some(args[1].clone())
}

fn list_files(path: &Path) {
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                println!("File: {:?}", path);
            } else if path.is_dir() {
                println!("Dir: {:?}", path);
                list_files(&path);
            }
        }
    }
}