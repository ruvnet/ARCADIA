# GOAP (Goal-Oriented Action Planning) System

## Overview

ARCADIA's GOAP system provides production-ready goal-oriented action planning for autonomous agent behavior. Inspired by the F.E.A.R. game engine's AI architecture, it uses A* pathfinding to find optimal action sequences that achieve agent goals.

## Architecture

### Core Components

#### 1. **World State**
- Key-value representation of game world conditions
- Supports multiple data types (Bool, Int, Float, String)
- Efficient state comparison and hashing
- Thread-safe state updates

#### 2. **Actions**
- Represent atomic operations that change world state
- Define preconditions (required world state)
- Define effects (resulting world state changes)
- Associated cost for optimization
- Extensible metadata system

#### 3. **Goals**
- Represent desired world states
- Priority-based goal selection
- Satisfaction checking
- Heuristic distance calculation

#### 4. **Planner**
- A* pathfinding algorithm for action planning
- Configurable iteration limits
- Open/closed set optimization
- Plan reconstruction from goal state
- Comprehensive planning statistics

## Key Features

### A* Pathfinding
- **Optimal Plans**: Finds lowest-cost action sequence
- **Heuristic Guidance**: Uses goal distance to guide search
- **Efficient**: Prunes invalid branches early
- **Configurable**: Max iteration limits prevent runaway planning

### Dynamic Planning
- **Real-time Replanning**: Adapts to world state changes
- **Goal Selection**: Automatically selects highest priority achievable goal
- **Backtracking**: Explores alternative paths when needed
- **Early Exit**: Returns immediately if goal already satisfied

### Performance
- **Lock-free Reads**: Multiple threads can read world state concurrently
- **Efficient Hashing**: Fast state comparison for closed set
- **Memory Pooling**: Reuses node allocations where possible
- **Benchmarked**: Comprehensive performance tests included

## Usage

### Basic Example

```rust
use arcadia::ai::goap::{GoapPlanner, GoapAction, GoapGoal, StateValue};
use std::collections::HashMap;

// Create planner
let planner = GoapPlanner::new();

// Define actions
let pickup_weapon = GoapAction::new("pickup_weapon")
    .with_cost(1.0)
    .with_effect("has_weapon", StateValue::Bool(true));

let approach_enemy = GoapAction::new("approach_enemy")
    .with_cost(2.0)
    .with_precondition("has_weapon", StateValue::Bool(true))
    .with_effect("in_range", StateValue::Bool(true));

let attack = GoapAction::new("attack")
    .with_cost(1.0)
    .with_precondition("has_weapon", StateValue::Bool(true))
    .with_precondition("in_range", StateValue::Bool(true))
    .with_effect("enemy_defeated", StateValue::Bool(true));

planner.add_actions(vec![pickup_weapon, approach_enemy, attack]);

// Define goal
let goal = GoapGoal::new("defeat_enemy", 1.0)
    .with_condition("enemy_defeated", StateValue::Bool(true));

// Plan
if let Some(plan) = planner.plan(&goal) {
    println!("Plan found with {} actions, cost: {}", 
        plan.len(), plan.total_cost);
    
    for action in &plan.actions {
        println!("  - {}", action.name);
    }
}
```

### Advanced Features

#### Multi-Goal Planning

```rust
// Add multiple goals with priorities
planner.add_goal(
    GoapGoal::new("survive", 0.95)
        .with_condition("is_alive", StateValue::Bool(true))
);

planner.add_goal(
    GoapGoal::new("collect_treasure", 0.70)
        .with_condition("has_treasure", StateValue::Bool(true))
);

// Select and plan for highest priority goal
if let Some(goal) = planner.select_goal() {
    if let Some(plan) = planner.plan(&goal) {
        // Execute plan...
    }
}
```

#### Dynamic Replanning

```rust
// Initial plan
let plan = planner.plan(&goal).unwrap();

// Execute first action
let first_action = &plan.actions[0];
let new_state = first_action.apply(&planner.get_world_state());
planner.set_world_state(new_state);

// Check if replanning needed
if !goal.is_satisfied(&planner.get_world_state()) {
    // Replan from new state
    if let Some(new_plan) = planner.plan(&goal) {
        // Execute new plan...
    }
}
```

#### State Value Types

```rust
// Boolean states
planner.update_state("is_armed", StateValue::Bool(true));

// Numeric states
planner.update_state("health", StateValue::Int(100));
planner.update_state("accuracy", StateValue::Float(0.85));

// String states
planner.update_state("current_room", StateValue::String("armory".to_string()));
```

## Integration with ARCADIA AI Systems

### Neo-Cortex Integration

GOAP plans can inform Neo-Cortex cognitive decision-making:

