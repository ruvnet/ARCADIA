// VIVIAN Framework - Vector Index Management Module
// Handles vector embeddings, similarity search, and index operations

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Vector dimension type
pub type VectorDimension = usize;

/// Vector embedding representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorEmbedding {
    pub id: String,
    pub vector: Vec<f32>,
    pub metadata: HashMap<String, String>,
    pub timestamp: i64,
}

/// Vector index configuration
#[derive(Debug, Clone)]
pub struct VectorIndexConfig {
    pub dimension: VectorDimension,
    pub metric: SimilarityMetric,
    pub index_type: IndexType,
    pub capacity: usize,
    pub sharding_enabled: bool,
    pub shard_count: usize,
}

/// Similarity metrics for vector comparison
#[derive(Debug, Clone, Copy)]
pub enum SimilarityMetric {
    Cosine,
    Euclidean,
    DotProduct,
    Manhattan,
}

/// Vector index types
#[derive(Debug, Clone, Copy)]
pub enum IndexType {
    /// Flat index for exact search
    Flat,
    /// Hierarchical Navigable Small World graphs
    HNSW,
    /// Inverted File Index
    IVF,
    /// Product Quantization
    PQ,
}

/// Search result structure
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub embedding: VectorEmbedding,
}

/// Vector Index Manager
///
/// Manages vector embeddings, performs similarity searches,
/// and handles index operations for the VIVIAN framework.
pub struct VectorIndexManager {
    config: VectorIndexConfig,
    indices: HashMap<String, VectorIndex>,
    global_index: Option<VectorIndex>,
}

impl VectorIndexManager {
    /// Create a new vector index manager
    pub async fn new(config: VectorIndexConfig) -> Result<Self, VectorIndexError> {
        Ok(Self {
            config,
            indices: HashMap::new(),
            global_index: None,
        })
    }

    /// Initialize the vector index manager
    pub async fn initialize(&mut self) -> Result<(), VectorIndexError> {
        // Create global index
        self.global_index = Some(VectorIndex::new(
            "global".to_string(),
            self.config.clone(),
        )?);

        Ok(())
    }

    /// Create a named vector index
    pub async fn create_index(&mut self, name: String) -> Result<(), VectorIndexError> {
        let index = VectorIndex::new(name.clone(), self.config.clone())?;
        self.indices.insert(name, index);
        Ok(())
    }

    /// Add vector embedding to an index
    pub async fn add_embedding(
        &mut self,
        index_name: &str,
        embedding: VectorEmbedding,
    ) -> Result<(), VectorIndexError> {
        if let Some(index) = self.indices.get_mut(index_name) {
            index.add(embedding)?;
            Ok(())
        } else {
            Err(VectorIndexError::IndexNotFound(index_name.to_string()))
        }
    }

    /// Search for similar vectors
    pub async fn search(
        &self,
        index_name: &str,
        query: &[f32],
        k: usize,
    ) -> Result<Vec<SearchResult>, VectorIndexError> {
        if let Some(index) = self.indices.get(index_name) {
            index.search(query, k)
        } else {
            Err(VectorIndexError::IndexNotFound(index_name.to_string()))
        }
    }

    /// Batch add embeddings
    pub async fn batch_add(
        &mut self,
        index_name: &str,
        embeddings: Vec<VectorEmbedding>,
    ) -> Result<(), VectorIndexError> {
        if let Some(index) = self.indices.get_mut(index_name) {
            for embedding in embeddings {
                index.add(embedding)?;
            }
            Ok(())
        } else {
            Err(VectorIndexError::IndexNotFound(index_name.to_string()))
        }
    }

    /// Remove an embedding by ID
    pub async fn remove_embedding(
        &mut self,
        index_name: &str,
        id: &str,
    ) -> Result<(), VectorIndexError> {
        if let Some(index) = self.indices.get_mut(index_name) {
            index.remove(id)
        } else {
            Err(VectorIndexError::IndexNotFound(index_name.to_string()))
        }
    }

