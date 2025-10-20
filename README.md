# ARCADIA

[![Crates.io](https://img.shields.io/crates/v/arcadia.svg)](https://crates.io/crates/arcadia)
[![Documentation](https://docs.rs/arcadia/badge.svg)](https://docs.rs/arcadia)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![CI](https://github.com/ruvnet/arcadia/workflows/CI/badge.svg)](https://github.com/ruvnet/arcadia/actions)

**Advanced and Responsive Computational Architecture for Dynamic Interactive AI**

ARCADIA is a high-performance, AI-driven game engine designed for creating dynamic, adaptive, and emotionally responsive gaming experiences. Built on the innovative VIVIAN and PARIS frameworks, ARCADIA enables developers to create game worlds that evolve, learn, and adapt to player behavior in real-time.

## Features

- **VIVIAN Framework**: Vector Index Virtual Infrastructure for Autonomous Networks - efficient storage and retrieval of high-dimensional game data
- **PARIS Framework**: Perpetual Adaptive Regenerative Intelligence System - continuous learning and optimization
- **AI-Driven NPCs**: Emotionally intelligent characters that adapt to player interactions
- **Procedural Generation**: Dynamic world generation using Code DNA system
- **Performance Optimized**: SIMD-accelerated operations, memory pooling, and lock-free concurrency
- **Semantic Search**: Vector-based semantic search for intelligent game state management
- **Async/Await**: Non-blocking I/O for maximum performance
- **WebAssembly Support**: Browser-based gaming with IndexedDB persistence

## Quick Start

Add ARCADIA to your `Cargo.toml`:

```toml
[dependencies]
arcadia = "0.1.0"
tokio = { version = "1.40", features = ["full"] }
```

### Basic Example

```rust
use arcadia::{
    code_dna::{CodeDNA, GameWorld},
    vector_index::{VectorIndex, VectorIndexConfig},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a game world with sci-fi DNA
    let dna = CodeDNA::default_scifi();
    let mut world = GameWorld::new();
    dna.apply_to_game_world(&mut world);

    // Initialize vector index for semantic game state
    let config = VectorIndexConfig {
        api_key: std::env::var("OPENAI_API_KEY")?,
        collection_name: "my_game".to_string(),
        ..Default::default()
    };

    let index = VectorIndex::new(config).await?;

    // Store game entities with semantic understanding
    index.store(
        Some("player".to_string()),
        "Human player with laser rifle and shield",
        Default::default(),
    ).await?;

    // Semantic search for game elements
    let results = index.search("Who can fight enemies?", 5).await?;

    for result in results {
        println!("Found: {} (relevance: {:.2})", result.text, result.score);
    }

    Ok(())
}
```

## Architecture

ARCADIA is built on three core frameworks:

### VIVIAN (Vector Index Virtual Infrastructure)
Provides efficient vector-based storage and retrieval for game data, enabling:
- Semantic search across game elements
- High-dimensional data indexing
- Real-time similarity matching
- Distributed vector storage with Qdrant

### PARIS (Perpetual Adaptive Regenerative Intelligence System)
Enables continuous learning and optimization through:
- Multi-layer neural architecture
- Regenerative feedback loops
- Adaptive optimization strategies
- Self-improving AI models

### aiTOML Workflow Specification
Flexible workflow definition system for:
- Autonomous AI infrastructure
- Secure key management
- AI governance and auditing
- Multi-language support

## Core Concepts

### Code DNA
Define the fundamental attributes of your game world:

```rust
use arcadia::code_dna::CodeDNA;

let dna = CodeDNA {
    theme: "cyberpunk".to_string(),
    time_scale: 1.0,
    entropy_rate: 0.1,
    physical_laws: vec!["gravity".to_string(), "cybernetics".to_string()],
    ..Default::default()
};
```

### Emotional AI
Create NPCs with emotional intelligence:

```rust
use arcadia::ai::emotion::{EmotionalState, EmotionalEngine};

let mut engine = EmotionalEngine::new();
engine.process_event("player_helped_npc");

let state = engine.get_emotional_state();
println!("NPC feels: {:?} (intensity: {})", state.primary_emotion, state.intensity);
```

### Adaptive Learning
Enable NPCs to learn from interactions:

```rust
use arcadia::ai::evolutionary::EvolutionaryEngine;

let mut evolution = EvolutionaryEngine::new();
evolution.evaluate_behavior("defensive_tactic", 0.85);
let next_behavior = evolution.select_best_behavior();
```

## Examples

ARCADIA includes comprehensive examples:

- **`basic_game`** - Simple game setup with vector index and caching
- **`ai_npc`** - Emotionally intelligent NPC with adaptive behavior
- **`npc_ai_example`** - Advanced NPC decision-making system

Run examples with:

```bash
cargo run --example basic_game
cargo run --example ai_npc
```

## Performance

ARCADIA is optimized for high-performance gaming:

- **Zero-copy operations** where possible
- **SIMD-accelerated** vector computations
- **Memory pooling** for reduced allocations
- **Lock-free** concurrent data structures
- **Embedding cache** for 10-100x speedup on repeated queries
- **Benchmarks included** for performance validation

Run benchmarks:

```bash
cargo bench
```

## Documentation

- **[API Reference](https://docs.rs/arcadia)** - Complete API documentation
- **[Architecture Guide](docs/architecture/ARCHITECTURE.md)** - System design and architecture
- **[Integration Guide](docs/architecture/INTEGRATION_GUIDE.md)** - Integrate with Unreal Engine 5
- **[Tutorials](docs/tutorials/)** - Step-by-step guides
- **[Whitepaper](README_WHITEPAPER.md)** - Comprehensive technical overview

## Requirements

- Rust 1.75 or later
- OpenAI API key (for embeddings)
- Optional: Qdrant instance for distributed vector storage
- Optional: PostgreSQL/SQLite for persistent storage

## Installation

### From crates.io

```bash
cargo add arcadia
```

### From source

```bash
git clone https://github.com/ruvnet/arcadia.git
cd arcadia
cargo build --release
```

## Configuration

Create a `config.toml` file:

```toml
[vector_index]
api_key = "your-openai-api-key"
collection_name = "game_world"
embedding_model = "text-embedding-3-small"
vector_dimension = 1536

[qdrant]
url = "http://localhost:6333"
timeout_secs = 30

[cache]
max_size_mb = 256
ttl_secs = 3600
```

Or use environment variables:

```bash
export OPENAI_API_KEY="your-api-key"
export QDRANT_URL="http://localhost:6333"
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Clone repository
git clone https://github.com/ruvnet/arcadia.git
cd arcadia

# Install dependencies
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=arcadia=debug cargo run --example basic_game
```

## Testing

ARCADIA includes comprehensive tests:

```bash
# Run all tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Run with output
cargo test -- --nocapture
```

## Use Cases

- **Dynamic RPGs**: Create worlds that evolve based on player choices
- **Adaptive NPCs**: Characters that learn and respond emotionally
- **Procedural Worlds**: Generate unique environments using Code DNA
- **AI-Driven Narratives**: Stories that adapt to player behavior
- **Emotional Gaming**: Games that respond to player emotional state
- **Semantic Game State**: Intelligent search and retrieval of game elements

## Roadmap

- [ ] Unreal Engine 5 plugin
- [ ] Unity integration
- [ ] Real-time multiplayer support
- [ ] Enhanced emotional AI models
- [ ] Cloud-based vector storage
- [ ] Visual workflow designer for aiTOML

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Acknowledgments

ARCADIA builds upon the research and development of:
- VIVIAN (Vector Index Virtual Infrastructure for Autonomous Networks)
- PARIS (Perpetual Adaptive Regenerative Intelligence System)
- aiTOML Workflow Specification

## Support

- **Documentation**: https://docs.rs/arcadia
- **Issues**: https://github.com/ruvnet/arcadia/issues
- **Discussions**: https://github.com/ruvnet/arcadia/discussions

## Citation

If you use ARCADIA in your research or project, please cite:

```bibtex
@software{arcadia2024,
  title = {ARCADIA: Advanced and Responsive Computational Architecture for Dynamic Interactive AI},
  author = {Cohen, Reuven},
  year = {2024},
  url = {https://github.com/ruvnet/arcadia}
}
```

---

**Built with Rust. Powered by AI. Ready for the future of gaming.**