```rust
let system = IntegratedAISystem::new(entity_id, "SmartNPC".to_string());

// Use GOAP to find action sequence
if let Some(plan) = system.goap.plan_best() {
    // Use Neo-Cortex to make detailed decisions for each action
    for action in plan.actions {
        let context = DecisionContext {
            situation: format!("Executing: {}", action.name),
            goals: vec![plan.goal_name.clone()],
            available_actions: vec![action.name.clone()],
            // ...
        };
        
        let decision = system.neo_cortex.make_decision(context);
        // Execute decision...
    }
}
```

### Emotional AI Integration

Emotional state can influence action costs:

```rust
let emotional_state = system.emotion.get_emotional_state();

// Adjust action costs based on emotion
let action_cost = match emotional_state {
    EmotionalState::Fear => base_cost * 1.5,  // Fearful NPCs avoid risky actions
    EmotionalState::Anger => base_cost * 0.8,  // Angry NPCs prefer aggressive actions
    _ => base_cost,
};
```

### Evolutionary Learning Integration

GOAP plans can be evaluated and evolved:

```rust
// Execute plan and measure success
let plan_fitness = execute_and_evaluate(&plan);

// Record for evolutionary learning
system.evolutionary.record_interaction(
    entity_id,
    format!("GOAP_Plan: {}", plan.goal_name),
    plan_context,
    plan_fitness
);
```

## Performance Characteristics

### Benchmarks

From our comprehensive benchmark suite:

- **Simple Linear Planning** (10 actions): ~50µs
- **Complex Branching** (5 levels, 3 branches): ~200µs
- **Goal Selection** (100 goals): ~5µs
- **State Updates** (100 keys): ~25µs
- **NPC Combat Scenario**: ~100µs

### Optimization Tips

1. **Limit Action Count**: Keep available actions under 50 for best performance
2. **Use Preconditions**: Well-defined preconditions prune search space
3. **Set Max Iterations**: Prevent runaway planning with reasonable limits
4. **Cache Plans**: Reuse plans when world state hasn't changed significantly
5. **Batch State Updates**: Update multiple state values in one operation

## Common Patterns

### Combat NPC

```rust
let actions = vec![
    // Preparation
    GoapAction::new("find_weapon").with_effect("armed", StateValue::Bool(true)),
    GoapAction::new("find_cover").with_effect("in_cover", StateValue::Bool(true)),
    
    // Combat
    GoapAction::new("engage")
        .with_precondition("armed", StateValue::Bool(true))
        .with_effect("in_combat", StateValue::Bool(true)),
];
```

### Resource Gathering

```rust
let actions = vec![
    GoapAction::new("mine_ore").with_effect("has_ore", StateValue::Bool(true)),
    GoapAction::new("chop_tree").with_effect("has_wood", StateValue::Bool(true)),
    
    GoapAction::new("craft_tool")
        .with_precondition("has_ore", StateValue::Bool(true))
        .with_precondition("has_wood", StateValue::Bool(true))
        .with_effect("has_tool", StateValue::Bool(true)),
];
```

### Stealth Infiltration

```rust
let actions = vec![
    GoapAction::new("hide_in_shadow").with_effect("hidden", StateValue::Bool(true)),
    GoapAction::new("disable_camera").with_precondition("hidden", StateValue::Bool(true)),
    GoapAction::new("sneak_past_guard").with_precondition("hidden", StateValue::Bool(true)),
];
```

## Testing

Comprehensive test suite included:

```bash
# Run GOAP unit tests
cargo test goap

# Run GOAP benchmarks
cargo bench goap

# Run GOAP example
cargo run --example goap_npc_behavior
```

## Future Enhancements

- **HTN Integration**: Hierarchical Task Network planning
- **Dynamic Action Generation**: Create actions at runtime
- **Multi-Agent Coordination**: GOAP plans for agent swarms
- **Plan Caching**: LRU cache for frequently used plans
- **Partial Replanning**: Only replan affected portion
- **Goal Templates**: Reusable goal patterns

## References

- F.E.A.R. AI Architecture (Monolith Productions)
- "Goal-Oriented Action Planning" by Jeff Orkin (MIT)
- "AI Game Programming Wisdom" series
- ARCADIA Neo-Cortex integration patterns

## See Also

- [Neo-Cortex Reasoning](AI_SYSTEMS.md#neo-cortex)
- [Integrated AI System](AI_SYSTEMS.md#integrated-system)
- [Performance Benchmarks](../testing/TESTING.md#benchmarks)
- [Examples](../../examples/goap_npc_behavior.rs)

---

**Implementation**: `src/ai/goap.rs` (544 lines)
**Tests**: `src/ai/goap.rs::tests` (6 test cases)
**Benchmarks**: `benches/goap_benchmark.rs` (238 lines, 5 scenarios)
**Examples**: `examples/goap_npc_behavior.rs` (338 lines, 4 scenarios)