    /// Get index statistics
    pub async fn get_stats(&self, index_name: &str) -> Result<IndexStats, VectorIndexError> {
        if let Some(index) = self.indices.get(index_name) {
            Ok(index.stats())
        } else {
            Err(VectorIndexError::IndexNotFound(index_name.to_string()))
        }
    }

    /// Shutdown the vector index manager
    pub async fn shutdown(&mut self) -> Result<(), VectorIndexError> {
        self.indices.clear();
        self.global_index = None;
        Ok(())
    }
}

/// Individual vector index
struct VectorIndex {
    name: String,
    config: VectorIndexConfig,
    embeddings: HashMap<String, VectorEmbedding>,
    size: usize,
}

impl VectorIndex {
    fn new(name: String, config: VectorIndexConfig) -> Result<Self, VectorIndexError> {
        Ok(Self {
            name,
            config,
            embeddings: HashMap::new(),
            size: 0,
        })
    }

    fn add(&mut self, embedding: VectorEmbedding) -> Result<(), VectorIndexError> {
        if embedding.vector.len() != self.config.dimension {
            return Err(VectorIndexError::DimensionMismatch {
                expected: self.config.dimension,
                got: embedding.vector.len(),
            });
        }

        self.embeddings.insert(embedding.id.clone(), embedding);
        self.size += 1;
        Ok(())
    }

    fn search(&self, query: &[f32], k: usize) -> Result<Vec<SearchResult>, VectorIndexError> {
        if query.len() != self.config.dimension {
            return Err(VectorIndexError::DimensionMismatch {
                expected: self.config.dimension,
                got: query.len(),
            });
        }

        let mut results: Vec<SearchResult> = self
            .embeddings
            .values()
            .map(|embedding| {
                let score = self.compute_similarity(query, &embedding.vector);
                SearchResult {
                    id: embedding.id.clone(),
                    score,
                    embedding: embedding.clone(),
                }
            })
            .collect();

        // Sort by score (descending)
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.truncate(k);

        Ok(results)
    }

    fn remove(&mut self, id: &str) -> Result<(), VectorIndexError> {
        if self.embeddings.remove(id).is_some() {
            self.size -= 1;
            Ok(())
        } else {
            Err(VectorIndexError::EmbeddingNotFound(id.to_string()))
        }
    }

    fn compute_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        match self.config.metric {
            SimilarityMetric::Cosine => cosine_similarity(a, b),
            SimilarityMetric::Euclidean => euclidean_distance(a, b),
            SimilarityMetric::DotProduct => dot_product(a, b),
            SimilarityMetric::Manhattan => manhattan_distance(a, b),
        }
    }

    fn stats(&self) -> IndexStats {
        IndexStats {
            name: self.name.clone(),
            size: self.size,
            dimension: self.config.dimension,
            metric: format!("{:?}", self.config.metric),
            index_type: format!("{:?}", self.config.index_type),
        }
    }
}

/// Index statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    pub name: String,
    pub size: usize,
    pub dimension: usize,
    pub metric: String,
    pub index_type: String,
}

// Similarity computation functions
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot = dot_product(a, b);
    let norm_a = (a.iter().map(|x| x * x).sum::<f32>()).sqrt();
    let norm_b = (b.iter().map(|x| x * x).sum::<f32>()).sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}

fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    let sum: f32 = a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum();
    -sum.sqrt() // Negative to maintain descending sort order
}

fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

fn manhattan_distance(a: &[f32], b: &[f32]) -> f32 {
    let sum: f32 = a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).abs())
        .sum();
    -sum // Negative to maintain descending sort order
}

/// Vector Index Error Types
#[derive(Debug, thiserror::Error)]
pub enum VectorIndexError {
    #[error("Index not found: {0}")]
    IndexNotFound(String),

    #[error("Embedding not found: {0}")]
    EmbeddingNotFound(String),

    #[error("Dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: usize, got: usize },

    #[error("Index creation failed: {0}")]
    CreationFailed(String),

    #[error("Search failed: {0}")]
    SearchFailed(String),
}
