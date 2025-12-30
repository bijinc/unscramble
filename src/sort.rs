use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;

use finalfusion::prelude::*;
use finalfusion::embeddings::Embeddings;

use stop_words;

use crate::cli::SortOptions;

const JACCARD_THRESHOLD: f32 = 0.2;
const FASTTEXT_THRESHOLD: f32 = 0.99;

// const EMBEDDINGS_PATH: &str = const_format::formatcp!("embeddings{}fasttext_embeddings.bin", MAIN_SEPARATOR);
// const EMBEDDINGS_PATH: &str = const_format::formatcp!("embeddings{}fasttext_full_embeddings.bin", MAIN_SEPARATOR);
// const EMBEDDINGS_PATH: &str = const_format::formatcp!("embeddings{}fasttext_domain_embeddings.bin", MAIN_SEPARATOR);
const EMBEDDINGS_PATH: &str = const_format::formatcp!("embeddings{}crawl-300d-2M.vec", MAIN_SEPARATOR);

pub fn sort_dir(path: &Path, options: &SortOptions) {
    println!("Sorting directory: {} {:?}", path.display(), options);

    if options.ext {
        sort_dir_by_extension(path, options);
    } else {
        // sort semantically by name
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

fn sort_dir_semantic(path: &Path, options: &SortOptions) {
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
    let clusters = cluster_similar_files(&file_features, &options);

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

fn cluster_similar_files(file_features: &[(PathBuf, Vec<String>)], options: &SortOptions) -> HashMap<String, Vec<PathBuf>> {
    let mut groups: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let mut assigned: Vec<bool> = vec![false; file_features.len()];

    // Preload FastText model for all comparisons
    let model = load_model();
    
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
                let similarity = if options.name {
                    calculate_feature_similarity(features_i, features_j, &model)
                } else {
                    jaccard_similarity(features_i, features_j)
                };
                
                let threshold = if options.name {
                    FASTTEXT_THRESHOLD
                } else {
                    JACCARD_THRESHOLD
                };

                println!("similarity: {}, threshold: {}", similarity, threshold);
                
                if similarity > threshold {
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

/// Average embeddings for a list of features
fn average_feature_embeddings(features: &[String], model: &Embeddings<VocabWrap, StorageWrap>) -> Option<Vec<f32>> {
    let mut sum_vec: Option<Vec<f32>> = None;
    let mut count = 0;
    
    for feature in features {
        if let Some(embedding) = model.embedding(feature) {
            if sum_vec.is_none() {
                sum_vec = Some(embedding.to_vec());
            } else {
                let sum = sum_vec.as_mut().unwrap();
                for (i, &val) in embedding.iter().enumerate() {
                    sum[i] += val;
                }
            }
            count += 1;
        }
    }
    
    sum_vec.map(|mut vec| {
        vec.iter_mut().for_each(|v| *v /= count as f32);
        vec
    })
}

/// Calculate cosine similarity between two vectors
fn cosine_similarity(vec_a: &[f32], vec_b: &[f32]) -> f32 {
    if vec_a.len() != vec_b.len() || vec_a.is_empty() {
        return 0.0;
    }

    let dot = dot_product(&vec_a, &vec_b).unwrap_or(0.0);
    let norm_a = dot_product(&vec_a, &vec_a).unwrap_or(0.0).sqrt();
    let norm_b = dot_product(&vec_b, &vec_b).unwrap_or(0.0).sqrt();

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

/// Loads the FastText embeddings model
fn load_model() -> Embeddings<VocabWrap, StorageWrap> {
    let embeddings_path: &Path = Path::new(EMBEDDINGS_PATH);

    let file = File::open(embeddings_path).expect("Failed to open embeddings file");
    let mut reader = BufReader::new(file);

    // Check file extension to determine format
    if embeddings_path.extension().and_then(|e| e.to_str()) == Some("vec") {
        // Load .vec format (text-based, returns SimpleVocab)
        let embeddings = Embeddings::read_text_dims(&mut reader)
            .expect("Failed to load .vec embeddings");
        return embeddings.into();
    } else {
        // Load .bin format (binary FastText with subword info)
        let embeddings = Embeddings::read_fasttext(&mut reader)
            .expect("Failed to load FastText embeddings");
        return embeddings.into();
    }
}
