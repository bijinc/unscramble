use std::path::Path;
use std::fs;

pub fn populate_dir(path: &Path) {
    println!("Populating dir: {}", path.display());

    fs::create_dir_all(path.join("subdir1")).unwrap();
    fs::create_dir_all(path.join("subdir2")).unwrap();
    fs::write(path.join("file1.txt"), "This is file 1").unwrap();
    fs::write(path.join("file2.log"), "This is file 2").unwrap();
    fs::write(path.join("subdir1/file3.txt"), "This is file 3").unwrap();
    fs::write(path.join("subdir2/file4.md"), "This is file 4").unwrap();
}

pub fn clear_dir(path: &Path) {
    println!("Clearing dir: {}", path.display());

    match fs::read_dir(path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            if path.is_ok() {
                let p = path.unwrap().path();
                if p.is_dir() {
                    fs::remove_dir_all(p).unwrap();
                } else {
                    fs::remove_file(p).unwrap();
                }
            }
        },
    }
}
