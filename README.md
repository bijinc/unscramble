# unscramble
recursively organize files in a directory using semantics

### Features
* Organize files into dedicated folders using file extensions
* Organize files semantically using Jaccard indexing
* Organize files semantically using cosine similarity of FastText embedding vectors

Uses the `crawl-300d-2M` pre-trained model

### Optimizations
* Converted models to the finalfusion format to speed up loading
* Load models on initialization using finalfusion with memory-mapping (199s -> 2s)
* Used 'rayon' to parallelize certain stages of the semantic organization pipeline
