use std::path::Path;
use std::fs;

fn main() {
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

    // list out all files in sandbox, returns `io::Result<Vec<Path>>`
    match fs::read_dir(sandbox_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            println!("> {:?}", path.unwrap().path());
        },
    }

    // sort by file extension
}