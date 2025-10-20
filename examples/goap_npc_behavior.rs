//! GOAP NPC Behavior Example
//!
//! Demonstrates how to use ARCADIA's GOAP (Goal-Oriented Action Planning)
//! system to create intelligent, goal-driven NPC behavior.
//!
//! This example shows:
//! - Setting up a GOAP planner
//! - Defining actions with preconditions and effects
//! - Creating goals with priorities
//! - Planning optimal action sequences
//! - Executing plans and updating world state
//! - Replanning when world state changes

use arcadia::ai::goap::{GoapPlanner, GoapAction, GoapGoal, StateValue};
use std::collections::HashMap;

fn main() {
    println!("=== ARCADIA GOAP NPC Behavior Example ===\n");
    
    // Scenario 1: Simple Combat NPC
    println!("--- Scenario 1: Combat NPC Planning ---");
    combat_npc_example();
    
    // Scenario 2: Survival NPC
    println!("\n--- Scenario 2: Survival NPC Planning ---");
    survival_npc_example();
    
    // Scenario 3: Stealth NPC
    println!("\n--- Scenario 3: Stealth NPC Planning ---");
    stealth_npc_example();
    
    // Scenario 4: Dynamic Replanning
    println!("\n--- Scenario 4: Dynamic Replanning ---");
    dynamic_replanning_example();
}

fn combat_npc_example() {
    let planner = GoapPlanner::new();
    
    // Define combat actions
    let actions = vec![
        GoapAction::new("scout_area")
            .with_cost(1.0)
            .with_effect("area_scouted", StateValue::Bool(true))
            .with_metadata("description", "Scout the area for enemies"),
        
        GoapAction::new("find_weapon")
            .with_cost(3.0)
            .with_precondition("area_scouted", StateValue::Bool(true))
            .with_effect("has_weapon", StateValue::Bool(true))
            .with_metadata("description", "Search for and equip a weapon"),
        
        GoapAction::new("find_armor")
            .with_cost(2.0)
            .with_precondition("area_scouted", StateValue::Bool(true))
            .with_effect("has_armor", StateValue::Bool(true))
            .with_metadata("description", "Find and equip protective armor"),
        
        GoapAction::new("approach_enemy")
            .with_cost(2.0)
            .with_precondition("has_weapon", StateValue::Bool(true))
            .with_effect("in_combat_range", StateValue::Bool(true))
            .with_metadata("description", "Move within striking distance"),
        
        GoapAction::new("attack_enemy")
            .with_cost(1.0)
            .with_precondition("has_weapon", StateValue::Bool(true))
            .with_precondition("in_combat_range", StateValue::Bool(true))
            .with_effect("enemy_defeated", StateValue::Bool(true))
            .with_metadata("description", "Engage and defeat the enemy"),
    ];
    
    planner.add_actions(actions);
    
    // Set initial world state (NPC starts with nothing)
    planner.set_world_state(HashMap::new());
    
    // Create goal: defeat enemy
    let goal = GoapGoal::new("defeat_enemy", 1.0)
        .with_condition("enemy_defeated", StateValue::Bool(true));
    
    // Plan action sequence
    println!("Initial State: NPC has no equipment");
    println!("Goal: Defeat enemy\n");
    
    match planner.plan(&goal) {
        Some(plan) => {
            println!("✓ Plan found! ({} steps, cost: {:.1})", plan.len(), plan.total_cost);
            println!("Action sequence:");
            for (i, action) in plan.actions.iter().enumerate() {
                println!("  {}. {} - {}", 
                    i + 1, 
                    action.name,
                    action.metadata.get("description").unwrap_or(&"".to_string())
                );
            }
            
            let stats = planner.get_stats();
            println!("\nPlanning stats:");
            println!("  Nodes explored: {}", stats.nodes_explored);
            println!("  Planning time: {}ms", stats.planning_time_ms);
        }
        None => {
            println!("✗ No plan found to achieve goal");
        }
    }
}

