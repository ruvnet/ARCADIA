use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use arcadia::ai::IntegratedAISystem;
use arcadia::ai::neo_cortex::{DecisionContext, CognitiveLevel};
use arcadia::ai::goap::{GoapAction, GoapGoal, StateValue};
use uuid::Uuid;
use std::collections::HashMap;

fn integrated_ai_creation_benchmark(c: &mut Criterion) {
    c.bench_function("integrated_ai_creation", |b| {
        b.iter(|| {
            black_box(IntegratedAISystem::new(
                Uuid::new_v4(),
                "BenchmarkEntity".to_string()
            ));
        });
    });
}

fn integrated_ai_decision_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("integrated_ai_decision");
    
    let system = IntegratedAISystem::new(Uuid::new_v4(), "DecisionMaker".to_string());
    
    // Add goals to neo-cortex
    system.neo_cortex.add_goal("Survive".to_string(), 0.9);
    system.neo_cortex.add_goal("Explore".to_string(), 0.5);
    
    for num_actions in [5, 10, 20].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_actions),
            num_actions,
            |b, &num_actions| {
                let actions: Vec<String> = (0..*num_actions)
                    .map(|i| format!("Action {}", i))
                    .collect();
                
                b.iter(|| {
                    let context = DecisionContext {
                        id: Uuid::new_v4(),
                        timestamp: chrono::Utc::now(),
                        situation: "Combat encounter".to_string(),
                        goals: vec!["Survive".to_string()],
                        constraints: vec![],
                        available_actions: actions.clone(),
                        world_state: HashMap::new(),
                        priority: 0.8,
                    };
                    
                    black_box(system.make_integrated_decision(context));
                });
            },
        );
    }
    
    group.finish();
}

fn cognitive_level_switching_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("cognitive_level_switching");
    
    let system = IntegratedAISystem::new(Uuid::new_v4(), "Thinker".to_string());
    
    let levels = vec![
        ("reactive", CognitiveLevel::Reactive),
        ("tactical", CognitiveLevel::Tactical),
        ("strategic", CognitiveLevel::Strategic),
        ("abstract", CognitiveLevel::Abstract),
    ];
    
    for (name, level) in levels {
        group.bench_function(name, |b| {
            b.iter(|| {
                system.neo_cortex.set_cognitive_level(level.clone());
                
                let context = DecisionContext {
                    id: Uuid::new_v4(),
                    timestamp: chrono::Utc::now(),
                    situation: "Complex problem".to_string(),
                    goals: vec!["Solve".to_string()],
                    constraints: vec![],
                    available_actions: vec!["Think".to_string(), "Act".to_string()],
                    world_state: HashMap::new(),
                    priority: 0.7,
                };
                
                black_box(system.neo_cortex.make_decision(context));
            });
        });
    }
    
    group.finish();
}

fn goap_with_neocortex_integration_benchmark(c: &mut Criterion) {
    c.bench_function("goap_neocortex_integration", |b| {
        b.iter(|| {
            let system = IntegratedAISystem::new(Uuid::new_v4(), "IntegratedNPC".to_string());
            
            // Setup GOAP actions
            let actions = vec![
                GoapAction::new("gather_resources")
                    .with_cost(2.0)
                    .with_effect("has_resources", StateValue::Bool(true)),
                
                GoapAction::new("build_shelter")
                    .with_cost(3.0)
                    .with_precondition("has_resources", StateValue::Bool(true))
                    .with_effect("has_shelter", StateValue::Bool(true)),
                
                GoapAction::new("craft_tools")
                    .with_cost(2.0)
                    .with_precondition("has_resources", StateValue::Bool(true))
                    .with_effect("has_tools", StateValue::Bool(true)),
            ];
            
            system.goap.add_actions(actions);
            
            // Setup goals
            let goals = vec![
                GoapGoal::new("be_safe", 0.9)
                    .with_condition("has_shelter", StateValue::Bool(true)),
                GoapGoal::new("be_productive", 0.7)
                    .with_condition("has_tools", StateValue::Bool(true)),
            ];
            
            for goal in goals {
                system.goap.add_goal(goal);
            }
            
            // Plan best action sequence
            black_box(system.goap.plan_best());
            
            // Make neo-cortex decision based on GOAP plan
            let context = DecisionContext {
                id: Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                situation: "Survival scenario".to_string(),
                goals: vec!["Survive".to_string()],
                constraints: vec![],
                available_actions: vec!["Execute plan".to_string()],
                world_state: HashMap::new(),
                priority: 0.9,
            };
            
            black_box(system.make_integrated_decision(context));
        });
    });
}

fn system_update_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("system_update");
    
    for num_components in [10, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_components),
            num_components,
            |b, &num_components| {
                let system = IntegratedAISystem::new(Uuid::new_v4(), "UpdateTest".to_string());
                
                // Add components to autopoetic system
                for i in 0..num_components {
                    system.autopoetic.add_component(
                        format!("Component_{}", i),
                        vec![]
                    );
                }
                
                b.iter(|| {
                    black_box(system.update(1.0));
                });
            },
        );
    }
    
    group.finish();
}

fn emotional_state_processing_benchmark(c: &mut Criterion) {
    c.bench_function("emotional_state_processing", |b| {
        let system = IntegratedAISystem::new(Uuid::new_v4(), "EmotionalNPC".to_string());
        
        b.iter(|| {
            // Simulate emotional events
            system.emotion.process_event("player_helped");
            system.emotion.process_event("enemy_appeared");
            system.emotion.process_event("treasure_found");
            
            // Get emotional state
            black_box(system.emotion.get_emotional_state());
            
            // Adapt difficulty based on emotion
            black_box(system.emotion.adapt_difficulty());
        });
    });
}

fn symbolic_reasoning_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("symbolic_reasoning");
    
    for num_concepts in [50, 100, 200].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_concepts),
            num_concepts,
            |b, &num_concepts| {
                let system = IntegratedAISystem::new(Uuid::new_v4(), "Reasoner".to_string());
                
                // Add concepts
                for i in 0..num_concepts {
                    system.symbolic.add_concept_simple(
                        format!("concept_{}", i),
                        format!("Description of concept {}", i)
                    );
                }
                
                // Add relationships
                for i in 0..num_concepts/2 {
                    system.symbolic.add_relationship(
                        format!("concept_{}", i),
                        format!("concept_{}", i + num_concepts/2),
                        "relates_to".to_string()
                    );
                }
                
                b.iter(|| {
                    // Query concepts
                    black_box(system.symbolic.query_concept(&format!("concept_{}", num_concepts/2)));
                    
                    // Perform inference
                    black_box(system.symbolic.infer_new_facts());
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    integrated_ai_creation_benchmark,
    integrated_ai_decision_benchmark,
    cognitive_level_switching_benchmark,
    goap_with_neocortex_integration_benchmark,
    system_update_benchmark,
    emotional_state_processing_benchmark,
    symbolic_reasoning_benchmark
);
criterion_main!(benches);
