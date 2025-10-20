//! Memory Persistence Layer for AgentDB
//!
//! Handles storage and retrieval of agent experiences across sessions
//! with support for both native and WASM environments.

use super::{AgentDbConfig, AgentDbError, AgentExperience};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Memory persistence storage backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageBackend {
    /// File-based storage (native)
    File { path: PathBuf },
    /// IndexedDB (WASM/browser)
    IndexedDb { db_name: String },
    /// In-memory only (testing)
    Memory,
}

/// Memory persistence manager
pub struct MemoryPersistence {
    config: AgentDbConfig,
    backend: StorageBackend,
    /// In-memory cache of agent memories
    memory_cache: HashMap<String, Vec<AgentExperience>>,
    /// Dirty flag for auto-save
    dirty: bool,
}

impl MemoryPersistence {
    /// Create a new memory persistence layer
    pub async fn new(config: &AgentDbConfig) -> Result<Self, AgentDbError> {
        let backend = if config.wasm_enabled {
            StorageBackend::IndexedDb {
                db_name: config.db_name.clone(),
            }
        } else {
            StorageBackend::File {
                path: PathBuf::from(format!("./data/{}.db", config.db_name)),
            }
        };

        Ok(Self {
            config: config.clone(),
            backend,
            memory_cache: HashMap::new(),
            dirty: false,
        })
    }

    /// Initialize the persistence layer
    pub async fn initialize(&mut self) -> Result<(), AgentDbError> {
        match &self.backend {
            StorageBackend::File { path } => {
                // Create data directory if it doesn't exist
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                // Load existing data if available
                if path.exists() {
                    self.load_from_file(path).await?;
                }
            }
            StorageBackend::IndexedDb { db_name } => {
                #[cfg(target_arch = "wasm32")]
                {
                    self.initialize_indexed_db(db_name).await?;
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    return Err(AgentDbError::WasmError(
                        "IndexedDB is only available in WASM builds".to_string(),
                    ));
                }
            }
            StorageBackend::Memory => {
                // No initialization needed for in-memory storage
            }
        }

        Ok(())
    }

    /// Store an agent experience
    pub async fn store(
        &mut self,
        agent_id: &str,
        experience: &AgentExperience,
    ) -> Result<(), AgentDbError> {
        // Add to memory cache
        self.memory_cache
            .entry(agent_id.to_string())
            .or_insert_with(Vec::new)
            .push(experience.clone());

        self.dirty = true;

        // Check memory limits
        self.enforce_memory_limits()?;

        Ok(())
    }

    /// Retrieve all experiences for an agent
    pub async fn retrieve(&self, agent_id: &str) -> Result<Vec<AgentExperience>, AgentDbError> {
        Ok(self
            .memory_cache
            .get(agent_id)
            .cloned()
            .unwrap_or_default())
    }

    /// Retrieve recent experiences (last N)
    pub async fn retrieve_recent(
        &self,
        agent_id: &str,
        count: usize,
    ) -> Result<Vec<AgentExperience>, AgentDbError> {
        let experiences = self.retrieve(agent_id).await?;
        let start = experiences.len().saturating_sub(count);
        Ok(experiences[start..].to_vec())
    }

    /// Save all data to storage
    pub async fn save_all(&mut self) -> Result<(), AgentDbError> {
        if !self.dirty {
            return Ok(());
        }

        match &self.backend {
            StorageBackend::File { path } => {
                self.save_to_file(path).await?;
            }
            StorageBackend::IndexedDb { db_name: _ } => {
                #[cfg(target_arch = "wasm32")]
                {
                    self.save_to_indexed_db().await?;
                }
            }
            StorageBackend::Memory => {
                // No persistent storage for memory backend
            }
        }

        self.dirty = false;
        Ok(())
    }

    /// Clear all agent memories
    pub async fn clear_agent(&mut self, agent_id: &str) -> Result<(), AgentDbError> {
        self.memory_cache.remove(agent_id);
        self.dirty = true;
        Ok(())
    }

    /// Get total memory usage in MB
    pub fn get_memory_usage_mb(&self) -> f32 {
        let total_bytes: usize = self
            .memory_cache
            .values()
            .map(|experiences| {
                experiences
                    .iter()
                    .map(|exp| {
                        std::mem::size_of_val(exp)
                            + exp.state_vector.len() * std::mem::size_of::<f32>()
                            + exp.next_state_vector.len() * std::mem::size_of::<f32>()
                    })
                    .sum::<usize>()
            })
            .sum();

        total_bytes as f32 / (1024.0 * 1024.0)
    }

    /// Shutdown and cleanup
    pub async fn shutdown(&mut self) -> Result<(), AgentDbError> {
        self.save_all().await?;
        self.memory_cache.clear();
        Ok(())
    }

    // Private methods

