use std::path::Path;

use crate::cli::SortOptions;

pub fn sort_dir(path: &Path, options: &SortOptions) {
    println!("Sorting directory: {} {:?}", path.display(), options);

    // get all files and folders and store them temporarily

    for files in std::fs::read_dir(path).unwrap() {
        let file = files.unwrap();
        let file_path = file.path();

        if file_path.is_file() {
            if options.ext {
                if let Some(ext) = file_path.extension() {
                    // create dir for extension
                    let ext_dir = path.join(ext);
                    std::fs::create_dir_all(&ext_dir).unwrap();

                    // move file to new dir
                    let new_path = ext_dir.join(file_path.file_name().unwrap());
                    std::fs::rename(&file_path, &new_path).unwrap();
                    // dbg!("Moved {:?} to {:?}", file_path, new_path);
                }
            }
            // Additional sorting criteria can be implemented here
        } else if options.recursive && file_path.is_dir() {
            sort_dir(&file_path, options);
        }
    }
}

pub fn sort_dir_semantic(path: &Path, options: &SortOptions) {

    // get all files
    let files = std::fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .collect();

    if files.is_empty() {
        println!("No files found in directory");
        return;
    }

    // extract features
    let file_features = files.iter()
        .map(|file| {
            let filename = file.file_name().to_string_lossy().to_string();
            let features = extract_filename_features(&filename);
            (file.path(), features)
        })
        .collect();
}

fn extract_filename_features(filename: &str) -> Vec<String> {
    let stem = filename
        .rsplit_once('.')
        .map(|(name, _)| name)
        .unwrap_or(filename);

    let features: Vec<String> = stem
        .split(|c: char| c == '_' || c == '-' || c == ' ' || c.is_numeric())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .collect();

    return features;
}