//! Vector Index module for ARCADIA
//! 
//! Provides efficient storage, retrieval, and manipulation of high-dimensional vectors
//! for AI-driven game content and character data

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorIndexConfig {
    pub url: String,
    pub api_key: String,
}

impl VectorIndexConfig {
    pub fn new(url: String, api_key: String) -> Self {
        VectorIndexConfig { url, api_key }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.url.is_empty() {
            return Err("URL cannot be empty".to_string());
        }
        if self.api_key.is_empty() {
            return Err("API key cannot be empty".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Vector {
    pub id: String,
    pub data: Vec<f32>,
    pub metadata: HashMap<String, String>,
}

impl Vector {
    pub fn new(id: String, data: Vec<f32>) -> Self {
        Vector {
            id,
            data,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn dimension(&self) -> usize {
        self.data.len()
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        if magnitude > 0.0 {
            for val in &mut self.data {
                *val /= magnitude;
            }
        }
    }

    pub fn magnitude(&self) -> f32 {
        self.data.iter().map(|x| x * x).sum::<f32>().sqrt()
    }

    pub fn dot_product(&self, other: &Vector) -> Result<f32, String> {
        if self.data.len() != other.data.len() {
            return Err("Vector dimensions must match".to_string());
        }
        Ok(self.data.iter().zip(&other.data).map(|(a, b)| a * b).sum())
    }

    pub fn cosine_similarity(&self, other: &Vector) -> Result<f32, String> {
        let dot = self.dot_product(other)?;
        let mag_product = self.magnitude() * other.magnitude();
        if mag_product == 0.0 {
            return Ok(0.0);
        }
        Ok(dot / mag_product)
    }
}

#[derive(Debug)]
pub struct VectorIndex {
    config: VectorIndexConfig,
    vectors: HashMap<String, Vector>,
    dimension: Option<usize>,
}

impl VectorIndex {
    pub fn new(config: VectorIndexConfig) -> Self {
        VectorIndex {
            config,
            vectors: HashMap::new(),
            dimension: None,
        }
    }

    pub fn insert(&mut self, vector: Vector) -> Result<(), String> {
        // Validate dimension consistency
        if let Some(dim) = self.dimension {
            if vector.dimension() != dim {
                return Err(format!(
                    "Vector dimension {} does not match index dimension {}",
                    vector.dimension(),
                    dim
                ));
            }
        } else {
            self.dimension = Some(vector.dimension());
        }

        self.vectors.insert(vector.id.clone(), vector);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&Vector> {
        self.vectors.get(id)
    }

    pub fn remove(&mut self, id: &str) -> Option<Vector> {
        self.vectors.remove(id)
    }

    pub fn search(&self, query: &Vector, top_k: usize) -> Result<Vec<(String, f32)>, String> {
        let mut similarities: Vec<(String, f32)> = self
            .vectors
            .iter()
            .filter_map(|(id, vec)| {
                query
                    .cosine_similarity(vec)
                    .ok()
                    .map(|sim| (id.clone(), sim))
            })
            .collect();

        // Sort by similarity (descending)
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Return top_k results
        Ok(similarities.into_iter().take(top_k).collect())
    }

    pub fn size(&self) -> usize {
        self.vectors.len()
    }

    pub fn dimension(&self) -> Option<usize> {
        self.dimension
    }

    pub fn clear(&mut self) {
        self.vectors.clear();
        self.dimension = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_vector(id: &str, data: Vec<f32>) -> Vector {
        Vector::new(id.to_string(), data)
    }

    #[test]
    fn test_vector_creation() {
        let vec = create_test_vector("vec1", vec![1.0, 2.0, 3.0]);
        assert_eq!(vec.id, "vec1");
        assert_eq!(vec.dimension(), 3);
    }

    #[test]
    fn test_vector_magnitude() {
        let vec = create_test_vector("vec1", vec![3.0, 4.0]);
        assert_eq!(vec.magnitude(), 5.0);
    }

    #[test]
    fn test_vector_normalize() {
        let mut vec = create_test_vector("vec1", vec![3.0, 4.0]);
        vec.normalize();
        assert!((vec.magnitude() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_dot_product() {
        let vec1 = create_test_vector("vec1", vec![1.0, 2.0, 3.0]);
        let vec2 = create_test_vector("vec2", vec![4.0, 5.0, 6.0]);
        let result = vec1.dot_product(&vec2).unwrap();
        assert_eq!(result, 32.0); // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_cosine_similarity() {
        let vec1 = create_test_vector("vec1", vec![1.0, 0.0]);
        let vec2 = create_test_vector("vec2", vec![1.0, 0.0]);
        let sim = vec1.cosine_similarity(&vec2).unwrap();
        assert_eq!(sim, 1.0);
    }

    #[test]
    fn test_vector_index_creation() {
        let config = VectorIndexConfig::new("http://localhost".to_string(), "key123".to_string());
        let index = VectorIndex::new(config);
        assert_eq!(index.size(), 0);
        assert!(index.dimension().is_none());
    }

    #[test]
    fn test_vector_index_insert() {
        let config = VectorIndexConfig::new("http://localhost".to_string(), "key123".to_string());
        let mut index = VectorIndex::new(config);
        
        let vec = create_test_vector("vec1", vec![1.0, 2.0, 3.0]);
        assert!(index.insert(vec).is_ok());
        assert_eq!(index.size(), 1);
        assert_eq!(index.dimension(), Some(3));
    }

    #[test]
    fn test_vector_index_dimension_validation() {
        let config = VectorIndexConfig::new("http://localhost".to_string(), "key123".to_string());
        let mut index = VectorIndex::new(config);
        
        let vec1 = create_test_vector("vec1", vec![1.0, 2.0, 3.0]);
        let vec2 = create_test_vector("vec2", vec![1.0, 2.0]);
        
        index.insert(vec1).unwrap();
        let result = index.insert(vec2);
        assert!(result.is_err());
    }

    #[test]
    fn test_vector_index_search() {
        let config = VectorIndexConfig::new("http://localhost".to_string(), "key123".to_string());
        let mut index = VectorIndex::new(config);
        
        index.insert(create_test_vector("vec1", vec![1.0, 0.0, 0.0])).unwrap();
        index.insert(create_test_vector("vec2", vec![0.0, 1.0, 0.0])).unwrap();
        index.insert(create_test_vector("vec3", vec![1.0, 0.0, 0.0])).unwrap();
        
        let query = create_test_vector("query", vec![1.0, 0.0, 0.0]);
        let results = index.search(&query, 2).unwrap();
        
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].1, 1.0); // Perfect match
    }

    #[test]
    fn test_vector_index_get_remove() {
        let config = VectorIndexConfig::new("http://localhost".to_string(), "key123".to_string());
        let mut index = VectorIndex::new(config);
        
        let vec = create_test_vector("vec1", vec![1.0, 2.0, 3.0]);
        index.insert(vec).unwrap();
        
        assert!(index.get("vec1").is_some());
        assert!(index.remove("vec1").is_some());
        assert!(index.get("vec1").is_none());
    }
}
