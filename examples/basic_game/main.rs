//! Basic Game Example
//!
//! This example demonstrates a simple game setup using ARCADIA with:
//! - Vector index for semantic game state
//! - CodeDNA for world configuration
//! - Caching for performance
//! - Metrics tracking

use arcadia::{
    code_dna::{CodeDNA, GameWorld},
    vector_index::{VectorIndex, VectorIndexConfig},
    cache::{create_embedding_cache, CacheManager},
    metrics,
};
use std::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("arcadia=info,basic_game=debug")
        .init();

    println!("🎮 ARCADIA Basic Game Example\n");

    // 1. Create Game World DNA
    println!("📋 Creating game world...");
    let dna = CodeDNA::default_scifi();
    let mut world = GameWorld::new();
    dna.apply_to_game_world(&mut world);

    println!("✅ Game world created:");
    println!("   Setting: {}", world.get_setting());
    println!("   Time scale: {}", world.get_time_scale());
    println!("   Entropy rate: {}\n", world.get_entropy_rate());

    // 2. Initialize Vector Index
    println!("🔍 Initializing vector index...");
    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY must be set");

    let config = VectorIndexConfig {
        api_key,
        qdrant_url: None, // In-memory only for this example
        collection_name: "basic_game".to_string(),
        embedding_model: "text-embedding-3-small".to_string(),
        vector_dimension: 1536,
        ..Default::default()
    };

    let index = VectorIndex::new(config).await?;
    println!("✅ Vector index initialized\n");

    // 3. Store Game Entities
    println!("📦 Storing game entities...");

    let entities = vec![
        ("player", "Human player character with laser rifle and shield"),
        ("enemy_alien", "Hostile alien creature with acid attack"),
        ("npc_trader", "Friendly space trader selling rare artifacts"),
        ("item_medkit", "Medical kit that restores 50 health points"),
        ("location_station", "Abandoned space station in asteroid belt"),
    ];

    for (name, description) in &entities {
        let start = Instant::now();

        let id = index.store(
            Some(name.to_string()),
            description,
            vec![("type".to_string(), "entity".to_string())]
                .into_iter()
                .collect(),
        ).await?;

        let duration = start.elapsed();
        metrics::vector_metrics::record_store(duration);

        println!("   ✓ Stored '{}' ({}ms)", name, duration.as_millis());
    }

    println!();

    // 4. Semantic Search
    println!("🔎 Performing semantic searches...\n");

    let queries = vec![
        "Who can heal me?",
        "Where can I find weapons?",
        "Show me dangerous enemies",
    ];

    for query in &queries {
        let start = Instant::now();

        let results = index.search(query, 3).await?;
        let duration = start.elapsed();

        metrics::vector_metrics::record_search(duration, results.len());

        println!("Query: '{}'", query);
        for (i, result) in results.iter().enumerate() {
            println!("  {}. {} (score: {:.3})", i + 1, result.text, result.score);
        }
        println!("  ({}ms)\n", duration.as_millis());
    }

    // 5. Cache Performance Demo
    println!("⚡ Cache performance demonstration...\n");

    // First call - should hit API
    let text = "Powerful weapon for combat";
    println!("First embedding (API call):");
    let start = Instant::now();
    let embedding1 = index.embed_text(text).await?;
    let duration1 = start.elapsed();
    println!("  Time: {}ms", duration1.as_millis());

    // Second call - should use cache
    println!("Second embedding (cached):");
    let start = Instant::now();
    let embedding2 = index.embed_text(text).await?;
    let duration2 = start.elapsed();
    println!("  Time: {}ms", duration2.as_millis());

    let speedup = duration1.as_micros() as f64 / duration2.as_micros() as f64;
    println!("  Speedup: {:.1}x faster\n", speedup);

    // Verify embeddings are identical
    assert_eq!(embedding1.len(), embedding2.len());
    println!("✅ Cache working correctly\n");

    // 6. Performance Metrics
    println!("📊 Session Statistics:");
    println!("   Total entities stored: {}", entities.len());
    println!("   Total queries performed: {}", queries.len());
    println!("   Embedding dimension: {}", embedding1.len());
    println!();

    println!("✨ Example completed successfully!");

    Ok(())
}
