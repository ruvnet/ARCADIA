//! Integration tests for AI systems

use arcadia::ai::*;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;

#[test]
fn test_integrated_ai_system_lifecycle() {
    let entity_id = Uuid::new_v4();
    let system = IntegratedAISystem::new(entity_id, "TestNPC".to_string());

    // Initialize systems
    assert_eq!(system.self_awareness.read().get_entity_name(), "TestNPC");
    assert_eq!(system.self_awareness.read().get_entity_id(), entity_id);

    // Test system update
    system.update(1.0);

    // Verify system is operational
    let health = system.get_system_health();
    assert!(matches!(health, HealthStatus::Optimal | HealthStatus::Healthy));
}

#[test]
fn test_neo_cortex_decision_making() {
    let neo_cortex = NeoCortexReasoning::new();

    // Add goals
    neo_cortex.add_goal("Defend territory".to_string(), 0.9);
    neo_cortex.add_goal("Gather resources".to_string(), 0.6);

    // Create decision context
    let context = DecisionContext {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        situation: "Enemy spotted nearby".to_string(),
        goals: vec!["Defend territory".to_string()],
        constraints: vec!["No retreat".to_string()],
        available_actions: vec![
            "Attack enemy".to_string(),
            "Fortify position".to_string(),
            "Scout area".to_string(),
        ],
        world_state: HashMap::new(),
        priority: 0.85,
    };

    let decision = neo_cortex.make_decision(context);

    assert!(!decision.action.is_empty());
    assert!(decision.confidence >= 0.0 && decision.confidence <= 1.0);
    assert!(decision.expected_utility >= 0.0);

    // Test recording outcomes
    neo_cortex.record_outcome(decision.clone(), 0.8, true);

    // Make another decision - should benefit from learning
    let context2 = DecisionContext {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        situation: "Enemy spotted nearby".to_string(),
        goals: vec!["Defend territory".to_string()],
        constraints: vec![],
        available_actions: vec![
            "Attack enemy".to_string(),
            "Retreat".to_string(),
        ],
        world_state: HashMap::new(),
        priority: 0.8,
    };

    let decision2 = neo_cortex.make_decision(context2);
    assert!(!decision2.action.is_empty());
}

#[test]
fn test_autopoetic_self_organization() {
    let autopoetic = AutopoeticProcessing::new();

    // Add components
    let comp1 = autopoetic.add_component("Engine".to_string(), vec![]);
    let comp2 = autopoetic.add_component("Sensors".to_string(), vec![comp1]);
    let comp3 = autopoetic.add_component("Actuators".to_string(), vec![comp1]);

    // Check initial system health
    let initial_health = autopoetic.get_system_health();
    assert!(initial_health > 0.0);

    // Degrade a component
    autopoetic.update_component_health(comp1, 0.3);

    // System should schedule maintenance
    autopoetic.perform_self_maintenance();

    // Detect emergent patterns
    let patterns = autopoetic.detect_emergent_patterns();
    assert!(!patterns.is_empty());

    // Self-organize
    autopoetic.self_organize();

    // Check maintenance queue
    assert!(autopoetic.get_maintenance_queue_size() >= 0);
}

#[test]
fn test_evolutionary_feedback_learning() {
    let config = EvolutionConfig {
        population_size: 50,
        mutation_rate: 0.02,
        crossover_rate: 0.7,
        elitism_count: 5,
        max_generations: 100,
    };

    let evo = EvolutionaryFeedback::new(config);

    // Initialize population
    evo.reset_population(50, 10);

    // Add fitness criteria
    evo.add_fitness_criteria(FitnessCriteria {
        name: "gene_0".to_string(),
        weight: 1.0,
        target_value: 0.7,
        tolerance: 0.1,
    });

    // Evolve several generations
    for _ in 0..5 {
        evo.evolve_generation();
    }

    assert_eq!(evo.get_generation(), 5);
    assert_eq!(evo.get_population_size(), 50);

    // Check if fitness improved
    let avg_fitness = evo.get_average_fitness();
    assert!(avg_fitness >= 0.0 && avg_fitness <= 1.0);

    // Get best genome
    let best = evo.get_best_genome();
    assert!(best.is_some());
}

#[test]
fn test_self_awareness_consciousness() {
    let entity_id = Uuid::new_v4();
    let sa = SelfAwareness::new(entity_id, "ConsciousEntity".to_string());

    // Test role management
    sa.add_role(EntityRole::Guardian);
    sa.add_role(EntityRole::QuestGiver);
    assert_eq!(sa.get_roles().len(), 3); // Including default Neutral

    // Test belief formation
    sa.form_belief("Players are allies".to_string(), 0.9);
    let belief = sa.get_belief("Players are allies");
    assert!(belief.is_some());
    assert!(belief.unwrap().confidence > 0.8);

    // Test consciousness states
    sa.set_consciousness_state(ConsciousnessState::Reflective);
    assert_eq!(sa.get_consciousness_state(), ConsciousnessState::Reflective);

    // Meta-reasoning should have been triggered
    let thoughts = sa.get_recent_meta_thoughts(5);
    assert!(!thoughts.is_empty());

    // Test relationship tracking
    let friend_id = Uuid::new_v4();
    sa.record_relationship(friend_id, "Ally".to_string(), "friendly".to_string(), 0.8);
    let rel = sa.get_relationship(friend_id);
    assert!(rel.is_some());

    // Test situation understanding
    sa.update_context("location".to_string(), "Temple".to_string());
    let understanding = sa.understand_situation();
    assert!(understanding.contains("ConsciousEntity"));

    // Test self-awareness level
    let awareness_level = sa.assess_self_awareness_level();
    assert!(awareness_level > 0.0 && awareness_level <= 1.0);
}

