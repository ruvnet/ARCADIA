//! WASM Bindings for AgentDB
//!
//! Provides JavaScript-compatible bindings for running AgentDB in browser
//! and Node.js environments.

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// WASM-compatible AgentDB configuration
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmAgentDbConfig {
    db_name: String,
    vector_dim: usize,
    max_memory_mb: usize,
    replay_buffer_size: usize,
}

#[wasm_bindgen]
impl WasmAgentDbConfig {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            db_name: "arcadia_agents".to_string(),
            vector_dim: 1536,
            max_memory_mb: 512,
            replay_buffer_size: 10000,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn db_name(&self) -> String {
        self.db_name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_db_name(&mut self, name: String) {
        self.db_name = name;
    }

    #[wasm_bindgen(getter)]
    pub fn vector_dim(&self) -> usize {
        self.vector_dim
    }

    #[wasm_bindgen(setter)]
    pub fn set_vector_dim(&mut self, dim: usize) {
        self.vector_dim = dim;
    }
}

/// WASM AgentDB Manager
#[wasm_bindgen]
pub struct WasmAgentDb {
    config: WasmAgentDbConfig,
    experiences: Vec<JsValue>,
    initialized: bool,
}

#[wasm_bindgen]
impl WasmAgentDb {
    /// Create a new WASM AgentDB instance
    #[wasm_bindgen(constructor)]
    pub fn new(config: WasmAgentDbConfig) -> Result<WasmAgentDb, JsValue> {
        console_error_panic_hook::set_once();

        Ok(Self {
            config,
            experiences: Vec::new(),
            initialized: false,
        })
    }

    /// Initialize the database
    #[wasm_bindgen]
    pub async fn initialize(&mut self) -> Result<(), JsValue> {
        // Initialize IndexedDB or localStorage
        self.init_storage().await?;
        self.initialized = true;

        web_sys::console::log_1(&format!(
            "AgentDB initialized: {}",
            self.config.db_name
        ).into());

        Ok(())
    }

    /// Store an experience
    #[wasm_bindgen]
    pub async fn store_experience(
        &mut self,
        agent_id: String,
        experience_json: JsValue,
    ) -> Result<(), JsValue> {
        if !self.initialized {
            return Err(JsValue::from_str("Database not initialized"));
        }

        // Store in memory
        self.experiences.push(experience_json.clone());

        // Store in IndexedDB
        self.store_in_indexed_db(&agent_id, experience_json).await?;

        Ok(())
    }

    /// Query similar experiences
    #[wasm_bindgen]
    pub async fn query_similar(
        &self,
        query_vector: Vec<f32>,
        limit: usize,
    ) -> Result<JsValue, JsValue> {
        if !self.initialized {
            return Err(JsValue::from_str("Database not initialized"));
        }

        // Perform vector similarity search
        let results = self.vector_similarity_search(&query_vector, limit)?;

        // Convert to JS array
        let js_array = js_sys::Array::new();
        for result in results {
            js_array.push(&result);
        }

        Ok(js_array.into())
    }

    /// Get all experiences for an agent
    #[wasm_bindgen]
    pub async fn get_agent_experiences(
        &self,
        agent_id: String,
    ) -> Result<JsValue, JsValue> {
        if !self.initialized {
            return Err(JsValue::from_str("Database not initialized"));
        }

        self.retrieve_from_indexed_db(&agent_id).await
    }

    /// Get statistics
    #[wasm_bindgen]
    pub fn get_stats(&self) -> JsValue {
        let stats = serde_wasm_bindgen::to_value(&serde_json::json!({
            "initialized": self.initialized,
            "total_experiences": self.experiences.len(),
            "db_name": self.config.db_name,
            "vector_dim": self.config.vector_dim,
        })).unwrap();

        stats
    }

    /// Clear all data
    #[wasm_bindgen]
    pub async fn clear(&mut self) -> Result<(), JsValue> {
        self.experiences.clear();
        self.clear_indexed_db().await?;
        Ok(())
    }
}

// Private methods
impl WasmAgentDb {
    /// Initialize storage (IndexedDB or localStorage)
    async fn init_storage(&self) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("No window object")?;

        // Check for IndexedDB support
        if js_sys::Reflect::has(&window, &"indexedDB".into())? {
            self.init_indexed_db().await?;
        } else {
            // Fallback to localStorage
            web_sys::console::warn_1(&"IndexedDB not available, using localStorage".into());
        }

        Ok(())
    }

    /// Initialize IndexedDB
    async fn init_indexed_db(&self) -> Result<(), JsValue> {
        // IndexedDB initialization will be handled by rexie
        web_sys::console::log_1(&"IndexedDB initialized".into());
        Ok(())
    }

    /// Store in IndexedDB
    async fn store_in_indexed_db(
        &self,
        _agent_id: &str,
        _experience: JsValue,
    ) -> Result<(), JsValue> {
        // TODO: Implement actual IndexedDB storage using rexie
        Ok(())
    }

    /// Retrieve from IndexedDB
    async fn retrieve_from_indexed_db(&self, _agent_id: &str) -> Result<JsValue, JsValue> {
        // TODO: Implement actual IndexedDB retrieval using rexie
        Ok(js_sys::Array::new().into())
    }

    /// Clear IndexedDB
    async fn clear_indexed_db(&self) -> Result<(), JsValue> {
        // TODO: Implement actual IndexedDB clearing
        Ok(())
    }

    /// Vector similarity search using cosine similarity
    fn vector_similarity_search(
        &self,
        query: &[f32],
        limit: usize,
    ) -> Result<Vec<JsValue>, JsValue> {
        let mut results = Vec::new();
        let mut scores = Vec::new();

        // Calculate similarity scores
        for exp in &self.experiences {
            // Extract vector from experience
            if let Ok(exp_obj) = serde_wasm_bindgen::from_value::<serde_json::Value>(exp.clone()) {
                if let Some(vector) = exp_obj.get("state_vector") {
                    if let Ok(vec) = serde_json::from_value::<Vec<f32>>(vector.clone()) {
                        let score = cosine_similarity(query, &vec);
                        scores.push((score, exp.clone()));
                    }
                }
            }
        }

        // Sort by score descending
        scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        // Take top K
        for (_, exp) in scores.iter().take(limit) {
            results.push(exp.clone());
        }

        Ok(results)
    }
}

/// Calculate cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let magnitude_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let magnitude_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        return 0.0;
    }

    dot_product / (magnitude_a * magnitude_b)
}

/// Set panic hook for better error messages in browser
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let similarity = cosine_similarity(&a, &b);
        assert!((similarity - 1.0).abs() < 0.001);

        let c = vec![1.0, 0.0, 0.0];
        let d = vec![0.0, 1.0, 0.0];
        let similarity = cosine_similarity(&c, &d);
        assert!(similarity.abs() < 0.001);
    }

    #[test]
    fn test_config_creation() {
        let config = WasmAgentDbConfig::new();
        assert_eq!(config.db_name, "arcadia_agents");
        assert_eq!(config.vector_dim, 1536);
    }
}
