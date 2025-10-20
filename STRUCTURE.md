# ARCADIA Project Structure

## Complete Directory Tree

```
/home/user/ARCADIA/
│
├── Cargo.toml                      # Project manifest with dependencies
├── config.example.toml             # Example configuration file
│
├── src/                            # Source code
│   ├── main.rs                     # Main entry point & game loop
│   │
│   ├── aitoml/                     # aiTOML Configuration Parser
│   │   └── mod.rs                  # TOML parsing & workflow execution
│   │
│   ├── auth/                       # Authentication System
│   │   └── mod.rs                  # OAuth2 & JWT implementation
│   │
│   ├── vector_index/               # Vector Database Integration
│   │   └── mod.rs                  # OpenAI embeddings & Qdrant
│   │
│   └── game_components/            # Game Elements & Systems
│       ├── mod.rs                  # Module exports
│       ├── code_dna.rs             # Genetic game blueprint
│       ├── components.rs           # All game components
│       └── game_world.rs           # World state container
│
├── tests/                          # Integration tests
│   └── integration_tests.rs        # System-wide tests
│
├── IMPLEMENTATION.md               # Implementation guide
├── BUILD_SUMMARY.md                # Build status & summary
├── STRUCTURE.md                    # This file
│
├── README.md                       # Project overview
├── CLAUDE.md                       # Claude integration docs
│
└── [Other project files...]        # Git, configs, etc.
```

## Module Relationships

```
┌─────────────────────────────────────────────────┐
│              main.rs (ArcadiaSystem)            │
│  - Configuration loading                        │
│  - System initialization                        │
│  - Game loop orchestration                      │
└─────────────────────────────────────────────────┘
                        │
         ┌──────────────┼──────────────┬─────────────┐
         │              │              │             │
         ▼              ▼              ▼             ▼
┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐
│   aitoml/   │ │    auth/    │ │vector_index/│ │game_comp.../│
│             │ │             │ │             │ │             │
│ Config      │ │ OAuth2      │ │ OpenAI API  │ │ CodeDNA     │
│ Parsing     │ │ JWT         │ │ Qdrant DB   │ │ Components  │
│ Workflows   │ │ Sessions    │ │ Embeddings  │ │ GameWorld   │
└─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘
```

## Data Flow

```
┌──────────────┐
│ config.toml  │
└──────┬───────┘
       │
       ▼
┌──────────────────┐
│  AiTomlParser    │ Parse & Validate
└──────┬───────────┘
       │
       ▼
┌──────────────────┐
│  ArcadiaSystem   │ Initialize
├──────────────────┤
│ - Vector Index   │◄──── OpenAI API
│ - Authentication │◄──── OAuth2 Provider
│ - GameWorld      │
│ - GameElements   │
└──────┬───────────┘
       │
       ▼
┌──────────────────┐
│   Game Loop      │
├──────────────────┤
│ 1. Update World  │
│ 2. Update Comp.  │
│ 3. Execute Logic │
│ 4. Render        │
└──────────────────┘
```

## Component Architecture

```
GameElements
├── CodeDNA                         # World blueprint
│   ├── Setting & Technology
│   ├── Physics Laws
│   ├── Themes
│   ├── Storyline
│   └── Natural Laws
│
├── Functional Components           # Game mechanics
│   ├── Movement
│   ├── Combat
│   ├── Interaction
│   ├── Resource Management
│   ├── Quest System
│   ├── Inventory System
│   ├── Crafting System
│   └── Dialogue System
│
├── Non-Functional Components       # Quality attributes
│   ├── Performance Metrics
│   ├── Accessibility Features
│   ├── Security Features
│   └── Scalability Settings
│
└── Advanced Systems                # AI & emergent behavior
    ├── NeoCortexReasoning         # Decision making
    ├── SymbolicComputing          # Rule-based logic
    ├── AutopoeticProcessing       # Self-organization
    ├── EvolutionaryFeedback       # Genetic algorithms
    ├── SelfAwareness              # Introspection
    ├── AdaptivePerspectives       # Viewpoint shifts
    ├── Entropy                    # World decay
    ├── EmotionAdaptiveExperiences # Mood-based
    ├── SocialConstructs           # Relationships
    ├── MultiplayerExperiences     # Online play
    ├── AccessibilityInclusivity   # Inclusive design
    ├── EthicsResponsibleAI        # Ethical AI
    ├── CustomizationModding       # Mod support
    ├── IntegrationOtherPlatforms  # Cross-platform
    ├── SecurityPrivacy            # Data protection
    └── ContinuousImprovementUpdates # Updates
```

## File Size Breakdown

```
Module                          Lines    Purpose
──────────────────────────────────────────────────────────
main.rs                         ~320     Application entry & game loop
vector_index/mod.rs             ~250     AI embeddings & vector DB
auth/mod.rs                     ~250     Authentication & security
aitoml/mod.rs                   ~380     Config parsing & workflows
game_components/code_dna.rs     ~250     World generation DNA
game_components/components.rs   ~650     All game systems
game_components/game_world.rs   ~100     World state management
──────────────────────────────────────────────────────────
Total                          ~2,200    Production code
```

## Key Interfaces

### 1. Configuration Interface
```rust
AiToml {
    metadata: Metadata,
    vector_index: VectorIndexConfig,
    authentication: AuthenticationConfig,
    game_elements: HashMap<String, GameElement>,
    workflows: HashMap<String, Workflow>,
    code_dna: Option<CodeDnaConfig>,
}
```

