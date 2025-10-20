# ARCADIA API Reference

Complete API reference for the ARCADIA engine, including VIVIAN and PARIS frameworks.

## Table of Contents

1. [VIVIAN Framework API](#vivian-framework-api)
   - [Vector Index](#vector-index-api)
   - [Distributed](#distributed-api)
   - [Network](#network-api)
   - [Storage](#storage-api)
2. [PARIS Framework API](#paris-framework-api)
   - [Feedback](#feedback-api)
   - [Learning](#learning-api)
   - [Optimization](#optimization-api)
   - [Layers](#layers-api)
3. [Core Integration API](#core-integration-api)

---

## VIVIAN Framework API

### Vector Index API

#### `VectorIndexManager::new(config: VectorIndexConfig)`
Creates a new vector index manager instance.

**Parameters**:
- `config`: Configuration for vector index operations
  - `dimension`: Vector dimension size (e.g., 512)
  - `metric`: Similarity metric to use
  - `index_type`: Type of index structure
  - `capacity`: Maximum number of vectors
  - `sharding_enabled`: Enable sharding for scalability
  - `shard_count`: Number of shards

**Returns**: `Result<VectorIndexManager, VectorIndexError>`

**Example**:
```rust
let config = VectorIndexConfig {
    dimension: 512,
    metric: SimilarityMetric::Cosine,
    index_type: IndexType::HNSW,
    capacity: 1_000_000,
    sharding_enabled: true,
    shard_count: 4,
};

let manager = VectorIndexManager::new(config).await?;
```

#### `VectorIndexManager::create_index(name: String)`
Creates a named vector index.

**Parameters**:
- `name`: Unique identifier for the index

**Returns**: `Result<(), VectorIndexError>`

#### `VectorIndexManager::add_embedding(index_name: &str, embedding: VectorEmbedding)`
Adds a vector embedding to the specified index.

**Parameters**:
- `index_name`: Name of the target index
- `embedding`: Vector embedding with metadata

**Returns**: `Result<(), VectorIndexError>`

#### `VectorIndexManager::search(index_name: &str, query: &[f32], k: usize)`
Performs similarity search for the k nearest neighbors.

**Parameters**:
- `index_name`: Name of the index to search
- `query`: Query vector
- `k`: Number of results to return

**Returns**: `Result<Vec<SearchResult>, VectorIndexError>`

#### `VectorIndexManager::batch_add(index_name: &str, embeddings: Vec<VectorEmbedding>)`
Batch adds multiple embeddings for improved performance.

**Parameters**:
- `index_name`: Name of the target index
- `embeddings`: Vector of embeddings to add

**Returns**: `Result<(), VectorIndexError>`

#### `VectorIndexManager::get_stats(index_name: &str)`
Returns statistics for the specified index.

**Returns**: `Result<IndexStats, VectorIndexError>`

---

### Distributed API

#### `DistributedManager::new(config: DistributedConfig)`
Creates a new distributed manager instance.

**Parameters**:
- `config`: Distributed system configuration
  - `node_id`: Unique node identifier
  - `cluster_size`: Expected cluster size
  - `replication_factor`: Number of replicas
  - `consistency_level`: Consistency guarantee level
  - `gossip_interval_ms`: Gossip protocol interval
  - `heartbeat_timeout_ms`: Node failure timeout

**Returns**: `Result<DistributedManager, DistributedError>`

#### `DistributedManager::join_cluster(seed_nodes: Vec<SocketAddr>)`
Joins an existing cluster using seed nodes.

**Parameters**:
- `seed_nodes`: List of known cluster nodes

**Returns**: `Result<(), DistributedError>`

#### `DistributedManager::dht_put(key: String, value: Vec<u8>)`
Stores data in the distributed hash table.

**Parameters**:
- `key`: Unique key for the data
- `value`: Binary data to store

**Returns**: `Result<(), DistributedError>`

#### `DistributedManager::dht_get(key: &str)`
Retrieves data from the distributed hash table.

**Parameters**:
- `key`: Key to look up

**Returns**: `Result<Option<Vec<u8>>, DistributedError>`

#### `DistributedManager::get_cluster_stats()`
Returns cluster health and statistics.

**Returns**: `ClusterStats`

---

### Network API

#### `NetworkManager::new(config: NetworkConfig)`
Creates a new network manager instance.

**Parameters**:
- `config`: Network configuration
  - `listen_address`: Address to bind to
  - `protocol`: Network protocol to use
  - `max_connections`: Maximum concurrent connections
  - `connection_timeout_ms`: Connection timeout
  - `enable_encryption`: Enable TLS/encryption
  - `buffer_size`: Network buffer size

**Returns**: `Result<NetworkManager, NetworkError>`

#### `NetworkManager::connect(address: SocketAddr)`
Establishes a connection to a remote node.

**Parameters**:
- `address`: Remote node address

**Returns**: `Result<String, NetworkError>` (connection ID)

#### `NetworkManager::send(connection_id: &str, message: NetworkMessage)`
Sends a message over an established connection.

**Parameters**:
- `connection_id`: Target connection identifier
- `message`: Message to send

**Returns**: `Result<(), NetworkError>`

#### `NetworkManager::broadcast(message: NetworkMessage)`
Broadcasts a message to all connections.

**Parameters**:
- `message`: Message to broadcast

**Returns**: `Result<(), NetworkError>`

#### `NetworkManager::get_network_stats()`
Returns network statistics.

**Returns**: `NetworkStats`

---

### Storage API

#### `StorageManager::new(config: StorageConfig)`
Creates a new storage manager instance.

**Parameters**:
- `config`: Storage configuration
  - `storage_type`: Backend storage type
  - `data_path`: Path for file storage
  - `cache_size_mb`: In-memory cache size
  - `enable_compression`: Enable data compression
  - `enable_encryption`: Enable data encryption
  - `backup_enabled`: Enable automatic backups
  - `backup_interval_hours`: Backup interval

**Returns**: `Result<StorageManager, StorageError>`

#### `StorageManager::put(key: String, data: Vec<u8>)`
Stores data with the specified key.

**Parameters**:
- `key`: Unique key for the data
- `data`: Binary data to store

**Returns**: `Result<(), StorageError>`

#### `StorageManager::get(key: &str)`
Retrieves data by key.

**Parameters**:
- `key`: Key to look up

**Returns**: `Result<Option<Vec<u8>>, StorageError>`

#### `StorageManager::delete(key: &str)`
Deletes data by key.

**Parameters**:
- `key`: Key to delete

**Returns**: `Result<(), StorageError>`

#### `StorageManager::backup()`
Creates a backup of all data.

**Returns**: `Result<PathBuf, StorageError>` (backup path)

---

## PARIS Framework API

### Feedback API

#### `FeedbackManager::new(config: FeedbackConfig)`
Creates a new feedback manager instance.

**Parameters**:
- `config`: Feedback configuration
  - `max_queue_size`: Maximum feedback queue size
  - `feedback_types`: Types of feedback to collect
  - `aggregation_interval_ms`: Aggregation interval
  - `enable_filtering`: Enable priority filtering
  - `priority_threshold`: Minimum priority threshold

**Returns**: `Result<FeedbackManager, FeedbackError>`

#### `FeedbackManager::submit_feedback(feedback: FeedbackData)`
Submits a feedback data point.

**Parameters**:
- `feedback`: Feedback data to submit

**Returns**: `Result<(), FeedbackError>`

#### `FeedbackManager::process_feedback()`
Processes and aggregates pending feedback.

**Returns**: `Result<Vec<AggregatedFeedback>, FeedbackError>`

#### `FeedbackManager::get_history(feedback_type: FeedbackType)`
Retrieves historical aggregated feedback.

**Parameters**:
- `feedback_type`: Type of feedback to retrieve

**Returns**: `Result<Vec<AggregatedFeedback>, FeedbackError>`

---

### Learning API

#### `LearningManager::new(config: LearningConfig)`
Creates a new learning manager instance.

**Parameters**:
- `config`: Learning configuration
  - `learning_rate`: Base learning rate
  - `adaptation_threshold`: Adaptation sensitivity
  - `pattern_window_size`: Pattern detection window
  - `enable_online_learning`: Enable continuous learning
  - `enable_transfer_learning`: Enable transfer learning
  - `model_update_frequency`: Update frequency

**Returns**: `Result<LearningManager, LearningError>`

#### `LearningManager::register_model(model_id: String, model: AIModel)`
Registers an AI model for learning.

**Parameters**:
- `model_id`: Unique model identifier
- `model`: AI model instance

**Returns**: `Result<(), LearningError>`

#### `LearningManager::process_feedback(feedback: &[AggregatedFeedback])`
Processes feedback and updates models.

**Parameters**:
- `feedback`: Aggregated feedback data

**Returns**: `Result<LearningResult, LearningError>`

#### `LearningManager::transfer_learning(source_model: &str, target_model: &str)`
Transfers learning from source to target model.

**Parameters**:
- `source_model`: Source model ID
- `target_model`: Target model ID

**Returns**: `Result<(), LearningError>`

#### `LearningManager::get_patterns(pattern_type: Option<String>)`
Retrieves discovered patterns.

**Parameters**:
- `pattern_type`: Optional pattern type filter

**Returns**: `Result<Vec<LearningPattern>, LearningError>`

---

### Optimization API

#### `OptimizationManager::new(config: OptimizationConfig)`
Creates a new optimization manager instance.

**Parameters**:
- `config`: Optimization configuration
  - `enable_auto_tuning`: Enable automatic tuning
  - `optimization_interval_ms`: Optimization interval
  - `performance_target`: Target performance level
  - `max_optimization_iterations`: Maximum iterations
  - `convergence_threshold`: Convergence threshold

**Returns**: `Result<OptimizationManager, OptimizationError>`

#### `OptimizationManager::apply_optimizations(learning_result: &LearningResult)`
Applies optimizations based on learning results.

**Parameters**:
- `learning_result`: Results from learning phase

**Returns**: `Result<OptimizationResult, OptimizationError>`

#### `OptimizationManager::optimize_iteration(target: &str, strategy: OptimizationStrategy)`
Runs a single optimization iteration.

**Parameters**:
- `target`: Optimization target
- `strategy`: Optimization strategy to use

**Returns**: `Result<Optimization, OptimizationError>`

#### `OptimizationManager::register_hyperparameter(hyperparameter: Hyperparameter)`
Registers a hyperparameter for tuning.

**Parameters**:
- `hyperparameter`: Hyperparameter definition

**Returns**: `Result<(), OptimizationError>`

---

### Layers API

#### `LayerManager::new(config: LayerConfig)`
Creates a new layer manager instance.

**Parameters**:
- `config`: Layer configuration
  - `layers`: Layer definitions
  - `enable_layer_fusion`: Enable layer fusion
  - `enable_skip_connections`: Enable skip connections
  - `layer_communication_protocol`: Communication protocol

**Returns**: `Result<LayerManager, LayerError>`

#### `LayerManager::create_layer(definition: LayerDefinition)`
Creates a new layer.

**Parameters**:
- `definition`: Layer definition

**Returns**: `Result<(), LayerError>`

#### `LayerManager::update_layers(optimization_result: &OptimizationResult)`
Updates layers based on optimization results.

**Parameters**:
- `optimization_result`: Optimization results

**Returns**: `Result<(), LayerError>`

#### `LayerManager::process_hierarchical<T>(input: T, layer_sequence: Vec<String>)`
Processes data through layer hierarchy.

**Parameters**:
- `input`: Input data
- `layer_sequence`: Ordered list of layers to process through

**Returns**: `Result<T, LayerError>`

#### `LayerManager::fuse_layers(source_layers: Vec<String>, target_layer: String)`
Fuses multiple layers into one.

**Parameters**:
- `source_layers`: Layers to fuse
- `target_layer`: Resulting fused layer name

**Returns**: `Result<(), LayerError>`

---

## Core Integration API

### `ArcadiaEngine::new(config: ArcadiaConfig)`
Creates a new ARCADIA engine instance.

**Parameters**:
- `config`: Complete engine configuration

**Returns**: `Result<ArcadiaEngine, ArcadiaError>`

### `ArcadiaEngine::initialize()`
Initializes both VIVIAN and PARIS frameworks.

**Returns**: `Result<(), ArcadiaError>`

### `ArcadiaEngine::run_adaptive_cycle()`
Executes one complete adaptation cycle.

**Returns**: `Result<(), ArcadiaError>`

### `ArcadiaEngine::shutdown()`
Gracefully shuts down the engine.

**Returns**: `Result<(), ArcadiaError>`

---

## Error Types

### VIVIAN Errors

```rust
pub enum VivianError {
    VectorIndex(VectorIndexError),
    Distributed(DistributedError),
    Network(NetworkError),
    Storage(StorageError),
    Initialization(String),
}
```

### PARIS Errors

```rust
pub enum ParisError {
    Feedback(FeedbackError),
    Learning(LearningError),
    Optimization(OptimizationError),
    Layer(LayerError),
    Initialization(String),
}
```

---

## Common Patterns

### Initialization Pattern

```rust
// Create configuration
let vivian_config = VivianConfig { /* ... */ };
let paris_config = ParisConfig { /* ... */ };

// Create engine
let engine = ArcadiaEngine::new(ArcadiaConfig {
    vivian_config,
    paris_config,
}).await?;

// Initialize
engine.initialize().await?;
```

### Feedback Loop Pattern

```rust
// 1. Collect feedback
feedback_manager.submit_feedback(feedback_data).await?;

// 2. Process feedback
let aggregated = feedback_manager.process_feedback().await?;

// 3. Learn from feedback
let learning_result = learning_manager.process_feedback(&aggregated).await?;

// 4. Optimize
let optimization_result = optimization_manager
    .apply_optimizations(&learning_result).await?;

// 5. Update layers
layer_manager.update_layers(&optimization_result).await?;
```

### Vector Search Pattern

```rust
// 1. Create/load vector embeddings
let embedding = create_embedding(game_asset);

// 2. Add to index
vector_index.add_embedding("assets", embedding).await?;

// 3. Search for similar
let query_vector = create_query_vector(search_criteria);
let results = vector_index.search("assets", &query_vector, 10).await?;

// 4. Use results
for result in results {
    // Process similar assets
    process_asset(&result.embedding);
}
```

---

**Document Version**: 1.0
**Last Updated**: 2025-10-20
