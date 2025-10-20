# AgentDB Module

Persistent learning and memory system for ARCADIA agents.

## Overview

The AgentDB module provides ARCADIA with powerful capabilities for agent learning, memory persistence, and experience management. It enables agents to:

- **Remember** past experiences across sessions
- **Learn** from historical interactions using vector similarity
- **Replay** important moments for training
- **Adapt** behavior based on pattern recognition

## Module Structure

```
agentdb/
├── mod.rs                  - Main module, AgentDbManager
├── wasm_bindings.rs        - JavaScript/WASM interface
├── memory_persistence.rs   - Storage backends (File/IndexedDB/Memory)
├── learning_database.rs    - Vector-based learning with patterns
└── experience_replay.rs    - Circular buffer with prioritization
```

## Core Components

### AgentDbManager

Central coordinator for all AgentDB operations.

```rust
use arcadia::agentdb::{AgentDbManager, AgentDbConfig};

let config = AgentDbConfig::default();
let mut manager = AgentDbManager::new(config).await?;
manager.initialize().await?;
```

### Memory Persistence

Handles storage and retrieval with multiple backends:

- **File**: Native persistent storage
- **IndexedDB**: Browser-based storage (WASM)
- **Memory**: Fast in-memory cache

### Learning Database

Vector-based experience storage with:

- Cosine similarity search
- Automatic pattern detection
- Success/failure classification
- Performance analytics

### Experience Replay

Reinforcement learning replay buffer with:

- Circular buffer implementation
- Priority-based sampling
- High-reward filtering
- Episode tracking

## Usage Examples

### Basic Storage and Retrieval

```rust
use arcadia::agentdb::{AgentDbManager, AgentExperience};
use std::collections::HashMap;

let mut manager = AgentDbManager::new(config).await?;
manager.initialize().await?;

// Create experience
let experience = AgentExperience {
    id: "exp_1".to_string(),
    agent_id: "agent_1".to_string(),
    state_vector: vec![0.5; 1536],
    action: "move_forward".to_string(),
    reward: 1.0,
    next_state_vector: vec![0.6; 1536],
    done: false,
    metadata: HashMap::new(),
    timestamp: chrono::Utc::now().timestamp(),
};

// Store
manager.store_experience("agent_1", experience).await?;

// Retrieve
let memories = manager.get_agent_memory("agent_1").await?;
```

### Vector Similarity Search

```rust
// Query similar experiences
let query_vector = vec![0.5; 1536];
let similar = manager.query_similar_experiences(query_vector, 10).await?;

for exp in similar {
    println!("Similar: {} (reward: {})", exp.action, exp.reward);
}
```

### Experience Replay for Training

```rust
// Sample batch for training
let batch_size = 32;
let experiences = manager.sample_replay_batch(batch_size)?;

// Use for reinforcement learning
for exp in experiences {
    // Train neural network with experience...
}
```

### Pattern Analysis

```rust
use arcadia::agentdb::learning_database::{LearningDatabase, PatternType};

let db = LearningDatabase::new(&config).await?;

// Get successful patterns
let successes = db.get_by_pattern(PatternType::Success).await?;

// Get all detected patterns
let patterns = db.get_patterns().await;
for pattern in patterns {
    println!("Pattern: {:?}, confidence: {}",
        pattern.pattern_type, pattern.confidence);
}
```

## WASM Integration

### Browser Usage

```javascript
import init, { WasmAgentDb, WasmAgentDbConfig } from './pkg/arcadia.js';

await init();

const config = new WasmAgentDbConfig();
const agentDb = new WasmAgentDb(config);
await agentDb.initialize();

// Store experience
await agentDb.store_experience('agent_1', {
    id: 'exp_1',
    agent_id: 'agent_1',
    state_vector: new Array(1536).fill(0.5),
    action: 'move_forward',
    reward: 1.0,
    // ...
});

// Query
const results = await agentDb.query_similar(queryVector, 10);
```

### Node.js Usage

```javascript
const { WasmAgentDb, WasmAgentDbConfig } = require('./pkg-node/arcadia.js');

const config = new WasmAgentDbConfig();
const agentDb = new WasmAgentDb(config);
await agentDb.initialize();

// Same API as browser...
```

## Configuration

### AgentDbConfig

```rust
pub struct AgentDbConfig {
    /// Database name
    pub db_name: String,

    /// Vector dimension (must match embeddings)
    pub vector_dim: usize,

    /// Enable WASM mode
    pub wasm_enabled: bool,

    /// Maximum memory in MB
    pub max_memory_mb: usize,

    /// Replay buffer size
    pub replay_buffer_size: usize,

    /// Auto-save interval (seconds)
    pub auto_save_interval: u64,

    /// Enable compression
    pub enable_compression: bool,
}
```

### Default Values

```rust
AgentDbConfig {
    db_name: "arcadia_agents".to_string(),
    vector_dim: 1536,
    wasm_enabled: false,
    max_memory_mb: 512,
    replay_buffer_size: 10000,
    auto_save_interval: 300,
    enable_compression: true,
}
```