fn survival_npc_example() {
    let planner = GoapPlanner::new();
    
    // Define survival actions
    let actions = vec![
        GoapAction::new("gather_wood")
            .with_cost(2.0)
            .with_effect("has_wood", StateValue::Bool(true))
            .with_metadata("description", "Collect wood from trees"),
        
        GoapAction::new("gather_stone")
            .with_cost(2.0)
            .with_effect("has_stone", StateValue::Bool(true))
            .with_metadata("description", "Mine stone from rocks"),
        
        GoapAction::new("hunt_food")
            .with_cost(3.0)
            .with_effect("has_food", StateValue::Bool(true))
            .with_metadata("description", "Hunt animals for food"),
        
        GoapAction::new("build_shelter")
            .with_cost(4.0)
            .with_precondition("has_wood", StateValue::Bool(true))
            .with_precondition("has_stone", StateValue::Bool(true))
            .with_effect("has_shelter", StateValue::Bool(true))
            .with_metadata("description", "Construct a protective shelter"),
        
        GoapAction::new("cook_food")
            .with_cost(1.0)
            .with_precondition("has_food", StateValue::Bool(true))
            .with_precondition("has_shelter", StateValue::Bool(true))
            .with_effect("has_cooked_food", StateValue::Bool(true))
            .with_metadata("description", "Prepare food at shelter"),
        
        GoapAction::new("eat_food")
            .with_cost(1.0)
            .with_precondition("has_cooked_food", StateValue::Bool(true))
            .with_effect("is_fed", StateValue::Bool(true))
            .with_metadata("description", "Consume prepared food"),
    ];
    
    planner.add_actions(actions);
    
    // Initial state: hungry survivor
    let mut initial_state = HashMap::new();
    initial_state.insert("is_hungry".to_string(), StateValue::Bool(true));
    planner.set_world_state(initial_state);
    
    // Create multiple goals with different priorities
    let goals = vec![
        GoapGoal::new("stay_alive", 0.95)
            .with_condition("is_fed", StateValue::Bool(true)),
        
        GoapGoal::new("be_safe", 0.80)
            .with_condition("has_shelter", StateValue::Bool(true)),
    ];
    
    for goal in goals {
        planner.add_goal(goal);
    }
    
    // Plan for highest priority goal
    println!("Initial State: Hungry survivor with no resources");
    println!("Multiple goals with different priorities\n");
    
    if let Some(selected_goal) = planner.select_goal() {
        println!("Selected goal: {} (priority: {:.2})", selected_goal.name, selected_goal.priority);
        
        if let Some(plan) = planner.plan(&selected_goal) {
            println!("✓ Plan found! ({} steps, cost: {:.1})", plan.len(), plan.total_cost);
            println!("Action sequence:");
            for (i, action) in plan.actions.iter().enumerate() {
                println!("  {}. {} - {}", 
                    i + 1, 
                    action.name,
                    action.metadata.get("description").unwrap_or(&"".to_string())
                );
            }
        }
    }
}

fn stealth_npc_example() {
    let planner = GoapPlanner::new();
    
    // Define stealth actions
    let actions = vec![
        GoapAction::new("find_shadow")
            .with_cost(1.0)
            .with_effect("in_shadows", StateValue::Bool(true))
            .with_metadata("description", "Move to shadowy area"),
        
        GoapAction::new("equip_silencer")
            .with_cost(2.0)
            .with_effect("has_silencer", StateValue::Bool(true))
            .with_metadata("description", "Attach silencer to weapon"),
        
        GoapAction::new("disable_lights")
            .with_cost(3.0)
            .with_precondition("in_shadows", StateValue::Bool(true))
            .with_effect("lights_disabled", StateValue::Bool(true))
            .with_metadata("description", "Turn off nearby lights"),
        
        GoapAction::new("sneak_past_guard")
            .with_cost(2.0)
            .with_precondition("in_shadows", StateValue::Bool(true))
            .with_precondition("lights_disabled", StateValue::Bool(true))
            .with_effect("guard_bypassed", StateValue::Bool(true))
            .with_metadata("description", "Quietly move past guard"),
        
        GoapAction::new("silent_takedown")
            .with_cost(2.0)
            .with_precondition("has_silencer", StateValue::Bool(true))
            .with_precondition("in_shadows", StateValue::Bool(true))
            .with_effect("guard_neutralized", StateValue::Bool(true))
            .with_metadata("description", "Silently neutralize guard"),
        
        GoapAction::new("access_terminal")
            .with_cost(1.0)
            .with_precondition("guard_neutralized", StateValue::Bool(true))
            .with_effect("data_acquired", StateValue::Bool(true))
            .with_metadata("description", "Extract data from terminal"),
    ];
    
    planner.add_actions(actions);
    
    // Initial state
    planner.set_world_state(HashMap::new());
    
    // Goal: acquire data stealthily
    let goal = GoapGoal::new("stealth_mission", 1.0)
        .with_condition("data_acquired", StateValue::Bool(true));
    
    println!("Initial State: Infiltration mission starting");
    println!("Goal: Acquire data without alerting guards\n");
    
    if let Some(plan) = planner.plan(&goal) {
        println!("✓ Stealth plan found! ({} steps, cost: {:.1})", plan.len(), plan.total_cost);
        println!("Action sequence:");
        for (i, action) in plan.actions.iter().enumerate() {
            println!("  {}. {} - {}", 
                i + 1, 
                action.name,
                action.metadata.get("description").unwrap_or(&"".to_string())
            );
        }
    }
}

