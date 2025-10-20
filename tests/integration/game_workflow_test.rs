//! Integration tests for complete game workflows

use arcadia::*;

#[test]
fn test_complete_game_initialization() {
    // Create CodeDNA for a sci-fi game
    let code_dna = code_dna::CodeDNA::default_scifi();
    assert!(code_dna.validate().is_ok());

    // Initialize game elements
    let game_elements = game_elements::GameElements::new(code_dna.clone());
    assert_eq!(game_elements.component_count(), 0);

    // Verify all systems are initialized
    assert_eq!(game_elements.neo_cortex_reasoning.decision_count(), 0);
    assert_eq!(game_elements.evolutionary_feedback.generation_count(), 0);
}

#[test]
fn test_vector_index_integration() {
    let config = vector_index::VectorIndexConfig::new(
        "https://example.com".to_string(),
        "test_api_key_12345".to_string(),
    );

    let mut index = vector_index::VectorIndex::new(config);

    // Create and insert vectors
    let vec1 = vector_index::Vector::new("character1".to_string(), vec![1.0, 0.0, 0.0])
        .with_metadata("type".to_string(), "hero".to_string());
    let vec2 = vector_index::Vector::new("character2".to_string(), vec![0.0, 1.0, 0.0])
        .with_metadata("type".to_string(), "villain".to_string());

    assert!(index.insert(vec1).is_ok());
    assert!(index.insert(vec2).is_ok());

    // Search for similar vectors
    let query = vector_index::Vector::new("query".to_string(), vec![1.0, 0.0, 0.0]);
    let results = index.search(&query, 2).unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].0, "character1");
}

#[test]
fn test_authentication_workflow() {
    let credentials = authentication::Credentials::new(
        "test_client_id".to_string(),
        "test_client_secret".to_string(),
    );

    let config = authentication::AuthenticationConfig::new("oauth2".to_string(), credentials);

    let mut auth = authentication::Authentication::new(config);

    // Authenticate user
    let token = auth.authenticate("user1", "password123").unwrap();
    assert!(!token.is_empty());

    // Validate token
    let user_id = auth.validate_token(&token).unwrap();
    assert_eq!(user_id, "user1");

    // Revoke token
    assert!(auth.revoke_token(&token).is_ok());
    assert!(auth.validate_token(&token).is_err());
}

#[test]
fn test_ai_systems_integration() {
    // Test Neo-Cortex reasoning
    let mut cortex = ai_systems::NeoCortexReasoning::new();
    cortex.learn("combat_situation", 0.8);
    let decision = cortex.make_decision("combat_situation");
    assert_eq!(decision, 0.8);

    // Test Evolutionary feedback
    let mut evolution = ai_systems::EvolutionaryFeedback::new(0.1);
    let traits = vec![0.5, 0.6, 0.7];
    let new_traits = evolution.evolve(traits.clone(), 0.9);
    assert_eq!(new_traits.len(), 3);

    // Test Self-awareness
    let mut awareness = ai_systems::SelfAwareness::new("NPC".to_string(), "Guard".to_string());
    awareness.record_experience("Player approached".to_string(), 0.3);
    assert_eq!(awareness.experience_count(), 1);

    // Test Adaptive perspectives
    let mut perspectives = ai_systems::AdaptivePerspectives::new();
    perspectives.adapt("defensive", 0.7);
    perspectives.adapt("aggressive", 0.9);
    assert_eq!(perspectives.get_current_strategy(), "aggressive");

    // Test Emotion-adaptive
    let mut emotions = ai_systems::EmotionAdaptiveExperiences::new();
    emotions.set_emotion(ai_systems::Emotion::Fear, 0.8);
    let modifier = emotions.adapt_environment();
    assert!(modifier.lighting < 0.5);
}

#[test]
fn test_game_element_lifecycle() {
    let dna = code_dna::CodeDNA::default_fantasy();
    let mut game = game_elements::GameElements::new(dna);

    // Add functional component
    let character = game_elements::FunctionalComponent::new(
        "player1".to_string(),
        game_elements::ComponentType::Character,
        (0.0, 0.0, 0.0),
    );

    game.add_component(character);
    assert_eq!(game.component_count(), 1);

    // Update game state
    game.update(1.0);

    // Check entropy system
    game.entropy.add_object("sword".to_string(), 1.0);
    game.entropy.update(10.0);
    let durability = game.entropy.get_durability("sword").unwrap();
    assert!(durability < 1.0);
}

#[test]
fn test_social_constructs_integration() {
    let dna = code_dna::CodeDNA::default_fantasy();
    let mut game = game_elements::GameElements::new(dna);

    // Add factions
    game.social_constructs.add_faction("Knights".to_string(), 0.8, 0.7);
    game.social_constructs.add_faction("Rogues".to_string(), -0.5, 0.4);

    // Update player reputation
    game.social_constructs.update_reputation("player1".to_string(), "Knights", 0.5);
    game.social_constructs.update_reputation("player1".to_string(), "Rogues", -0.3);

    // Check reputation
    let knights_rep = game.social_constructs.get_reputation("player1", "Knights");
    let rogues_rep = game.social_constructs.get_reputation("player1", "Rogues");

    assert_eq!(knights_rep, 0.5);
    assert_eq!(rogues_rep, -0.3);
}

#[test]
fn test_multi_component_interaction() {
    let dna = code_dna::CodeDNA::default_scifi();
    let mut game = game_elements::GameElements::new(dna);

    // Add multiple components
    for i in 0..5 {
        let component = game_elements::FunctionalComponent::new(
            format!("npc_{}", i),
            game_elements::ComponentType::Character,
            (i as f32, 0.0, 0.0),
        );
        game.add_component(component);
    }

    assert_eq!(game.component_count(), 5);

    // Test autopoetic processing
    for i in 0..5 {
        game.autopoetic_processing.add_component(format!("system_{}", i));
    }

    game.update(5.0);
    let health = game.autopoetic_processing.system_health();
    assert!(health > 0.0 && health <= 1.0);
}
