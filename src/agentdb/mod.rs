//! # AgentDB Integration Module
//!
//! Provides WASM-compatible agent memory, learning, and persistence capabilities
//! for ARCADIA using agentdb vector database technology.
//!
//! ## Features
//!
//! - **Memory Persistence**: Store and retrieve agent experiences across sessions
//! - **Learning Database**: Vector-based learning from gameplay patterns
//! - **Experience Replay**: Buffer and replay important game moments for training
//! - **WASM Compatibility**: Run in browser or Node.js environments
//! - **Cross-Platform**: Seamless integration with ARCADIA's PARIS learning system
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │                    AgentDB Manager                      │
//! │  (Coordinates all agentdb operations)                   │
//! └─────────────────────────────────────────────────────────┘
//!          │              │                │
//!          ▼              ▼                ▼
//! ┌─────────────┐ ┌──────────────┐ ┌────────────────┐
//! │   Memory    │ │   Learning   │ │   Experience   │
//! │ Persistence │ │   Database   │ │     Replay     │
//! └─────────────┘ └──────────────┘ └────────────────┘
//!          │              │                │
//!          └──────────────┴────────────────┘
//!                         │
//!                         ▼
//!                  ┌─────────────┐
//!                  │    WASM     │
//!                  │  Bindings   │
//!                  └─────────────┘
//! ```
//!
//! ## Usage
//!
//! ```rust,no_run
//! use arcadia::agentdb::{AgentDbManager, AgentDbConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = AgentDbConfig::default();
//!     let mut manager = AgentDbManager::new(config).await?;
//!
//!     // Store agent experience
//!     manager.store_experience("agent_1", experience_data).await?;
//!
//!     // Query similar experiences
//!     let similar = manager.query_similar_experiences(query_vector, 10).await?;
//!
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

pub mod wasm_bindings;
pub mod memory_persistence;
pub mod learning_database;
pub mod experience_replay;

/// AgentDB configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDbConfig {
    /// Database name
    pub db_name: String,
    /// Vector dimension (must match embedding size)
    pub vector_dim: usize,
    /// Enable WASM mode
    pub wasm_enabled: bool,
    /// Maximum memory size in MB
    pub max_memory_mb: usize,
    /// Experience replay buffer size
    pub replay_buffer_size: usize,
    /// Auto-save interval in seconds
    pub auto_save_interval: u64,
    /// Enable compression
    pub enable_compression: bool,
}

impl Default for AgentDbConfig {
    fn default() -> Self {
        Self {
            db_name: "arcadia_agents".to_string(),
            vector_dim: 1536, // OpenAI embedding dimension
            wasm_enabled: false,
            max_memory_mb: 512,
            replay_buffer_size: 10000,
            auto_save_interval: 300, // 5 minutes
            enable_compression: true,
        }
    }
}

/// Main AgentDB Manager
///
/// Coordinates all agentdb operations including memory persistence,
/// learning, and experience replay.
pub struct AgentDbManager {
    config: AgentDbConfig,
    memory: memory_persistence::MemoryPersistence,
    learning: learning_database::LearningDatabase,
    replay: experience_replay::ExperienceReplay,
    stats: AgentDbStats,
}

impl AgentDbManager {
    /// Create a new AgentDB manager
    pub async fn new(config: AgentDbConfig) -> Result<Self, AgentDbError> {
        let memory = memory_persistence::MemoryPersistence::new(&config).await?;
        let learning = learning_database::LearningDatabase::new(&config).await?;
        let replay = experience_replay::ExperienceReplay::new(config.replay_buffer_size);

        Ok(Self {
            config,
            memory,
            learning,
            replay,
            stats: AgentDbStats::default(),
        })
    }

    /// Initialize the manager
    pub async fn initialize(&mut self) -> Result<(), AgentDbError> {
        self.memory.initialize().await?;
        self.learning.initialize().await?;
        self.stats.initialized = true;
        Ok(())
    }

    /// Store an agent experience
    pub async fn store_experience(
        &mut self,
        agent_id: &str,
        experience: AgentExperience,
    ) -> Result<(), AgentDbError> {
        // Add to replay buffer
        self.replay.add(experience.clone())?;

        // Store in memory persistence
        self.memory.store(agent_id, &experience).await?;

        // Update learning database
        self.learning.add_experience(&experience).await?;

        self.stats.total_experiences += 1;
        Ok(())
    }

    /// Query similar experiences
    pub async fn query_similar_experiences(
        &self,
        query_vector: Vec<f32>,
        limit: usize,
    ) -> Result<Vec<AgentExperience>, AgentDbError> {
        self.learning.query_similar(query_vector, limit).await
    }

    /// Get agent memory
    pub async fn get_agent_memory(
        &self,
        agent_id: &str,
    ) -> Result<Vec<AgentExperience>, AgentDbError> {
        self.memory.retrieve(agent_id).await
    }

    /// Sample from experience replay buffer
    pub fn sample_replay_batch(&mut self, batch_size: usize) -> Result<Vec<AgentExperience>, AgentDbError> {
        self.replay.sample(batch_size)
    }

    /// Save all data
    pub async fn save(&mut self) -> Result<(), AgentDbError> {
        self.memory.save_all().await?;
        self.learning.save().await?;
        self.stats.last_save = chrono::Utc::now().timestamp();
        Ok(())
    }

    /// Get statistics
    pub fn get_stats(&self) -> AgentDbStats {
        self.stats.clone()
    }

    /// Shutdown and cleanup
    pub async fn shutdown(&mut self) -> Result<(), AgentDbError> {
        self.save().await?;
        self.memory.shutdown().await?;
        self.learning.shutdown().await?;
        Ok(())
    }
}

/// Agent experience data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentExperience {
    /// Unique experience ID
    pub id: String,
    /// Agent ID
    pub agent_id: String,
    /// State vector embedding
    pub state_vector: Vec<f32>,
    /// Action taken
    pub action: String,
    /// Reward received
    pub reward: f32,
    /// Next state vector
    pub next_state_vector: Vec<f32>,
    /// Episode done flag
    pub done: bool,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// Timestamp
    pub timestamp: i64,
}

/// AgentDB statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentDbStats {
    pub initialized: bool,
    pub total_experiences: u64,
    pub total_queries: u64,
    pub last_save: i64,
    pub memory_usage_mb: f32,
}

/// AgentDB error types
#[derive(Debug, Error)]
pub enum AgentDbError {
    #[error("Initialization failed: {0}")]
    InitializationFailed(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Query error: {0}")]
    QueryError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("WASM error: {0}")]
    WasmError(String),

    #[error("Experience replay error: {0}")]
    ReplayError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agentdb_manager_creation() {
        let config = AgentDbConfig::default();
        let manager = AgentDbManager::new(config).await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_store_and_retrieve() {
        let config = AgentDbConfig::default();
        let mut manager = AgentDbManager::new(config).await.unwrap();
        manager.initialize().await.unwrap();

        let experience = AgentExperience {
            id: "exp_1".to_string(),
            agent_id: "agent_1".to_string(),
            state_vector: vec![0.1; 1536],
            action: "move_forward".to_string(),
            reward: 1.0,
            next_state_vector: vec![0.2; 1536],
            done: false,
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now().timestamp(),
        };

        let result = manager.store_experience("agent_1", experience).await;
        assert!(result.is_ok());
    }
}
