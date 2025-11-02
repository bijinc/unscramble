use std::fs;

mod populate {

    pub fn populate_dir(path: String) {
        println!("Populating dir: {}", path);
        
        fs::create_dir_all(format!("{}/subdir1", path)).unwrap();
        fs::create_dir_all(format!("{}/subdir2", path)).unwrap();
        fs::write(format!("{}/file1.txt", path), "This is file 1").unwrap();
        fs::write(format!("{}/file2.log", path), "This is file 2").unwrap();
        fs::write(format!("{}/subdir1/file3.txt", path), "This is file 3").unwrap();
        fs::write(format!("{}/subdir2/file4.md", path), "This is file 4").unwrap();
    }

    pub fn clear_dir(path: String) {
        println!("Clearing dir: {}", path);

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