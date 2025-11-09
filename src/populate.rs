use std::path::Path;
use std::fs;
use rand::Rng;

pub fn populate_dir(path: &Path) {
    println!("Populating dir: {}", path.display());

    let num_files = rand::thread_rng().gen_range(10..=20);

    let prefixes = vec!["image", "document", "music", "video", "archive"];
    let extensions = vec!["txt", "log", "md", "jpg", "png", "mp3", "mp4", "zip"];

    for i in 0..num_files {
        let prefix = prefixes[rand::thread_rng().gen_range(0..prefixes.len())];
        let extension = extensions[rand::thread_rng().gen_range(0..extensions.len())];
        let filename = format!("{}_{}.{}", prefix, i, extension);
        let file_path = path.join(&filename);
        fs::write(&file_path, format!("This is file number {}", i)).unwrap();
    }

    fs::create_dir_all(path.join("subdir1")).unwrap();
    fs::create_dir_all(path.join("subdir2")).unwrap();
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