fn dynamic_replanning_example() {
    let planner = GoapPlanner::new();
    
    // Define flexible actions
    let actions = vec![
        GoapAction::new("use_door")
            .with_cost(1.0)
            .with_precondition("door_unlocked", StateValue::Bool(true))
            .with_effect("inside_building", StateValue::Bool(true)),
        
        GoapAction::new("pick_lock")
            .with_cost(3.0)
            .with_precondition("has_lockpicks", StateValue::Bool(true))
            .with_effect("door_unlocked", StateValue::Bool(true)),
        
        GoapAction::new("break_window")
            .with_cost(2.0)
            .with_effect("inside_building", StateValue::Bool(true))
            .with_effect("alarm_triggered", StateValue::Bool(true)),
        
        GoapAction::new("find_lockpicks")
            .with_cost(4.0)
            .with_effect("has_lockpicks", StateValue::Bool(true)),
    ];
    
    planner.add_actions(actions);
    
    // Goal: get inside
    let goal = GoapGoal::new("enter_building", 1.0)
        .with_condition("inside_building", StateValue::Bool(true));
    
    // Scenario 1: Door is already unlocked
    println!("Scenario 1: Door is unlocked");
    let mut state1 = HashMap::new();
    state1.insert("door_unlocked".to_string(), StateValue::Bool(true));
    planner.set_world_state(state1);
    
    if let Some(plan) = planner.plan(&goal) {
        println!("  Plan: {} (cost: {:.1})", 
            plan.actions.iter().map(|a| &a.name).cloned().collect::<Vec<_>>().join(" → "),
            plan.total_cost
        );
    }
    
    // Scenario 2: Door is locked, but NPC has lockpicks
    println!("\nScenario 2: Door locked, has lockpicks");
    let mut state2 = HashMap::new();
    state2.insert("door_unlocked".to_string(), StateValue::Bool(false));
    state2.insert("has_lockpicks".to_string(), StateValue::Bool(true));
    planner.set_world_state(state2);
    
    if let Some(plan) = planner.plan(&goal) {
        println!("  Plan: {} (cost: {:.1})", 
            plan.actions.iter().map(|a| &a.name).cloned().collect::<Vec<_>>().join(" → "),
            plan.total_cost
        );
    }
    
    // Scenario 3: Door is locked, no lockpicks
    println!("\nScenario 3: Door locked, no lockpicks (must break window)");
    let state3 = HashMap::new();
    planner.set_world_state(state3);
    
    if let Some(plan) = planner.plan(&goal) {
        println!("  Plan: {} (cost: {:.1})", 
            plan.actions.iter().map(|a| &a.name).cloned().collect::<Vec<_>>().join(" → "),
            plan.total_cost
        );
        
        // Check if alarm was triggered
        let final_state = plan.actions.iter().fold(HashMap::new(), |state, action| {
            action.apply(&state)
        });
        
        if final_state.get("alarm_triggered").and_then(|v| v.as_bool()).unwrap_or(false) {
            println!("  ⚠ Warning: This plan triggers the alarm!");
        }
    }
    
    println!("\n✓ GOAP successfully replanned for different situations");
}
