use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::cli::SortOptions;

const THRESHOLD: f32 = 0.2;

pub fn sort_dir(path: &Path, options: &SortOptions) {
    println!("Sorting directory: {} {:?}", path.display(), options);

    if options.ext {
        sort_dir_by_extension(path, options);
    } else {
        // default to semantic sort
        sort_dir_semantic(path, options);
    }
}

fn sort_dir_by_extension(path: &Path, options: &SortOptions) {
    let entries = std::fs::read_dir(path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();

        if file_type.is_file() {
            if let Some(ext) = entry.path().extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                let ext_dir = path.join(&ext_str);

                std::fs::create_dir_all(&ext_dir).unwrap();

                let new_path = ext_dir.join(entry.file_name());
                std::fs::rename(entry.path(), new_path).unwrap();
            }
        } else if file_type.is_dir() && options.recursive {
            sort_dir_by_extension(&entry.path(), options);
        }
    }
}

fn sort_dir_semantic(path: &Path, _options: &SortOptions) {
    // get all files
    let files: Vec<std::fs::DirEntry> = std::fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .collect();

    if files.is_empty() {
        println!("No files found in directory");
        return;
    }

    // extract features
    let file_features: Vec<(std::path::PathBuf, Vec<String>)> = files.iter()
        .map(|file| {
            let filename = file.file_name().to_string_lossy().to_string();
            let features = extract_filename_features(&filename);
            (file.path(), features)
        })
        .collect();

    // cluster files based on similar features
    let clusters = cluster_similar_files(&file_features);

    // move files using clusters
    for (cluster_name, file_paths) in clusters {
        if file_paths.len() > 1 {  // Only create folder if there are multiple files
            let group_dir = path.join(&cluster_name);
            std::fs::create_dir_all(&group_dir).unwrap();
            
            for file_path in file_paths {
                let new_path = group_dir.join(file_path.file_name().unwrap());
                std::fs::rename(&file_path, &new_path).unwrap();
            }
        }
    }
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

fn cluster_similar_files(
    file_features: &[(std::path::PathBuf, Vec<String>)]
) -> HashMap<String, Vec<std::path::PathBuf>> {
    let mut groups: HashMap<String, Vec<std::path::PathBuf>> = HashMap::new();
    let mut assigned: Vec<bool> = vec![false; file_features.len()];
    
    for (i, (path_i, features_i)) in file_features.iter().enumerate() {
        if assigned[i] {
            continue;
        }
        
        // Find the most common feature as group name
        let group_name = features_i
            .first()
            .map(|s| s.clone())
            .unwrap_or_else(|| "misc".to_string());
        
        let mut group_files = vec![path_i.clone()];
        assigned[i] = true;
        
        // Find similar files
        for (j, (path_j, features_j)) in file_features.iter().enumerate() {
            if i != j && !assigned[j] {
                let similarity = jaccard_similarity(features_i, features_j);
                if similarity > THRESHOLD {
                    group_files.push(path_j.clone());
                    assigned[j] = true;
                }
            }
        }
        groups.insert(group_name, group_files);
    }
    
    return groups;
}

/// Calculate Jaccard similarity between two feature sets
fn jaccard_similarity(features_a: &[String], features_b: &[String]) -> f32 {
    let set_a: HashSet<&String> = features_a.iter().collect();
    let set_b: HashSet<&String> = features_b.iter().collect();
    
    let intersection: HashSet<&&String> = set_a.intersection(&set_b).collect();
    let union: HashSet<&&String> = set_a.union(&set_b).collect();
    
    if union.is_empty() {
        return 0.0;
    }
    
    return intersection.len() as f32 / union.len() as f32;
}

/// Calculate cosine similarity between two strings
fn cosine_similarity(string_a: String, string_b: String) -> f32 {
    // convert strings to lower case
    let string_a_lower = string_a.to_lowercase();
    let string_b_lower = string_b.to_lowercase();

    // tokenize into words
    let mut string_a_tokens: Vec<String> = string_a_lower
        .split_whitespace()
        .map(str::to_string)
        .collect();
    let mut string_b_tokens: Vec<String> = string_b_lower
        .split_whitespace()
        .map(str::to_string)
        .collect();

    // remove stop words
    let stop_words: HashSet<String> = vec!["the" "a" "is"];
    string_a_tokens.retain(|token| !stop_words.contains(token));
    string_b_tokens.retain(|token| !stop_words.contains(token));

    // create embeddings for string_a and string_b





    0.0
}

/// Vector dot product of two vectors
fn dot_product(vec_a: &[f32], vec_b: &[f32]) -> Option<f32> {
    if (vec_a.len() != vec_b.len()) {
        return None;
    }

    let mut sum = 0.0
    for i in 0..vec_a.len() {
        sum += vec_a[i] * vec_b[i];
    }

    return Some(sum);
}