    /// Enforce memory limits by removing old experiences
    fn enforce_memory_limits(&mut self) -> Result<(), AgentDbError> {
        let current_usage = self.get_memory_usage_mb();
        let max_usage = self.config.max_memory_mb as f32;

        if current_usage > max_usage {
            // Remove oldest experiences until under limit
            let target_size = (max_usage * 0.8) as usize; // Aim for 80% of max

            for experiences in self.memory_cache.values_mut() {
                if experiences.len() > target_size {
                    experiences.drain(0..(experiences.len() - target_size));
                }
            }
        }

        Ok(())
    }

    /// Load data from file
    async fn load_from_file(&mut self, path: &PathBuf) -> Result<(), AgentDbError> {
        let data = std::fs::read(path)?;

        if self.config.enable_compression {
            // Decompress data (simplified - would use actual compression library)
            self.memory_cache = bincode::deserialize(&data)
                .map_err(|e| AgentDbError::SerializationError(e.to_string()))?;
        } else {
            self.memory_cache = bincode::deserialize(&data)
                .map_err(|e| AgentDbError::SerializationError(e.to_string()))?;
        }

        Ok(())
    }

    /// Save data to file
    async fn save_to_file(&self, path: &PathBuf) -> Result<(), AgentDbError> {
        let data = bincode::serialize(&self.memory_cache)
            .map_err(|e| AgentDbError::SerializationError(e.to_string()))?;

        if self.config.enable_compression {
            // Compress data (simplified - would use actual compression library)
            std::fs::write(path, data)?;
        } else {
            std::fs::write(path, data)?;
        }

        Ok(())
    }

    /// Initialize IndexedDB (WASM only)
    #[cfg(target_arch = "wasm32")]
    async fn initialize_indexed_db(&self, _db_name: &str) -> Result<(), AgentDbError> {
        // TODO: Implement using rexie crate
        Ok(())
    }

    /// Save to IndexedDB (WASM only)
    #[cfg(target_arch = "wasm32")]
    async fn save_to_indexed_db(&self) -> Result<(), AgentDbError> {
        // TODO: Implement using rexie crate
        Ok(())
    }
}

/// Memory statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_agents: usize,
    pub total_experiences: usize,
    pub memory_usage_mb: f32,
    pub oldest_timestamp: Option<i64>,
    pub newest_timestamp: Option<i64>,
}

impl MemoryPersistence {
    /// Get memory statistics
    pub fn get_stats(&self) -> MemoryStats {
        let total_experiences: usize = self.memory_cache.values().map(|v| v.len()).sum();

        let mut oldest: Option<i64> = None;
        let mut newest: Option<i64> = None;

        for experiences in self.memory_cache.values() {
            for exp in experiences {
                oldest = Some(oldest.map_or(exp.timestamp, |o| o.min(exp.timestamp)));
                newest = Some(newest.map_or(exp.timestamp, |n| n.max(exp.timestamp)));
            }
        }

        MemoryStats {
            total_agents: self.memory_cache.len(),
            total_experiences,
            memory_usage_mb: self.get_memory_usage_mb(),
            oldest_timestamp: oldest,
            newest_timestamp: newest,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_persistence_creation() {
        let config = AgentDbConfig {
            wasm_enabled: false,
            ..Default::default()
        };

        let persistence = MemoryPersistence::new(&config).await;
        assert!(persistence.is_ok());
    }

    #[tokio::test]
    async fn test_store_and_retrieve() {
        let config = AgentDbConfig {
            wasm_enabled: false,
            ..Default::default()
        };

        let mut persistence = MemoryPersistence::new(&config).await.unwrap();
        persistence.initialize().await.unwrap();

        let experience = AgentExperience {
            id: "exp_1".to_string(),
            agent_id: "agent_1".to_string(),
            state_vector: vec![0.1; 10],
            action: "test".to_string(),
            reward: 1.0,
            next_state_vector: vec![0.2; 10],
            done: false,
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now().timestamp(),
        };

        persistence.store("agent_1", &experience).await.unwrap();

        let retrieved = persistence.retrieve("agent_1").await.unwrap();
        assert_eq!(retrieved.len(), 1);
        assert_eq!(retrieved[0].id, "exp_1");
    }

    #[tokio::test]
    async fn test_memory_limits() {
        let config = AgentDbConfig {
            wasm_enabled: false,
            max_memory_mb: 1, // Very small limit for testing
            ..Default::default()
        };

        let mut persistence = MemoryPersistence::new(&config).await.unwrap();
        persistence.initialize().await.unwrap();

        // Add many experiences
        for i in 0..1000 {
            let experience = AgentExperience {
                id: format!("exp_{}", i),
                agent_id: "agent_1".to_string(),
                state_vector: vec![0.1; 1536],
                action: "test".to_string(),
                reward: 1.0,
                next_state_vector: vec![0.2; 1536],
                done: false,
                metadata: HashMap::new(),
                timestamp: chrono::Utc::now().timestamp(),
            };

            persistence.store("agent_1", &experience).await.unwrap();
        }

        let usage = persistence.get_memory_usage_mb();
        assert!(usage <= config.max_memory_mb as f32);
    }
}