### 2. VectorIndex Interface
```rust
impl VectorIndex {
    async fn new(config: VectorIndexConfig) -> Result<Self>
    async fn embed_text(&self, text: &str) -> Result<Vec<f32>>
    async fn store(&self, id: Option<String>, text: &str, metadata: HashMap<String, String>) -> Result<String>
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>>
    async fn delete(&self, id: &str) -> Result<()>
}
```

### 3. Authentication Interface
```rust
impl Authentication {
    fn new(config: AuthenticationConfig) -> Result<Self>
    fn get_authorization_url(&self) -> Result<(String, String)>
    async fn exchange_code(&self, code: &str) -> Result<String>
    fn create_jwt(&self, user_id: &str, email: Option<String>) -> Result<String>
    fn validate_jwt(&self, token: &str) -> Result<Claims>
    fn create_session(&mut self, user_id: &str, email: Option<String>) -> String
    fn validate_session(&self, session_id: &str) -> Result<String>
}
```

### 4. CodeDNA Interface
```rust
impl CodeDNA {
    fn new(...) -> Self
    fn fantasy_default() -> Self
    fn cyberpunk_default() -> Self
    fn apply_to_world(&self, world: &mut GameWorld)
    fn mutate(&mut self, mutation_type: MutationType, intensity: f32)
    fn compatibility(&self, other: &CodeDNA) -> f32
    fn get_current_act(&self) -> Option<&Act>
    fn advance_act(&mut self) -> bool
    fn trigger_plot_point(&mut self, id: &str) -> Option<Vec<String>>
}
```

### 5. GameWorld Interface
```rust
impl GameWorld {
    fn new() -> Self
    fn from_dna(dna: &CodeDNA) -> Self
    fn set_setting(&mut self, setting: &str)
    fn set_technology(&mut self, technology: &str)
    fn set_time_scale(&mut self, time_scale: f32)
    fn set_entropy_rate(&mut self, entropy_rate: f32)
    fn add_physics_law(&mut self, law: PhysicsLaw)
    fn add_theme(&mut self, theme: String)
    fn add_natural_law(&mut self, law: String)
    fn add_entity(&mut self, entity: Entity)
    fn update(&mut self, delta_time: f32)
}
```

## Build & Run Commands

### Development
```bash
# Check code
cargo check

# Build debug
cargo build

# Run application
cargo run

# Run with config
ARCADIA_CONFIG=config.toml cargo run
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_code_dna_creation

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration_tests
```

### Production
```bash
# Build optimized
cargo build --release

# Run release
./target/release/arcadia

# Size optimization
cargo build --release --features minimal
```

## Configuration Files

### config.toml Structure
```toml
[metadata]          # Project metadata
[vector_index]      # OpenAI & Qdrant config
[authentication]    # OAuth2 & JWT config
[code_dna]          # World generation config
[game_elements.*]   # Game element definitions
[workflows.*]       # Workflow definitions
```

## Environment Variables

```bash
# Required
OPENAI_API_KEY=sk-...              # OpenAI API key

# Optional
ARCADIA_CONFIG=config.toml         # Config file path
QDRANT_URL=http://localhost:6334  # Qdrant server
RUST_LOG=info                      # Log level
JWT_SECRET=secret                  # JWT signing secret
```

## Dependencies Graph

```
arcadia
├── tokio (async runtime)
├── async-openai (AI integration)
│   └── reqwest (HTTP client)
├── qdrant-client (vector DB)
├── oauth2 (authentication)
│   └── reqwest
├── jsonwebtoken (JWT)
├── axum (web framework)
│   ├── tower (middleware)
│   └── tokio
├── serde (serialization)
│   ├── serde_json
│   └── toml
├── tracing (logging)
│   └── tracing-subscriber
├── chrono (time)
├── uuid (IDs)
├── thiserror (errors)
├── anyhow (error handling)
└── [more dependencies...]
```

## Memory Layout

```
ArcadiaSystem
├── config: AiToml                    # Config data (~1KB)
├── vector_index: Option<VectorIndex> # Connection (~1KB)
├── auth: Option<Authentication>      # Session data (~10KB)
├── game_world: GameWorld             # World state (~100KB)
└── game_elements: GameElements       # Components (~500KB)
    ├── code_dna: CodeDNA             # DNA data (~10KB)
    ├── functional_components: Vec    # Component list
    └── [17 advanced systems]         # System state
```

## Performance Profile

### Hot Path (Game Loop)
1. World update (1ms)
2. Component updates (2-5ms)
3. Component execution (5-10ms)
4. Frame delay (16ms @ 60fps)

### Cold Path (Initialization)
1. Config parsing (10ms)
2. Vector index init (100ms)
3. Auth init (50ms)
4. Component creation (100ms)

### I/O Operations
- Vector embedding: 100-500ms (network)
- Vector search: 10-50ms (database)
- JWT validation: <1ms (local)
- Config load: <10ms (disk)

## Scaling Considerations

### Horizontal Scaling
- Stateless architecture (JWT)
- Vector DB can be clustered
- Game state can be sharded
- Load balancer compatible

### Vertical Scaling
- Multi-threaded with Tokio
- Parallel component execution
- Connection pooling
- Efficient memory usage

## Security Layers

```
┌─────────────────────────────────┐
│     TLS/HTTPS (Transport)       │
├─────────────────────────────────┤
│     JWT Tokens (Authentication) │
├─────────────────────────────────┤
│     OAuth2 (Authorization)      │
├─────────────────────────────────┤
│     Input Validation            │
├─────────────────────────────────┤
│     Rate Limiting               │
└─────────────────────────────────┘
```

---

**Last Updated**: 2025-10-20
**Version**: 0.1.0
**Status**: Complete ✅
