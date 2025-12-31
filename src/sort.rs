use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use finalfusion::prelude::*;
use finalfusion::embeddings::Embeddings;
use rayon::prelude::*;
use stop_words;

use crate::cli::{SortMethod, SortOptions};
use crate::state::State;

const JACCARD_THRESHOLD: f32 = 0.2;
const FASTTEXT_THRESHOLD: f32 = 0.5;

pub fn sort_dir(state: &State, path: &Path, options: &SortOptions) {
    println!("Sorting directory: {} {:?}", path.display(), options);

    if options.ext {
        sort_dir_by_extension(path, options);
    } else {
        if let Some(embeddings) = &state.embeddings {
            // sort semantically by name
            sort_dir_semantic(path, options, embeddings.as_ref());
        } else {
            eprintln!("Embeddings not loaded");
        }
    }
}

/// Sort files by their extension, recursively if specified
fn sort_dir_by_extension(path: &Path, options: &SortOptions) {
    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();

        if file_type.is_file() {
            if let Some(ext) = entry.path().extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                let ext_dir = path.join(&ext_str);

                fs::create_dir_all(&ext_dir).unwrap();

                let new_path = ext_dir.join(entry.file_name());
                fs::rename(entry.path(), new_path).unwrap();
            }
        } else if file_type.is_dir() && options.recursive {
            sort_dir_by_extension(&entry.path(), options);
        }
    }
}

fn sort_dir_semantic(path: &Path, options: &SortOptions, embeddings: &Embeddings<VocabWrap, StorageWrap>) {
    // get all files
    let files: Vec<fs::DirEntry> = fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .collect();

    if files.is_empty() {
        println!("No files found in directory");
        return;
    }

    // extract features
    println!("Extracting features...");
    let mut start = Instant::now();
    let file_features: Vec<(PathBuf, Vec<String>)> = files.par_iter()
        .map(|file| {
            let filename = file.file_name().to_string_lossy().to_string();
            let features = extract_filename_features(&filename);
            (file.path(), features)
        })
        .collect();
    let mut duration = start.elapsed();
    println!("Done! Extracted in {:.2?}", duration);

    // cluster files based on similar features
    let clusters = cluster_similar_files(&file_features, options, embeddings);

    // move files using clusters
    println!("Rearranging...");
    start = Instant::now();
    clusters.par_iter().for_each(|(cluster_name, file_paths)| {
        if file_paths.len() > 1 {  // Only create folder if there are multiple files
            let group_dir = path.join(cluster_name);
            fs::create_dir_all(&group_dir).unwrap();
            
            for file_path in file_paths {
                let new_path = group_dir.join(file_path.file_name().unwrap());
                fs::rename(file_path, &new_path).unwrap();
            }
        }
    });
    duration = start.elapsed();
    println!("Done! Took {:.2?}", duration);
}

fn extract_filename_features(filename: &str) -> Vec<String> {
    let stem = filename
        .rsplit_once('.')
        .map(|(name, _)| name)
        .unwrap_or(filename);
        
    let mut features: Vec<String> = stem
        .split(|c: char| c == '_' || c == '-' || c == ' ' || c.is_numeric())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .collect();
        
    // Get the NLTK English stop words list
    let stopwords_vec: Vec<String> = stop_words::get(stop_words::LANGUAGE::English)
        .iter()
        .map(|s| s.to_string())
        .collect();
    let stop_words: HashSet<_> = stopwords_vec.iter().collect();

    // Remove stop words
    features.retain(|token| !stop_words.contains(token));

    return features;
}

fn cluster_similar_files(file_features: &[(PathBuf, Vec<String>)], options: &SortOptions, embeddings: &Embeddings<VocabWrap, StorageWrap>) -> HashMap<String, Vec<PathBuf>> {
    let mut groups: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let mut assigned: Vec<bool> = vec![false; file_features.len()];
    
    for (i, (path_i, features_i)) in file_features.iter().enumerate() {
        if assigned[i] {
            continue;
        }
        
        let mut group_files = vec![path_i.clone()];
        assigned[i] = true;
        
        // Find similar files
        for (j, (path_j, features_j)) in file_features.iter().enumerate() {
            if i != j && !assigned[j] {
                let similarity = if options.method == SortMethod::Jac {
                    jaccard_similarity(features_i, features_j)
                } else {
                    calculate_feature_similarity(features_i, features_j, embeddings)
                };

                let threshold = if options.method == SortMethod::Jac {
                    JACCARD_THRESHOLD
                } else {
                    FASTTEXT_THRESHOLD
                };

                if similarity > threshold {
                    group_files.push(path_j.clone());
                    assigned[j] = true;
                }
            }
        }
        
        // Find the best group name after clustering
        let group_name = find_best_group_name(&group_files, file_features);
        groups.insert(group_name, group_files);
    }

    return groups;
}

/// Find the most common feature across all files in a cluster
fn find_best_group_name(group_files: &[PathBuf], file_features: &[(PathBuf, Vec<String>)]) -> String {
    let mut feature_counts: HashMap<&String, usize> = HashMap::new();
    
    for path in group_files {
        if let Some((_, features)) = file_features.iter().find(|(p, _)| p == path) {
            for feature in features {
                *feature_counts.entry(feature).or_insert(0) += 1;
            }
        }
    }
    
    feature_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(feature, _)| feature.clone())
        .unwrap_or_else(|| "misc".to_string())
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

/// Calculate similarity between two feature lists using maximum pairwise similarity
fn calculate_feature_similarity(features_a: &[String], features_b: &[String], model: &Embeddings<VocabWrap, StorageWrap>) -> f32 {
    if features_a.is_empty() || features_b.is_empty() {
        return 0.0;
    }
    
    // Get embeddings for all features
    let embeddings_a: Vec<Vec<f32>> = features_a.iter()
        .filter_map(|f| model.embedding(f).map(|e| e.to_vec()))
        .collect();
    
    let embeddings_b: Vec<Vec<f32>> = features_b.iter()
        .filter_map(|f| model.embedding(f).map(|e| e.to_vec()))
        .collect();
    
    if embeddings_a.is_empty() || embeddings_b.is_empty() {
        return 0.0;
    }
    
    // Find maximum similarity across all pairs
    let mut max_sim = 0.0;
    for emb_a in &embeddings_a {
        for emb_b in &embeddings_b {
            let sim = cosine_similarity(emb_a, emb_b);
            if sim > max_sim {
                max_sim = sim;
            }
        }
    }
    
    return max_sim;
}

/// Calculate cosine similarity between two vectors
fn cosine_similarity(vec_a: &[f32], vec_b: &[f32]) -> f32 {
    if vec_a.len() != vec_b.len() || vec_a.is_empty() {
        return 0.0;
    }

    let dot = dot_product(vec_a, vec_b).unwrap_or(0.0);
    let norm_a = dot_product(vec_a, vec_a).unwrap_or(0.0).sqrt();
    let norm_b = dot_product(vec_b, vec_b).unwrap_or(0.0).sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    return dot / (norm_a * norm_b);
}

/// Vector dot product of two vectors
fn dot_product(vec_a: &[f32], vec_b: &[f32]) -> Option<f32> {
    if vec_a.len() != vec_b.len() {
        return None;
    }

    let sum = vec_a.iter()
        .zip(vec_b.iter())
        .map(|(a, b)| a * b)
        .sum();

    return Some(sum);
}