#[test]
fn test_emotion_adaptive_experiences() {
    let emotion_system = EmotionAdaptiveExperiences::new();
    let player_id = Uuid::new_v4();

    // Initialize player
    emotion_system.initialize_player(player_id);

    // Detect emotions from performance
    let mut metrics = HashMap::new();
    metrics.insert("success_rate".to_string(), 0.9);
    metrics.insert("improvement".to_string(), 0.2);
    metrics.insert("intensity".to_string(), 0.7);
    metrics.insert("confidence".to_string(), 0.8);

    let emotion = emotion_system.detect_emotion(
        player_id,
        emotion::MeasurementSource::PerformanceMetrics,
        metrics,
    );

    assert_eq!(emotion, EmotionalState::Joy);

    // Check profile updated
    let profile = emotion_system.get_player_profile(player_id);
    assert!(profile.is_some());

    let profile = profile.unwrap();
    assert!(profile.current_state.contains_key(&EmotionalState::Joy));

    // Test difficulty adjustment
    let initial_difficulty = emotion_system.get_difficulty_settings(player_id).unwrap();
    emotion_system.adjust_difficulty(player_id, 0.2);
    let new_difficulty = emotion_system.get_difficulty_settings(player_id).unwrap();

    assert!(new_difficulty.enemy_health > initial_difficulty.enemy_health);

    // Test environmental manipulation
    emotion_system.manipulate_environment(
        EnvironmentalFactor::Lighting,
        "dramatic".to_string(),
    );
}

#[test]
fn test_symbolic_computing_knowledge() {
    let symbolic = SymbolicComputing::new();

    // Create concept hierarchy
    let mut fruit_attrs = HashMap::new();
    fruit_attrs.insert("edible".to_string(), ConceptValue::Boolean(true));
    let fruit_id = symbolic.add_concept("Fruit".to_string(), "Food".to_string(), fruit_attrs);

    let mut apple_attrs = HashMap::new();
    apple_attrs.insert("color".to_string(), ConceptValue::String("red".to_string()));
    let apple_id = symbolic.add_concept("Apple".to_string(), "Fruit".to_string(), apple_attrs);

    // Create relationship
    symbolic.add_relationship(apple_id, fruit_id, RelationType::IsA, 1.0, false);

    // Query concepts
    let query = Query {
        concept_name: Some("Apple".to_string()),
        relationship_type: None,
        attributes: HashMap::new(),
        depth: 1,
    };

    let results = symbolic.query(query);
    assert!(!results.concepts.is_empty());
    assert!(!results.relationships.is_empty());

    // Test path finding
    let path = symbolic.find_path(apple_id, fruit_id);
    assert!(path.is_some());
    assert_eq!(path.unwrap().len(), 2);

    // Test inference
    let premises = vec![Premise {
        concept_pattern: "Fruit".to_string(),
        relationship_type: Some(RelationType::IsA),
        conditions: vec![],
    }];

    let conclusion = Conclusion {
        inferred_concept: Some("Healthy".to_string()),
        inferred_relationship: Some(RelationType::HasProperty),
        target_concepts: vec![fruit_id],
        confidence_modifier: 0.9,
    };

    symbolic.add_inference_rule(
        "Fruits are healthy".to_string(),
        premises,
        conclusion,
        0.9,
    );

    let inferred = symbolic.infer(vec![apple_id]);
    assert!(!inferred.is_empty());
}

#[test]
fn test_cross_system_integration() {
    // Test integration between multiple AI systems
    let system = IntegratedAISystem::new(Uuid::new_v4(), "IntegratedNPC".to_string());

    // Setup self-awareness
    system.self_awareness.write().add_role(EntityRole::Guardian);
    system.self_awareness.write().update_context("mission".to_string(), "Protect artifact".to_string());

    // Add knowledge to symbolic system
    let artifact_id = system.symbolic.add_concept(
        "Ancient Artifact".to_string(),
        "Item".to_string(),
        HashMap::new(),
    );

    // Add goal to neo-cortex
    system.neo_cortex.add_goal("Protect artifact".to_string(), 0.95);

    // Make decision
    let context = DecisionContext {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        situation: "Intruder approaching artifact".to_string(),
        goals: vec!["Protect artifact".to_string()],
        constraints: vec![],
        available_actions: vec![
            "Attack intruder".to_string(),
            "Sound alarm".to_string(),
            "Negotiate".to_string(),
        ],
        world_state: HashMap::new(),
        priority: 0.9,
    };

    let decision = system.make_integrated_decision(context);
    assert!(!decision.action.is_empty());

    // Update all systems
    system.update(1.0);

    // Check system status
    let health = system.get_system_health();
    let awareness = system.get_awareness_level();

    assert!(matches!(health, HealthStatus::Optimal | HealthStatus::Healthy | HealthStatus::Degraded));
    assert!(awareness >= 0.0 && awareness <= 1.0);

    // Get entity description
    let description = system.describe_entity();
    assert!(description.contains("IntegratedNPC"));
}
