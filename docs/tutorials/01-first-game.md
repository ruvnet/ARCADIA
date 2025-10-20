# Tutorial: Creating Your First Game with ARCADIA

This tutorial will guide you through creating a simple text-based adventure game using ARCADIA's AI-driven features.

## What You'll Build

A text adventure game with:
- AI-powered NPCs with personalities
- Semantic search for game world queries
- Dynamic quest generation
- Player action memory

## Prerequisites

- Rust installed (1.70+)
- OpenAI API key
- Basic Rust knowledge
- 30 minutes of your time

## Step 1: Project Setup

Create a new Rust project:

```bash
cargo new my_arcadia_game
cd my_arcadia_game
```

Add ARCADIA to `Cargo.toml`:

```toml
[dependencies]
arcadia = "0.1.0"
tokio = { version = "1.40", features = ["full"] }
anyhow = "1.0"
```

Set your API key:

```bash
export OPENAI_API_KEY="sk-your-key-here"
```

## Step 2: Initialize the Game World

Create `src/main.rs`:

```rust
use arcadia::code_dna::CodeDNA;
use arcadia::vector_index::{VectorIndex, VectorIndexConfig};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize ARCADIA
    println!("🎮 Welcome to ARCADIA Adventure!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Create game world DNA
    let world_dna = CodeDNA::new(
        "Medieval Fantasy Kingdom",
        "Magic and Sorcery",
        &vec!["Magic".to_string(), "Medieval Physics".to_string()],
        &vec!["Dragon Threat".to_string(), "Lost Artifact".to_string()],
        1.0,  // Normal time scale
        0.05, // Low entropy (stable world)
        &vec!["Natural Selection".to_string()],
    );

    println!("📜 World: {}", world_dna.setting);
    println!("⚔️  Technology: {}", world_dna.technology);
    println!();

    // Initialize vector index for game state
    let config = VectorIndexConfig {
        api_key: std::env::var("OPENAI_API_KEY")?,
        collection_name: "adventure_game".to_string(),
        ..Default::default()
    };

    let mut index = VectorIndex::new(config).await?;

    Ok(())
}
```

Run the program:

```bash
cargo run
```

You should see the game world initialized!

## Step 3: Add Game Locations

Add locations to the game world:

```rust
async fn setup_world(index: &VectorIndex) -> anyhow::Result<()> {
    println!("🗺️  Setting up game world...\n");

    let locations = vec![
        (
            "village_square",
            "The village square bustles with merchants and travelers. \
             A fountain sits in the center, and the tavern door is open."
        ),
        (
            "dark_forest",
            "Dense trees block most sunlight. Strange sounds echo in the distance. \
             A narrow path leads deeper into the woods."
        ),
        (
            "ancient_ruins",
            "Crumbling stone structures covered in moss. Ancient runes glow faintly. \
             A sealed door bears mysterious symbols."
        ),
        (
            "dragon_lair",
            "A massive cave entrance with scorched earth around it. \
             The smell of sulfur is overwhelming. Treasure glints in the darkness."
        ),
    ];

    for (id, description) in locations {
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "location".to_string());
        metadata.insert("id".to_string(), id.to_string());

        index.store(Some(id.to_string()), description, metadata).await?;
        println!("  ✓ Added: {}", id.replace('_', " "));
    }

    println!();
    Ok(())
}
```

Call it from main:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ... previous code ...

    setup_world(&index).await?;

    Ok(())
}
```

## Step 4: Create NPCs

Add NPCs with personalities:

```rust
async fn create_npcs(index: &VectorIndex) -> anyhow::Result<()> {
    println!("👥 Creating NPCs...\n");

    let npcs = vec![
        (
            "merchant_elena",
            "Elena is a friendly merchant who sells potions and magical items. \
             She knows many secrets about the kingdom and loves to gossip."
        ),
        (
            "wizard_aldric",
            "Aldric is an ancient wizard with knowledge of forgotten spells. \
             He is wise but cryptic, often speaking in riddles."
        ),
        (
            "guard_captain",
            "The captain of the guard is a stern but honorable warrior. \
             He knows about threats to the kingdom and coordinates defenses."
        ),
    ];

    for (id, description) in npcs {
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "npc".to_string());
        metadata.insert("id".to_string(), id.to_string());

        index.store(Some(id.to_string()), description, metadata).await?;
        println!("  ✓ Created: {}", id.replace('_', " "));
    }

    println!();
    Ok(())
}
```

## Step 5: Implement Player Commands

Add a command system:

```rust
async fn process_command(
    command: &str,
    index: &VectorIndex,
) -> anyhow::Result<()> {
    println!("\n🔍 Processing: {}", command);

    // Use semantic search to find relevant game elements
    let results = index.search(command, 3).await?;

    println!("\nRelevant game elements:");
    for (i, result) in results.iter().enumerate() {
        println!("{}. {} (relevance: {:.2})", i + 1, result.text, result.score);
    }

    Ok(())
}
```

## Step 6: Create the Game Loop

Add an interactive game loop:

```rust
use std::io::{self, Write};

