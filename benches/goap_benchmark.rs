use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use arcadia::ai::goap::{GoapPlanner, GoapAction, GoapGoal, StateValue};
use std::collections::HashMap;

fn simple_planning_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("goap_simple_planning");
    
    for num_actions in [5, 10, 20, 50].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_actions),
            num_actions,
            |b, &num_actions| {
                b.iter(|| {
                    let planner = GoapPlanner::new();
                    
                    // Create actions
                    for i in 0..num_actions {
                        let action = GoapAction::new(format!("action_{}", i))
                            .with_cost(1.0)
                            .with_precondition(format!("state_{}", i), StateValue::Bool(true))
                            .with_effect(format!("state_{}", i + 1), StateValue::Bool(true));
                        planner.add_action(action);
                    }
                    
                    // Set initial state
                    let mut initial_state = HashMap::new();
                    initial_state.insert("state_0".to_string(), StateValue::Bool(true));
                    planner.set_world_state(initial_state);
                    
                    // Create goal
                    let goal = GoapGoal::new("reach_end", 1.0)
                        .with_condition(format!("state_{}", num_actions), StateValue::Bool(true));
                    
                    // Plan
                    black_box(planner.plan(&goal));
                });
            },
        );
    }
    
    group.finish();
}

fn complex_planning_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("goap_complex_planning");
    
    for branching_factor in [2, 3, 5].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(branching_factor),
            branching_factor,
            |b, &branching_factor| {
                b.iter(|| {
                    let planner = GoapPlanner::new();
                    
                    // Create branching action tree
                    for level in 0..5 {
                        for branch in 0..branching_factor {
                            let action = GoapAction::new(format!("action_{}_{}", level, branch))
                                .with_cost((level + 1) as f64)
                                .with_precondition(
                                    format!("level_{}", level),
                                    StateValue::Bool(true)
                                )
                                .with_effect(
                                    format!("level_{}_branch_{}", level + 1, branch),
                                    StateValue::Bool(true)
                                );
                            planner.add_action(action);
                        }
                    }
                    
                    // Merge actions
                    for branch in 0..branching_factor {
                        let merge = GoapAction::new(format!("merge_{}", branch))
                            .with_cost(1.0)
                            .with_precondition(
                                format!("level_5_branch_{}", branch),
                                StateValue::Bool(true)
                            )
                            .with_effect("goal_reached".to_string(), StateValue::Bool(true));
                        planner.add_action(merge);
                    }
                    
                    // Set initial state
                    let mut initial_state = HashMap::new();
                    initial_state.insert("level_0".to_string(), StateValue::Bool(true));
                    planner.set_world_state(initial_state);
                    
                    // Create goal
                    let goal = GoapGoal::new("complete", 1.0)
                        .with_condition("goal_reached".to_string(), StateValue::Bool(true));
                    
                    // Plan
                    black_box(planner.plan(&goal));
                });
            },
        );
    }
    
    group.finish();
}

fn goal_selection_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("goap_goal_selection");
    
    for num_goals in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_goals),
            num_goals,
            |b, &num_goals| {
                b.iter(|| {
                    let planner = GoapPlanner::new();
                    
                    // Add many goals with varying priorities
                    for i in 0..num_goals {
                        let goal = GoapGoal::new(format!("goal_{}", i), i as f64 / num_goals as f64)
                            .with_condition(format!("condition_{}", i), StateValue::Bool(true));
                        planner.add_goal(goal);
                    }
                    
                    // Select best goal
                    black_box(planner.select_goal());
                });
            },
        );
    }
    
    group.finish();
}

fn world_state_operations_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("goap_world_state");
    
    // Test state updates
    group.bench_function("state_update", |b| {
        let planner = GoapPlanner::new();
        b.iter(|| {
            for i in 0..100 {
                planner.update_state(
                    format!("key_{}", i),
                    StateValue::Int(i as i64)
                );
            }
        });
    });
    
    // Test action validity checking
    group.bench_function("action_validity", |b| {
        let mut state = HashMap::new();
        for i in 0..50 {
            state.insert(format!("key_{}", i), StateValue::Bool(true));
        }
        
        let action = GoapAction::new("test")
            .with_precondition("key_25", StateValue::Bool(true))
            .with_precondition("key_30", StateValue::Bool(true));
        
        b.iter(|| {
            black_box(action.is_valid(&state));
        });
    });
    
    // Test action application
    group.bench_function("action_application", |b| {
        let mut state = HashMap::new();
        for i in 0..50 {
            state.insert(format!("key_{}", i), StateValue::Bool(false));
        }
        
        let action = GoapAction::new("test")
            .with_effect("key_10", StateValue::Bool(true))
            .with_effect("key_20", StateValue::Bool(true))
            .with_effect("key_30", StateValue::Bool(true));
        
        b.iter(|| {
            black_box(action.apply(&state));
        });
    });
    
    group.finish();
}

fn npc_combat_scenario_benchmark(c: &mut Criterion) {
    c.bench_function("goap_npc_combat_planning", |b| {
        b.iter(|| {
            let planner = GoapPlanner::new();
            
            // Define combat actions
            let actions = vec![
                GoapAction::new("find_weapon")
                    .with_cost(3.0)
                    .with_effect("has_weapon", StateValue::Bool(true)),
                
                GoapAction::new("find_health")
                    .with_cost(2.0)
                    .with_effect("has_health_pack", StateValue::Bool(true)),
                
                GoapAction::new("heal_self")
                    .with_cost(1.0)
                    .with_precondition("has_health_pack", StateValue::Bool(true))
                    .with_effect("is_healthy", StateValue::Bool(true)),
                
                GoapAction::new("approach_enemy")
                    .with_cost(2.0)
                    .with_precondition("has_weapon", StateValue::Bool(true))
                    .with_effect("in_combat_range", StateValue::Bool(true)),
                
                GoapAction::new("attack_enemy")
                    .with_cost(1.0)
                    .with_precondition("has_weapon", StateValue::Bool(true))
                    .with_precondition("in_combat_range", StateValue::Bool(true))
                    .with_effect("enemy_defeated", StateValue::Bool(true)),
            ];
            
            planner.add_actions(actions);
            
            // Initial state: NPC has nothing
            planner.set_world_state(HashMap::new());
            
            // Goal: defeat enemy
            let goal = GoapGoal::new("defeat_enemy", 1.0)
                .with_condition("enemy_defeated", StateValue::Bool(true));
            
            // Plan
            black_box(planner.plan(&goal));
        });
    });
}

criterion_group!(
    benches,
    simple_planning_benchmark,
    complex_planning_benchmark,
    goal_selection_benchmark,
    world_state_operations_benchmark,
    npc_combat_scenario_benchmark
);
criterion_main!(benches);
