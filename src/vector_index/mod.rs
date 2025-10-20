//! Vector Index module for ARCADIA
//! Provides vector storage and similarity search using Qdrant and OpenAI embeddings

use async_openai::{
    types::{CreateEmbeddingRequestArgs, EmbeddingInput},
    Client as OpenAIClient,
};
use qdrant_client::{
    client::QdrantClient,
    qdrant::{
        vectors_config::Config, CreateCollection, Distance, PointStruct, SearchPoints,
        VectorParams, VectorsConfig, Value, with_payload_selector::SelectorOptions,
        WithPayloadSelector,
    },
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum VectorIndexError {
    #[error("OpenAI API error: {0}")]
    OpenAIError(String),
    
    #[error("Qdrant error: {0}")]
    QdrantError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Vector dimension mismatch")]
    DimensionMismatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorIndexConfig {
    pub url: String,
    pub api_key: String,
    pub qdrant_url: Option<String>,
    pub collection_name: String,
    pub embedding_model: String,
    pub vector_dimension: usize,
}

impl Default for VectorIndexConfig {
    fn default() -> Self {
        Self {
            url: "https://api.openai.com".to_string(),
            api_key: String::new(),
            qdrant_url: Some("http://localhost:6334".to_string()),
            collection_name: "arcadia_vectors".to_string(),
            embedding_model: "text-embedding-3-small".to_string(),
            vector_dimension: 1536,
        }
    }
}

pub struct VectorIndex {
    config: VectorIndexConfig,
    openai_client: OpenAIClient<async_openai::config::OpenAIConfig>,
    qdrant_client: Option<QdrantClient>,
    #[cfg(feature = "vector-index")]
    embedding_cache: Option<Arc<crate::cache::EmbeddingCache>>,
}

use std::sync::Arc;

impl VectorIndex {
    /// Create a new VectorIndex instance with caching enabled
    pub async fn new(config: VectorIndexConfig) -> Result<Self, VectorIndexError> {
        let openai_config = async_openai::config::OpenAIConfig::new()
            .with_api_key(&config.api_key);

        let openai_client = OpenAIClient::with_config(openai_config);

        let qdrant_client = if let Some(ref qdrant_url) = config.qdrant_url {
            Some(
                QdrantClient::from_url(qdrant_url)
                    .build()
                    .map_err(|e| VectorIndexError::QdrantError(e.to_string()))?
            )
        } else {
            None
        };

        #[cfg(feature = "vector-index")]
        let embedding_cache = Some(Arc::new(crate::cache::create_embedding_cache()));

        let index = Self {
            config,
            openai_client,
            qdrant_client,
            #[cfg(feature = "vector-index")]
            embedding_cache,
        };

        // Initialize collection if Qdrant is available
        if index.qdrant_client.is_some() {
            index.init_collection().await?;
        }

        Ok(index)
    }
    
    /// Initialize the Qdrant collection
    async fn init_collection(&self) -> Result<(), VectorIndexError> {
        if let Some(ref client) = self.qdrant_client {
            let collections = client
                .list_collections()
                .await
                .map_err(|e| VectorIndexError::QdrantError(e.to_string()))?;
            
            let collection_exists = collections
                .collections
                .iter()
                .any(|c| c.name == self.config.collection_name);
            
            if !collection_exists {
                client
                    .create_collection(&CreateCollection {
                        collection_name: self.config.collection_name.clone(),
                        vectors_config: Some(VectorsConfig {
                            config: Some(Config::Params(VectorParams {
                                size: self.config.vector_dimension as u64,
                                distance: Distance::Cosine.into(),
                                ..Default::default()
                            })),
                        }),
                        ..Default::default()
                    })
                    .await
                    .map_err(|e| VectorIndexError::QdrantError(e.to_string()))?;
            }
        }
        
        Ok(())
    }
    
    /// Generate embeddings for text using OpenAI with caching and metrics
    pub async fn embed_text(&self, text: &str) -> Result<Vec<f32>, VectorIndexError> {
        #[cfg(feature = "vector-index")]
        {
            let start = std::time::Instant::now();

            // Check cache first
            if let Some(ref cache) = self.embedding_cache {
                if let Some(cached_embedding) = cache.get(&text.to_string()).await {
                    crate::metrics::cache_metrics::record_hit("embedding");
                    crate::metrics::vector_metrics::record_embedding(start.elapsed());
                    return Ok(cached_embedding);
                }
                crate::metrics::cache_metrics::record_miss("embedding");
            }

            // Generate embedding via API
            let request = CreateEmbeddingRequestArgs::default()
                .model(&self.config.embedding_model)
                .input(EmbeddingInput::String(text.to_string()))
                .build()
                .map_err(|e| VectorIndexError::OpenAIError(e.to_string()))?;

            let response = self
                .openai_client
                .embeddings()
                .create(request)
                .await
                .map_err(|e| {
                    crate::metrics::vector_metrics::record_error("embed_text");
                    VectorIndexError::OpenAIError(e.to_string())
                })?;

            let embedding = response.data[0].embedding.clone();

            // Cache the result
            if let Some(ref cache) = self.embedding_cache {
                cache.insert(text.to_string(), embedding.clone()).await;
            }

            crate::metrics::vector_metrics::record_embedding(start.elapsed());
            Ok(embedding)
        }

        #[cfg(not(feature = "vector-index"))]
        {
            let request = CreateEmbeddingRequestArgs::default()
                .model(&self.config.embedding_model)
                .input(EmbeddingInput::String(text.to_string()))
                .build()
                .map_err(|e| VectorIndexError::OpenAIError(e.to_string()))?;

            let response = self
                .openai_client
                .embeddings()
                .create(request)
                .await
                .map_err(|e| VectorIndexError::OpenAIError(e.to_string()))?;

            Ok(response.data[0].embedding.clone())
        }
    }
    
    /// Store a vector with metadata
    pub async fn store(
        &self,
        id: Option<String>,
        text: &str,
        metadata: HashMap<String, String>,
    ) -> Result<String, VectorIndexError> {
        let vector = self.embed_text(text).await?;
        let point_id = id.unwrap_or_else(|| Uuid::new_v4().to_string());
        
        if let Some(ref client) = self.qdrant_client {
            let mut payload: HashMap<String, Value> = metadata
                .into_iter()
                .map(|(k, v)| (k, Value::from(v)))
                .collect();
            
            payload.insert("text".to_string(), Value::from(text.to_string()));
            
            let point = PointStruct::new(
                point_id.clone(),
                vector,
                payload,
            );
            
            client
                .upsert_points_blocking(&self.config.collection_name, None, vec![point], None)
                .await
                .map_err(|e| VectorIndexError::QdrantError(e.to_string()))?;
        }
        
        Ok(point_id)
    }
    
    /// Search for similar vectors
    pub async fn search(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>, VectorIndexError> {
        let query_vector = self.embed_text(query).await?;
        
        if let Some(ref client) = self.qdrant_client {
            let search_result = client
                .search_points(&SearchPoints {
                    collection_name: self.config.collection_name.clone(),
                    vector: query_vector,
                    limit: limit as u64,
                    with_payload: Some(WithPayloadSelector {
                        selector_options: Some(SelectorOptions::Enable(true)),
                    }),
                    ..Default::default()
                })
                .await
                .map_err(|e| VectorIndexError::QdrantError(e.to_string()))?;
            
            let results = search_result
                .result
                .into_iter()
                .map(|point| {
                    let text = point
                        .payload
                        .get("text")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    
                    let metadata: HashMap<String, String> = point
                        .payload
                        .into_iter()
                        .filter(|(k, _)| k != "text")
                        .filter_map(|(k, v)| {
                            v.as_str().map(|s| (k, s.to_string()))
                        })
                        .collect();
                    
                    SearchResult {
                        id: point.id.unwrap().to_string(),
                        score: point.score,
                        text,
                        metadata,
                    }
                })
                .collect();
            
            Ok(results)
        } else {
            Ok(vec![])
        }
    }
    
    /// Delete a vector by ID
    pub async fn delete(&self, id: &str) -> Result<(), VectorIndexError> {
        if let Some(ref client) = self.qdrant_client {
            client
                .delete_points(
                    &self.config.collection_name,
                    None,
                    &[id.into()],
                    None,
                )
                .await
                .map_err(|e| VectorIndexError::QdrantError(e.to_string()))?;
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub text: String,
    pub metadata: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_embed_text() {
        // This test requires an actual API key
        // Skip in CI/CD environments
        if std::env::var("OPENAI_API_KEY").is_err() {
            return;
        }
        
        let config = VectorIndexConfig {
            api_key: std::env::var("OPENAI_API_KEY").unwrap(),
            qdrant_url: None,
            ..Default::default()
        };
        
        let index = VectorIndex::new(config).await.unwrap();
        let embedding = index.embed_text("Hello, world!").await.unwrap();
        
        assert_eq!(embedding.len(), 1536);
    }
}
