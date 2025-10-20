// VIVIAN Framework - Vector Index Virtual Infrastructure for Autonomous Networks
// Module: Core Framework Entry Point

pub mod vector_index;
pub mod distributed;
pub mod network;
pub mod storage;

use std::sync::Arc;
use tokio::sync::RwLock;

/// VIVIAN Framework Configuration
#[derive(Debug, Clone)]
pub struct VivianConfig {
    /// Vector index configuration
    pub vector_config: vector_index::VectorIndexConfig,
    /// Distributed system configuration
    pub distributed_config: distributed::DistributedConfig,
    /// Network protocol configuration
    pub network_config: network::NetworkConfig,
    /// Storage backend configuration
    pub storage_config: storage::StorageConfig,
}

/// Core VIVIAN Framework
///
/// Provides the foundation for vector-based AI infrastructure with
/// decentralized capabilities, efficient storage, and network protocols.
pub struct VivianFramework {
    config: VivianConfig,
    vector_index: Arc<RwLock<vector_index::VectorIndexManager>>,
    distributed: Arc<RwLock<distributed::DistributedManager>>,
    network: Arc<RwLock<network::NetworkManager>>,
    storage: Arc<RwLock<storage::StorageManager>>,
}

impl VivianFramework {
    /// Create a new VIVIAN framework instance
    pub async fn new(config: VivianConfig) -> Result<Self, VivianError> {
        let vector_index = Arc::new(RwLock::new(
            vector_index::VectorIndexManager::new(config.vector_config.clone()).await?
        ));

        let distributed = Arc::new(RwLock::new(
            distributed::DistributedManager::new(config.distributed_config.clone()).await?
        ));

        let network = Arc::new(RwLock::new(
            network::NetworkManager::new(config.network_config.clone()).await?
        ));

        let storage = Arc::new(RwLock::new(
            storage::StorageManager::new(config.storage_config.clone()).await?
        ));

        Ok(Self {
            config,
            vector_index,
            distributed,
            network,
            storage,
        })
    }

    /// Initialize the VIVIAN framework
    pub async fn initialize(&self) -> Result<(), VivianError> {
        // Initialize all subsystems in parallel
        let (vector_result, distributed_result, network_result, storage_result) = tokio::join!(
            self.vector_index.write().await.initialize(),
            self.distributed.write().await.initialize(),
            self.network.write().await.initialize(),
            self.storage.write().await.initialize()
        );

        vector_result?;
        distributed_result?;
        network_result?;
        storage_result?;

        Ok(())
    }

    /// Get vector index manager reference
    pub fn vector_index(&self) -> Arc<RwLock<vector_index::VectorIndexManager>> {
        Arc::clone(&self.vector_index)
    }

    /// Get distributed manager reference
    pub fn distributed(&self) -> Arc<RwLock<distributed::DistributedManager>> {
        Arc::clone(&self.distributed)
    }

    /// Get network manager reference
    pub fn network(&self) -> Arc<RwLock<network::NetworkManager>> {
        Arc::clone(&self.network)
    }

    /// Get storage manager reference
    pub fn storage(&self) -> Arc<RwLock<storage::StorageManager>> {
        Arc::clone(&self.storage)
    }

    /// Shutdown the VIVIAN framework gracefully
    pub async fn shutdown(&self) -> Result<(), VivianError> {
        let (vector_result, distributed_result, network_result, storage_result) = tokio::join!(
            self.vector_index.write().await.shutdown(),
            self.distributed.write().await.shutdown(),
            self.network.write().await.shutdown(),
            self.storage.write().await.shutdown()
        );

        vector_result?;
        distributed_result?;
        network_result?;
        storage_result?;

        Ok(())
    }
}

/// VIVIAN Framework Error Types
#[derive(Debug, thiserror::Error)]
pub enum VivianError {
    #[error("Vector index error: {0}")]
    VectorIndex(#[from] vector_index::VectorIndexError),

    #[error("Distributed system error: {0}")]
    Distributed(#[from] distributed::DistributedError),

    #[error("Network error: {0}")]
    Network(#[from] network::NetworkError),

    #[error("Storage error: {0}")]
    Storage(#[from] storage::StorageError),

    #[error("Initialization error: {0}")]
    Initialization(String),
}
