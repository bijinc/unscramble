use std::path::Path;

use crate::cli::SortOptions;

pub fn sort_dir(path: &Path, options: &SortOptions) {
    println!("Sorting directory: {}", path.display());
    println!("Options: {:?}", options);

    for files in std::fs::read_dir(path).unwrap() {
        let file = files.unwrap();
        let file_path = file.path();

        if file_path.is_file() {
            if options.ext {
                if let Some(ext) = file_path.extension() {
                    let ext_dir = path.join(ext);
                    std::fs::create_dir_all(&ext_dir).unwrap();
                    let new_path = ext_dir.join(file_path.file_name().unwrap());
                    std::fs::rename(&file_path, &new_path).unwrap();
                    println!("Moved {:?} to {:?}", file_path, new_path);
                }
            }
            // Additional sorting criteria can be implemented here
        } else if options.recursive && file_path.is_dir() {
            sort_dir(&file_path, options);
        }
    }
}