## Performance

### Native Performance

- Storage: <1ms per experience
- Query (1K experiences): ~5-10ms
- Batch sampling: ~1ms for 32 experiences
- Memory: ~1MB per 1K experiences

### WASM Performance

- Initial load: ~100ms
- Storage: <1ms
- Query: ~10ms for 1K experiences
- Browser memory limit: ~2GB

## Testing

```bash
# Run module tests
cargo test --lib agentdb

# Run integration tests
cargo test --test agentdb_integration_test

# Run WASM tests
wasm-pack test --headless --firefox

# Run with output
cargo test -- --nocapture
```

## Integration Points

### With PARIS Framework

```rust
use arcadia::paris::learning::LearningManager;

let paris = LearningManager::new(paris_config).await?;
let agentdb = AgentDbManager::new(agentdb_config).await?;

// Sample experiences for training
let batch = agentdb.sample_replay_batch(32)?;

// Feed to PARIS for learning
// paris.process_experiences(&batch).await?;
```

### With VIVIAN Framework

```rust
use arcadia::vector_index::VectorIndex;

// Store experience embeddings in VIVIAN
let embedding = vector_index.embed(&experience.action).await?;
experience.state_vector = embedding;

// Store in AgentDB
agentdb.store_experience("agent_1", experience).await?;
```

## Error Handling

```rust
use arcadia::agentdb::AgentDbError;

match manager.store_experience("agent_1", exp).await {
    Ok(_) => println!("Stored successfully"),
    Err(AgentDbError::StorageError(e)) => {
        eprintln!("Storage failed: {}", e);
    },
    Err(AgentDbError::SerializationError(e)) => {
        eprintln!("Serialization failed: {}", e);
    },
    Err(e) => eprintln!("Error: {}", e),
}
```

## Best Practices

### 1. Memory Management

```rust
// Configure appropriate limits
let config = AgentDbConfig {
    max_memory_mb: 256,  // Set based on available RAM
    replay_buffer_size: 5000,  // Adjust for use case
    ..Default::default()
};
```

### 2. Periodic Saves

```rust
// Save regularly to prevent data loss
tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_secs(300));
    loop {
        interval.tick().await;
        manager.save().await?;
    }
});
```

### 3. Efficient Queries

```rust
// Use appropriate vector dimensions
let config = AgentDbConfig {
    vector_dim: 384,  // Use smaller dim if possible
    ..Default::default()
};

// Query with reasonable limits
let results = manager.query_similar_experiences(query, 20).await?;
```

### 4. Priority Sampling

```rust
use arcadia::agentdb::experience_replay::Priority;

// Mark important experiences
replay.add_with_priority(critical_exp, Priority::Critical)?;

// Sample with priority bias
let batch = replay.sample_prioritized(32)?;
```

## Advanced Features

### Episode Tracking

```rust
use arcadia::agentdb::experience_replay::Episode;

let mut episode = Episode::new("ep_1".to_string(), "agent_1".to_string());
episode.add_experience(exp1);
episode.add_experience(exp2);

println!("Episode reward: {}", episode.total_reward);
println!("Average: {}", episode.avg_reward());
```

### Pattern Detection

```rust
// Patterns are detected automatically
let patterns = learning_db.get_patterns().await;

for pattern in patterns {
    match pattern.pattern_type {
        PatternType::Success => {
            println!("Success pattern: {} occurrences",
                pattern.occurrence_count);
        },
        PatternType::Failure => {
            println!("Failure pattern to avoid");
        },
        _ => {}
    }
}
```

### High-Reward Filtering

```rust
// Get top 20% of experiences
let top_experiences = replay.get_high_reward(0.2);

// Use for imitation learning
for exp in top_experiences {
    // Train policy to imitate these actions
}
```

## Troubleshooting

### Memory Issues

```rust
// Monitor memory usage
let stats = manager.get_stats();
println!("Memory: {}MB", stats.memory_usage_mb);

// Reduce if needed
if stats.memory_usage_mb > 400.0 {
    // Clear old data or reduce buffer size
}
```

### Performance Issues

```rust
// Use smaller vector dimensions
let config = AgentDbConfig {
    vector_dim: 256,  // Instead of 1536
    ..Default::default()
};

// Limit query results
let results = manager.query_similar_experiences(query, 5).await?;
```

### WASM Compatibility

```rust
// Check if running in WASM
#[cfg(target_arch = "wasm32")]
{
    config.wasm_enabled = true;
}
```

## Future Enhancements

- Distributed storage support
- GPU-accelerated similarity search
- Automatic experience prioritization
- Model versioning and checkpoints
- Advanced pattern recognition
- Cloud storage backends

## References

- Main documentation: `/docs/AGENTDB_INTEGRATION.md`
- Examples: `/examples/agentdb_*`
- Tests: `/tests/agentdb_integration_test.rs`
- PARIS integration: `/src/paris/learning.rs`
