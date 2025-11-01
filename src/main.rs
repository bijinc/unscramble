use std::path::Path;
use std::fs;

fn main() {
    // define sandbox dir
    let sandbox_path = Path::new("./test");

    if sandbox_path.exists() {
        println!("Sandbox path set.");
    } else {
        match fs::create_dir(sandbox_path) {
            Ok(_) => println!("Sandbox directory created."),
            Err(e) => eprintln!("Failed to create sandbox directory: {}", e),
        }
    }
}