//! AI NPC Example
//!
//! This example demonstrates creating intelligent NPCs with:
//! - Personality definitions using CodeDNA
//! - Contextual dialogue using vector search
//! - Memory of past interactions
//! - Emotion-adaptive responses

use arcadia::{
    code_dna::CodeDNA,
    vector_index::{VectorIndex, VectorIndexConfig},
    metrics,
};
use std::collections::HashMap;
use std::time::Instant;

/// NPC with AI-driven personality
struct AINPC {
    name: String,
    personality: String,
    conversation_history: Vec<String>,
    vector_index: VectorIndex,
}

impl AINPC {
    async fn new(
        name: String,
        personality: String,
        api_key: String,
    ) -> anyhow::Result<Self> {
        let config = VectorIndexConfig {
            api_key,
            collection_name: format!("npc_{}", name.to_lowercase()),
            ..Default::default()
        };

        let vector_index = VectorIndex::new(config).await?;

        Ok(Self {
            name,
            personality,
            conversation_history: Vec::new(),
            vector_index,
        })
    }

    /// Store a conversation exchange
    async fn remember_conversation(
        &mut self,
        player_input: &str,
        npc_response: &str,
    ) -> anyhow::Result<()> {
        let conversation = format!(
            "Player: {} | NPC: {}",
            player_input, npc_response
        );

        let metadata: HashMap<String, String> = [
            ("type".to_string(), "conversation".to_string()),
            ("player_input".to_string(), player_input.to_string()),
            ("npc_response".to_string(), npc_response.to_string()),
        ]
        .iter()
        .cloned()
        .collect();

        self.vector_index
            .store(None, &conversation, metadata)
            .await?;

        self.conversation_history.push(conversation);

        Ok(())
    }

    /// Generate response based on context and past conversations
    async fn respond(&mut self, player_input: &str) -> anyhow::Result<String> {
        println!("\n💭 {} is thinking...", self.name);

        let start = Instant::now();

        // Find similar past conversations
        let similar = self.vector_index.search(player_input, 3).await?;

        let duration = start.elapsed();
        metrics::ai_metrics::record_decision(duration, "npc_response");

        println!("   Found {} similar past conversations", similar.len());

        // Generate context-aware response
        let response = if similar.is_empty() {
            // First-time interaction
            format!(
                "Hello! I'm {}, {}. How can I help you?",
                self.name, self.personality
            )
        } else {
            // Use past conversations for context
            let context: Vec<&str> = similar
                .iter()
                .map(|r| r.text.as_str())
                .collect();

            self.generate_contextual_response(player_input, &context)
        };

        // Remember this conversation
        self.remember_conversation(player_input, &response).await?;

        println!("   Response generated in {}ms", duration.as_millis());

        Ok(response)
    }

    /// Generate a contextual response based on similar conversations
    fn generate_contextual_response(
        &self,
        input: &str,
        context: &[&str],
    ) -> String {
        // Simplified response generation
        // In production, this would use GPT-4 or similar

        if input.to_lowercase().contains("buy") ||
           input.to_lowercase().contains("sell") {
            format!(
                "As {}, I have some interesting items for trade. \
                 Based on our past conversations, you might be interested \
                 in rare artifacts.",
                self.personality
            )
        } else if input.to_lowercase().contains("quest") ||
                  input.to_lowercase().contains("help") {
            format!(
                "I remember you asking about this before. Let me think... \
                 I know of a dangerous mission in the asteroid belt."
            )
        } else if input.to_lowercase().contains("story") ||
                  input.to_lowercase().contains("tell") {
            format!(
                "Ah, you want to hear a story? Well, being {}, \
                 I've seen many strange things in this galaxy...",
                self.personality
            )
        } else {
            format!(
                "Interesting question. As {}, I think we should \
                 consider past events before deciding.",
                self.personality
            )
        }
    }

    /// Get conversation statistics
    fn get_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        stats.insert("total_conversations".to_string(), self.conversation_history.len());
        stats
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("arcadia=info,ai_npc=debug")
        .init();

    println!("🤖 ARCADIA AI NPC Example\n");

    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY must be set");

    // Create NPCs with different personalities
    println!("Creating NPCs...");

    let mut trader = AINPC::new(
        "Zara".to_string(),
        "a friendly space trader who loves rare artifacts".to_string(),
        api_key.clone(),
    )
    .await?;

    let mut guard = AINPC::new(
        "Kane".to_string(),
        "a stern security guard focused on station safety".to_string(),
        api_key.clone(),
    )
    .await?;

    println!("✅ NPCs created\n");

    // Simulate conversation with trader
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 Conversation with Trader (Zara)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let trader_interactions = vec![
        "Hello, do you have anything for sale?",
        "What kind of artifacts do you have?",
        "I'm looking to buy some rare items",
        "Do you have any quests for me?",
    ];

    for input in &trader_interactions {
        println!("Player: {}", input);
        let response = trader.respond(input).await?;
        println!("{}: {}\n", trader.name, response);
    }

    // Simulate conversation with guard
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 Conversation with Guard (Kane)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let guard_interactions = vec![
        "Is this area safe?",
        "Can you help me with a security problem?",
        "Tell me about the station's defenses",
    ];

    for input in &guard_interactions {
        println!("Player: {}", input);
        let response = guard.respond(input).await?;
        println!("{}: {}\n", guard.name, response);
    }

    // Show statistics
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 Statistics");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("Zara (Trader):");
    for (key, value) in trader.get_stats() {
        println!("  {}: {}", key, value);
    }

    println!("\nKane (Guard):");
    for (key, value) in guard.get_stats() {
        println!("  {}: {}", key, value);
    }

    println!("\n✨ Example completed successfully!");

    Ok(())
}
