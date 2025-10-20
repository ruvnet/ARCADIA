//! Example: Creating an Intelligent NPC with Full AI Systems
//!
//! This example demonstrates how to use all ARCADIA AI systems together
//! to create a sophisticated, adaptive NPC.

use arcadia::ai::*;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{Utc, Duration};

fn main() {
    // Create a new NPC with integrated AI
    let npc_id = Uuid::new_v4();
    let guardian_ai = IntegratedAISystem::new(npc_id, "Ancient Guardian".to_string());

    println!("=== Creating Ancient Guardian NPC ===\n");

    // ===== 1. SETUP SELF-AWARENESS =====
    println!("1. Setting up self-awareness...");
    {
        let mut sa = guardian_ai.self_awareness.write();

        // Define roles
        sa.add_role(EntityRole::Guardian);
        sa.add_role(EntityRole::QuestGiver);

        // Set consciousness
        sa.set_consciousness_state(ConsciousnessState::Aware);

        // Form initial beliefs
        sa.form_belief("The artifact must be protected at all costs".to_string(), 0.95);
        sa.form_belief("Most visitors are curious, not hostile".to_string(), 0.7);

        // Set context
        sa.update_context("location".to_string(), "Ancient Temple".to_string());
        sa.update_context("duty".to_string(), "Protect the Sacred Artifact".to_string());

        // Add motivations
        sa.add_motivation("Fulfill my sacred duty".to_string());
        sa.add_motivation("Share knowledge with the worthy".to_string());

        println!("  ✓ Identity established: {}", sa.describe_self());
    }

    // ===== 2. SETUP KNOWLEDGE BASE =====
    println!("\n2. Building knowledge base...");
    {
        // Create concepts
        let mut artifact_attrs = HashMap::new();
        artifact_attrs.insert("sacred".to_string(), ConceptValue::Boolean(true));
        artifact_attrs.insert("power_level".to_string(), ConceptValue::Number(0.9));
        let artifact_id = guardian_ai.symbolic.add_concept(
            "Sacred Artifact".to_string(),
            "Item".to_string(),
            artifact_attrs,
        );

        let temple_id = guardian_ai.symbolic.add_concept(
            "Ancient Temple".to_string(),
            "Location".to_string(),
            HashMap::new(),
        );

        // Create relationships
        guardian_ai.symbolic.add_relationship(
            artifact_id,
            temple_id,
            RelationType::PartOf,
            1.0,
            false,
        );

        // Add inference rules
        let premises = vec![Premise {
            concept_pattern: "Item".to_string(),
            relationship_type: Some(RelationType::PartOf),
            conditions: vec!["sacred=true".to_string()],
        }];

        let conclusion = Conclusion {
            inferred_concept: Some("Protected".to_string()),
            inferred_relationship: Some(RelationType::HasProperty),
            target_concepts: vec![artifact_id],
            confidence_modifier: 0.9,
        };

        guardian_ai.symbolic.add_inference_rule(
            "Sacred items in temple are protected".to_string(),
            premises,
            conclusion,
            0.95,
        );

        println!("  ✓ Knowledge base created with {} concepts",
                 guardian_ai.symbolic.get_concept_count());
    }

    // ===== 3. SETUP GOALS AND STRATEGIES =====
    println!("\n3. Configuring goals and strategies...");
    {
        // Set goals with priorities
        guardian_ai.neo_cortex.add_goal("Protect artifact".to_string(), 1.0);
        guardian_ai.neo_cortex.add_goal("Educate worthy visitors".to_string(), 0.6);
        guardian_ai.neo_cortex.add_goal("Maintain temple".to_string(), 0.4);

        // Set cognitive level
        guardian_ai.neo_cortex.set_cognitive_level(CognitiveLevel::Strategic);

        // Create a defensive strategy
        let strategy = guardian_ai.neo_cortex.plan_strategy(
            "Defend against intruders".to_string(),
            5,
        );

        println!("  ✓ Goals set, strategy planned: {}", strategy.name);
    }

    // ===== 4. SETUP AUTOPOETIC SYSTEM =====
    println!("\n4. Initializing self-maintenance systems...");
    {
        // Add system components
        let awareness = guardian_ai.autopoetic.add_component(
            "Awareness Systems".to_string(),
            vec![],
        );

        let defense = guardian_ai.autopoetic.add_component(
            "Defense Mechanisms".to_string(),
            vec![awareness],
        );

        let knowledge = guardian_ai.autopoetic.add_component(
            "Knowledge Core".to_string(),
            vec![awareness],
        );

        // Add adaptation rule
        let mut conditions = HashMap::new();
        conditions.insert("threat_level".to_string(), 0.7);
        guardian_ai.autopoetic.add_adaptation_rule(
            "threat_level".to_string(),
            "increase_efficiency".to_string(),
            0.7,
        );

        println!("  ✓ Self-maintenance systems online, health: {:.2}",
                 guardian_ai.autopoetic.get_system_health());
    }

    // ===== 5. INITIALIZE EVOLUTIONARY LEARNING =====
    println!("\n5. Activating evolutionary learning...");
    {
        // Create initial behavioral genome
        let genome = guardian_ai.evolutionary.create_genome(8);
        guardian_ai.evolutionary.add_to_population(genome);

        // Add fitness criteria for behavior
        guardian_ai.evolutionary.add_fitness_criteria(FitnessCriteria {
            name: "gene_0".to_string(), // Represents "vigilance"
            weight: 0.8,
            target_value: 0.8,
            tolerance: 0.1,
        });

        guardian_ai.evolutionary.add_fitness_criteria(FitnessCriteria {
            name: "gene_1".to_string(), // Represents "wisdom"
            weight: 0.6,
            target_value: 0.7,
            tolerance: 0.15,
        });

        println!("  ✓ Evolutionary learning initialized");
    }

    // ===== 6. SIMULATE NPC BEHAVIOR =====
    println!("\n6. Simulating NPC behavior...\n");
    println!("--- Scenario: Player Approaches the Temple ---\n");

    // Detect player approach
    let player_id = Uuid::new_v4();
    {
        let mut sa = guardian_ai.self_awareness.write();
        sa.record_relationship(
            player_id,
            "Visitor".to_string(),
            "unknown".to_string(),
            0.0, // Neutral initially
        );

        // Update state
        let mut physical = HashMap::new();
        physical.insert("alertness".to_string(), 0.8);

        let mut emotional = HashMap::new();
        emotional.insert("caution".to_string(), 0.6);

        let mut cognitive = HashMap::new();
        cognitive.insert("threat_assessment".to_string(), 0.3);

        let snapshot = StateSnapshot {
            timestamp: Utc::now(),
            physical_state: physical,
            emotional_state: emotional,
            cognitive_state: cognitive,
            environmental_context: HashMap::new(),
        };

        sa.record_state(snapshot);
    }

    // Make decision about how to greet player
    let greeting_context = DecisionContext {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        situation: "Unknown visitor approaching temple entrance".to_string(),
        goals: vec!["Protect artifact".to_string(), "Assess visitor intentions".to_string()],
        constraints: vec!["No violence unless provoked".to_string()],
        available_actions: vec![
            "Greet visitor warmly".to_string(),
            "Challenge visitor's purpose".to_string(),
            "Observe silently".to_string(),
            "Block entrance".to_string(),
        ],
        world_state: HashMap::new(),
        priority: 0.7,
    };

    let decision = guardian_ai.make_integrated_decision(greeting_context);
    println!("Decision: {}", decision.action);
    println!("Reasoning: {}", decision.reasoning);
    println!("Confidence: {:.2}%\n", decision.confidence * 100.0);

    // Record outcome
    guardian_ai.neo_cortex.record_outcome(decision.clone(), 0.8, true);

    // ===== 7. SIMULATE PLAYER INTERACTION =====
    println!("--- Player asks about the artifact ---\n");

    // Query knowledge base
    let query = Query {
        concept_name: Some("Sacred Artifact".to_string()),
        relationship_type: None,
        attributes: HashMap::new(),
        depth: 2,
    };

    let knowledge = guardian_ai.symbolic.query(query);
    println!("Knowledge retrieved: {} related concepts", knowledge.concepts.len());

    // Make decision about sharing knowledge
    let knowledge_context = DecisionContext {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        situation: "Visitor asks about the artifact".to_string(),
        goals: vec!["Educate worthy visitors".to_string()],
        constraints: vec![],
        available_actions: vec![
            "Share ancient legends".to_string(),
            "Test visitor's worthiness".to_string(),
            "Refuse to answer".to_string(),
        ],
        world_state: HashMap::new(),
        priority: 0.6,
    };

    let decision2 = guardian_ai.make_integrated_decision(knowledge_context);
    println!("\nDecision: {}", decision2.action);
    println!("Confidence: {:.2}%\n", decision2.confidence * 100.0);

    // Update relationship based on positive interaction
    {
        let mut sa = guardian_ai.self_awareness.write();
        sa.record_relationship(
            player_id,
            "Worthy Seeker".to_string(),
            "respectful".to_string(),
            0.6,
        );
    }

    // ===== 8. SYSTEM MAINTENANCE CYCLE =====
    println!("--- Running system maintenance ---\n");

    // Update all systems
    guardian_ai.update(1.0);

    // Check system health
    let health_status = guardian_ai.get_system_health();
    let awareness_level = guardian_ai.get_awareness_level();

    println!("System Health: {:?}", health_status);
    println!("Self-Awareness Level: {:.2}%", awareness_level * 100.0);

    // Detect emergent patterns
    let patterns = guardian_ai.autopoetic.detect_emergent_patterns();
    if !patterns.is_empty() {
        println!("\nEmergent Patterns Detected:");
        for pattern in patterns {
            println!("  - {} (strength: {:.2})", pattern.name, pattern.strength);
        }
    }

    // ===== 9. META-REASONING =====
    println!("\n--- NPC engages in self-reflection ---\n");

    {
        let mut sa = guardian_ai.self_awareness.write();
        sa.set_consciousness_state(ConsciousnessState::Reflective);
        sa.engage_meta_reasoning();

        let thoughts = sa.get_recent_meta_thoughts(3);
        println!("Meta-Thoughts:");
        for thought in thoughts {
            println!("  [{:?}] {}", thought.thought_type, thought.content);
        }
    }

    // ===== 10. FINAL STATUS =====
    println!("\n=== Final NPC Status ===\n");
    println!("{}", guardian_ai.describe_entity());

    let goals = guardian_ai.neo_cortex.get_goals_by_priority();
    println!("\nActive Goals:");
    for (goal, priority) in goals.iter().take(3) {
        println!("  - {} (priority: {:.2})", goal, priority);
    }

    let all_beliefs = guardian_ai.self_awareness.read().get_all_beliefs();
    println!("\nCore Beliefs:");
    for belief in all_beliefs.iter().take(3) {
        println!("  - {} (confidence: {:.2})", belief.statement, belief.confidence);
    }

    println!("\nThe Ancient Guardian is now fully operational and ready to interact with players!");
}