async fn game_loop(index: VectorIndex) -> anyhow::Result<()> {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎮 Game Started!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\nType commands like:");
    println!("  - 'where can I buy potions?'");
    println!("  - 'who knows about the dragon?'");
    println!("  - 'explore the forest'");
    println!("  - 'quit' to exit\n");

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let command = input.trim();

        if command.is_empty() {
            continue;
        }

        if command.eq_ignore_ascii_case("quit") {
            println!("\n👋 Thanks for playing!");
            break;
        }

        process_command(command, &index).await?;
    }

    Ok(())
}
```

Update main to use the game loop:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ... previous initialization ...

    setup_world(&index).await?;
    create_npcs(&index).await?;

    // Start game loop
    game_loop(index).await?;

    Ok(())
}
```

## Step 7: Run Your Game!

```bash
cargo run
```

Try these commands:
- "where can I find potions?"
- "who can help me with magic?"
- "tell me about dangerous places"
- "where should I look for treasure?"

## Complete Code

Here's the full `src/main.rs`:

```rust
use arcadia::code_dna::CodeDNA;
use arcadia::vector_index::{VectorIndex, VectorIndexConfig};
use std::collections::HashMap;
use std::io::{self, Write};

async fn setup_world(index: &VectorIndex) -> anyhow::Result<()> {
    println!("🗺️  Setting up game world...\n");

    let locations = vec![
        ("village_square", "The village square bustles with merchants..."),
        ("dark_forest", "Dense trees block most sunlight..."),
        ("ancient_ruins", "Crumbling stone structures..."),
        ("dragon_lair", "A massive cave entrance..."),
    ];

    for (id, description) in locations {
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "location".to_string());
        metadata.insert("id".to_string(), id.to_string());

        index.store(Some(id.to_string()), description, metadata).await?;
        println!("  ✓ Added: {}", id.replace('_', " "));
    }

    println!();
    Ok(())
}

async fn create_npcs(index: &VectorIndex) -> anyhow::Result<()> {
    println!("👥 Creating NPCs...\n");

    let npcs = vec![
        ("merchant_elena", "Elena is a friendly merchant..."),
        ("wizard_aldric", "Aldric is an ancient wizard..."),
        ("guard_captain", "The captain of the guard..."),
    ];

    for (id, description) in npcs {
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "npc".to_string());
        metadata.insert("id".to_string(), id.to_string());

        index.store(Some(id.to_string()), description, metadata).await?;
        println!("  ✓ Created: {}", id.replace('_', " "));
    }

    println!();
    Ok(())
}

async fn process_command(command: &str, index: &VectorIndex) -> anyhow::Result<()> {
    println!("\n🔍 Processing: {}", command);

    let results = index.search(command, 3).await?;

    println!("\nRelevant game elements:");
    for (i, result) in results.iter().enumerate() {
        println!("{}. {} (relevance: {:.2})", i + 1, result.text, result.score);
    }

    Ok(())
}

async fn game_loop(index: VectorIndex) -> anyhow::Result<()> {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎮 Game Started!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let command = input.trim();

        if command.is_empty() {
            continue;
        }

        if command.eq_ignore_ascii_case("quit") {
            println!("\n👋 Thanks for playing!");
            break;
        }

        process_command(command, &index).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🎮 Welcome to ARCADIA Adventure!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let world_dna = CodeDNA::new(
        "Medieval Fantasy Kingdom",
        "Magic and Sorcery",
        &vec!["Magic".to_string()],
        &vec!["Dragon Threat".to_string()],
        1.0,
        0.05,
        &vec!["Natural Selection".to_string()],
    );

    println!("📜 World: {}", world_dna.setting);
    println!("⚔️  Technology: {}", world_dna.technology);
    println!();

    let config = VectorIndexConfig {
        api_key: std::env::var("OPENAI_API_KEY")?,
        collection_name: "adventure_game".to_string(),
        ..Default::default()
    };

    let index = VectorIndex::new(config).await?;

    setup_world(&index).await?;
    create_npcs(&index).await?;
    game_loop(index).await?;

    Ok(())
}
```

## Next Steps

Now that you have a basic game, you can:

1. **Add Items**: Create weapons, potions, and artifacts
2. **Implement Combat**: Use AI for dynamic enemy behavior
3. **Add Quests**: Generate procedural quests based on world state
4. **Save/Load**: Persist game state to disk
5. **Multiplayer**: Add other players using the multiplayer module

## Advanced Topics

- [AI NPC Tutorial](03-ai-npcs.md)
- [Procedural Generation](04-procedural-world.md)
- [Performance Optimization](07-optimization.md)

## Troubleshooting

**Problem**: "OpenAI API key not set"
- **Solution**: Set `export OPENAI_API_KEY="sk-..."`

**Problem**: Slow responses
- **Solution**: Embeddings are cached automatically after first use

**Problem**: Too many API calls
- **Solution**: Increase cache TTL in configuration

## Resources

- [ARCADIA Documentation](../README.md)
- [API Reference](../api/)
- [More Examples](../../examples/)

Congratulations! You've built your first ARCADIA game! 🎉
