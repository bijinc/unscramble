use std::fs::File;
use std::io::BufReader;
use std::path::{Path, MAIN_SEPARATOR};
use std::sync::Arc;

use finalfusion::embeddings::Embeddings;
use finalfusion::prelude::*;

const EMBEDDINGS_PATH: &str = const_format::formatcp!("embeddings{}crawl-300d-2M.vec", MAIN_SEPARATOR);

pub struct State {
    pub embeddings: Option<Arc<Embeddings<VocabWrap, StorageWrap>>>,
}

impl State {
    pub fn new() -> Self {
        Self { embeddings: None }
    }
    
    pub fn load_embeddings(&mut self) {
        if self.embeddings.is_none() {
            println!("Loading embeddings...");
            self.embeddings = Some(Arc::new(Self::load_model()));
        }
        println!("Done!")
    }

    fn load_model() -> Embeddings<VocabWrap, StorageWrap> {
        let embeddings_path: &Path = Path::new(EMBEDDINGS_PATH);

        let file = File::open(embeddings_path).expect("Failed to open embeddings file");
        let mut reader = BufReader::new(file);

        // Check file extension to determine format
        if embeddings_path.extension().and_then(|e| e.to_str()) == Some("vec") {
            // Load .vec format
            let embeddings = Embeddings::read_text_dims(&mut reader)
                .expect("Failed to load .vec embeddings");
            return embeddings.into();
        } else {
            // Load .bin format
            let embeddings = Embeddings::read_fasttext(&mut reader)
                .expect("Failed to load FastText embeddings");
            return embeddings.into();
        }
    }
}