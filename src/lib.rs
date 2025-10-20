//! # ARCADIA - Advanced and Responsive Computational Architecture for Dynamic Interactive AI
//!
//! [![Crates.io](https://img.shields.io/crates/v/arcadia.svg)](https://crates.io/crates/arcadia)
//! [![Documentation](https://docs.rs/arcadia/badge.svg)](https://docs.rs/arcadia)
//! [![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/ruvnet/arcadia)
//!
//! A high-performance, AI-driven game engine designed for real-time, dynamic, and emotionally
//! adaptive gaming experiences. Built on the innovative VIVIAN and PARIS frameworks, ARCADIA
//! enables developers to create game worlds that evolve, learn, and adapt to player behavior.
//!
//! ## Features
//!
//! - **VIVIAN Framework**: Vector-based storage and semantic search for game data
//! - **PARIS Framework**: Continuous learning and optimization system
//! - **AI-Driven NPCs**: Emotionally intelligent characters that adapt to player interactions
//! - **Procedural Generation**: Dynamic world generation using Code DNA system
//! - **Performance Optimized**: SIMD acceleration, memory pooling, lock-free concurrency
//! - **WebAssembly Support**: Browser-based gaming with IndexedDB persistence
//!
//! ## Quick Start
//!
//! Add ARCADIA to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! arcadia = "0.1.0"
//! tokio = { version = "1.40", features = ["full"] }
//! ```
//!
//! ### Basic Example
//!
//! ```rust,no_run
//! use arcadia::{
//!     code_dna::{CodeDNA, GameWorld},
//!     vector_index::{VectorIndex, VectorIndexConfig},
//! };
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Create a game world with sci-fi DNA
//!     let dna = CodeDNA::default_scifi();
//!     let mut world = GameWorld::new();
//!     dna.apply_to_game_world(&mut world);
//!
//!     // Initialize vector index for semantic game state
//!     let config = VectorIndexConfig {
//!         api_key: std::env::var("OPENAI_API_KEY")?,
//!         collection_name: "my_game".to_string(),
//!         ..Default::default()
//!     };
//!
//!     let index = VectorIndex::new(config).await?;
//!
//!     // Store game entities with semantic understanding
//!     index.store(
//!         Some("player".to_string()),
//!         "Human player with laser rifle and shield",
//!         Default::default(),
//!     ).await?;
//!
//!     // Semantic search for game elements
//!     let results = index.search("Who can fight enemies?", 5).await?;
//!
//!     for result in results {
//!         println!("Found: {} (relevance: {:.2})", result.text, result.score);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Architecture
//!
//! ARCADIA is built on three core frameworks:
//!
//! ### VIVIAN (Vector Index Virtual Infrastructure)
//!
//! Provides efficient vector-based storage and retrieval for game data:
//! - Semantic search across game elements
//! - High-dimensional data indexing with Qdrant
//! - Real-time similarity matching
//! - In-memory and persistent storage modes
//!
//! ### PARIS (Perpetual Adaptive Regenerative Intelligence System)
//!
//! Enables continuous learning and optimization:
//! - Multi-layer neural architecture
//! - Regenerative feedback loops
//! - Adaptive optimization strategies
//! - Self-improving AI models
//!
//! ### aiTOML Workflow Specification
//!
//! Flexible workflow definition system:
//! - TOML-based configuration
//! - Autonomous AI infrastructure
//! - Secure key management
//! - Multi-language support
//!
//! ## Core Concepts
//!
//! ### Code DNA
//!
//! Define the fundamental attributes of your game world:
//!
//! ```rust
//! use arcadia::code_dna::CodeDNA;
//!
//! let dna = CodeDNA {
//!     theme: "cyberpunk".to_string(),
//!     time_scale: 1.0,
//!     entropy_rate: 0.1,
//!     physical_laws: vec!["gravity".to_string(), "cybernetics".to_string()],
//!     ..Default::default()
//! };
//! ```
//!
//! ### Vector Index
//!
//! Store and search game data semantically:
//!
//! ```rust,no_run
//! # use arcadia::vector_index::{VectorIndex, VectorIndexConfig};
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! # let config = VectorIndexConfig::default();
//! # let index = VectorIndex::new(config).await?;
//! // Store an entity
//! let id = index.store(
//!     Some("npc_trader".to_string()),
//!     "Friendly space trader selling rare artifacts",
//!     Default::default(),
//! ).await?;
//!
//! // Search semantically
//! let results = index.search("Where can I buy items?", 5).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Performance Features
//!
//! ARCADIA is optimized for high-performance gaming:
//!
//! - **Zero-copy operations** where possible
//! - **SIMD-accelerated** vector computations
//! - **Memory pooling** for reduced allocations
//! - **Lock-free** concurrent data structures (DashMap, parking_lot)
//! - **Async/await** for non-blocking I/O
//! - **Embedding cache** for 10-100x speedup on repeated queries
//!
//! ## Examples
//!
//! ARCADIA includes comprehensive examples in the `examples/` directory:
//!
//! - `basic_game` - Simple game setup with vector index and caching
//! - `ai_npc` - Emotionally intelligent NPC with adaptive behavior
//! - `npc_ai_example` - Advanced NPC decision-making system
//!
//! Run examples with:
//!
//! ```bash
//! cargo run --example basic_game
//! ```
//!
//! ## Modules
//!
//! - [`vector_index`] - Vector-based storage and semantic search
//! - [`code_dna`] - Game world DNA and procedural generation
//! - [`authentication`] - OAuth2 and JWT authentication
//! - [`cache`] - High-performance caching system
//! - [`memory`] - Memory management and pooling
//! - [`metrics`] - Performance metrics and monitoring
//!
//! ## Feature Flags
//!
//! ARCADIA supports the following feature flags:
//!
//! - `wasm` - Enable WebAssembly support (enabled by default)
//! - `qdrant` - Enable Qdrant vector database integration (enabled by default)
//! - `auth` - Enable authentication features (enabled by default)
//!
//! ## Requirements
//!
//! - Rust 1.75 or later
//! - OpenAI API key (for embeddings)
//! - Optional: Qdrant instance for distributed vector storage
//! - Optional: PostgreSQL/SQLite for persistent storage
//!
//! ## Configuration
//!
//! Set required environment variables:
//!
//! ```bash
//! export OPENAI_API_KEY="your-api-key"
//! export QDRANT_URL="http://localhost:6333"  # Optional
//! ```
//!
//! Or use a `config.toml` file. See `config.example.toml` for details.
//!
//! ## License
//!
//! Licensed under either of Apache License, Version 2.0 or MIT license at your option.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs, rust_2018_idioms)]
#![allow(clippy::too_many_arguments)]

// Core modules
pub mod code_dna;
pub mod vector_index;
pub mod authentication;

// Performance and optimization modules
pub mod cache;
pub mod memory;
pub mod metrics;

// AgentDB integration for persistent learning and memory
pub mod agentdb;

// Re-export commonly used types
pub use code_dna::{CodeDNA, GameWorld};
pub use vector_index::{VectorIndex, VectorIndexConfig};
pub use authentication::{Authentication, AuthenticationConfig, Credentials};

/// Performance-critical constants
pub const DEFAULT_CACHE_SIZE: usize = 256; // MB
pub const MAX_AI_THREADS: usize = 8;
pub const EMBEDDING_DIM: usize = 1536;
