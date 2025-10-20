//! # GOAP (Goal-Oriented Action Planning)
//!
//! Production-ready GOAP system for autonomous agent behavior planning.
//! Inspired by the F.E.A.R. game engine and integrated with ARCADIA's AI systems.
//!
//! ## Features
//! - A* pathfinding for optimal action sequences
//! - Dynamic world state management
//! - Flexible action preconditions and effects
//! - Priority-based goal selection
//! - Integration with Neo-Cortex reasoning
//! - Cost-based optimization
//! - Backtracking and replanning support

use std::collections::{HashMap, HashSet, BinaryHeap, VecDeque};
use std::cmp::Ordering;
use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};

/// World state key-value pair representing game world conditions
pub type WorldState = HashMap<String, StateValue>;

/// State value types supporting various data representations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StateValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

impl StateValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            StateValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            StateValue::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            StateValue::Float(f) => Some(*f),
            _ => None,
        }
    }
}

/// GOAP action representing a single operation that changes world state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoapAction {
    pub name: String,
    pub cost: f64,
    pub preconditions: WorldState,
    pub effects: WorldState,
    pub metadata: HashMap<String, String>,
}

impl GoapAction {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            cost: 1.0,
            preconditions: HashMap::new(),
            effects: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_cost(mut self, cost: f64) -> Self {
        self.cost = cost;
        self
    }

    pub fn with_precondition(mut self, key: impl Into<String>, value: StateValue) -> Self {
        self.preconditions.insert(key.into(), value);
        self
    }

    pub fn with_effect(mut self, key: impl Into<String>, value: StateValue) -> Self {
        self.effects.insert(key.into(), value);
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Check if this action's preconditions are met in the given world state
    pub fn is_valid(&self, world_state: &WorldState) -> bool {
        self.preconditions.iter().all(|(key, value)| {
            world_state.get(key).map_or(false, |state_val| state_val == value)
        })
    }

    /// Apply this action's effects to a world state, returning the new state
    pub fn apply(&self, world_state: &WorldState) -> WorldState {
        let mut new_state = world_state.clone();
        for (key, value) in &self.effects {
            new_state.insert(key.clone(), value.clone());
        }
        new_state
    }
}

/// GOAP goal representing a desired world state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoapGoal {
    pub name: String,
    pub priority: f64,
    pub desired_state: WorldState,
    pub metadata: HashMap<String, String>,
}

impl GoapGoal {
    pub fn new(name: impl Into<String>, priority: f64) -> Self {
        Self {
            name: name.into(),
            priority,
            desired_state: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_condition(mut self, key: impl Into<String>, value: StateValue) -> Self {
        self.desired_state.insert(key.into(), value);
        self
    }

    /// Check if this goal is satisfied in the given world state
    pub fn is_satisfied(&self, world_state: &WorldState) -> bool {
        self.desired_state.iter().all(|(key, value)| {
            world_state.get(key).map_or(false, |state_val| state_val == value)
        })
    }

    /// Calculate heuristic distance to goal (number of unsatisfied conditions)
    pub fn heuristic(&self, world_state: &WorldState) -> f64 {
        self.desired_state.iter()
            .filter(|(key, value)| {
                !world_state.get(*key).map_or(false, |state_val| state_val == *value)
            })
            .count() as f64
    }
}

/// Node in the GOAP planning graph
#[derive(Debug, Clone)]
struct PlanNode {
    world_state: WorldState,
    action: Option<GoapAction>,
    cost: f64,
    heuristic: f64,
    parent: Option<Box<PlanNode>>,
}

impl PlanNode {
    fn total_cost(&self) -> f64 {
        self.cost + self.heuristic
    }
}

impl PartialEq for PlanNode {
    fn eq(&self, other: &Self) -> bool {
        self.total_cost() == other.total_cost()
    }
}

impl Eq for PlanNode {}

impl PartialOrd for PlanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PlanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap behavior
        other.total_cost().partial_cmp(&self.total_cost()).unwrap_or(Ordering::Equal)
    }
}

/// GOAP planner result containing action sequence
#[derive(Debug, Clone)]
pub struct GoapPlan {
    pub actions: Vec<GoapAction>,
    pub total_cost: f64,
    pub goal_name: String,
}

impl GoapPlan {
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    pub fn len(&self) -> usize {
        self.actions.len()
    }
}

/// Statistics for GOAP planning process
#[derive(Debug, Clone, Default)]
pub struct PlanningStats {
    pub nodes_explored: usize,
    pub planning_time_ms: u128,
    pub plan_length: usize,
    pub total_cost: f64,
}

/// GOAP planner using A* pathfinding
pub struct GoapPlanner {
    actions: Arc<RwLock<Vec<GoapAction>>>,
    goals: Arc<RwLock<Vec<GoapGoal>>>,
    current_world_state: Arc<RwLock<WorldState>>,
    max_iterations: usize,
    stats: Arc<RwLock<PlanningStats>>,
}

impl GoapPlanner {
    pub fn new() -> Self {
        Self {
            actions: Arc::new(RwLock::new(Vec::new())),
            goals: Arc::new(RwLock::new(Vec::new())),
            current_world_state: Arc::new(RwLock::new(HashMap::new())),
            max_iterations: 1000,
            stats: Arc::new(RwLock::new(PlanningStats::default())),
        }
    }

    pub fn with_max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    /// Register a new action
    pub fn add_action(&self, action: GoapAction) {
        self.actions.write().unwrap().push(action);
    }

    /// Register multiple actions
    pub fn add_actions(&self, actions: Vec<GoapAction>) {
        self.actions.write().unwrap().extend(actions);
    }

    /// Register a new goal
    pub fn add_goal(&self, goal: GoapGoal) {
        self.goals.write().unwrap().push(goal);
    }

    /// Update world state
    pub fn set_world_state(&self, state: WorldState) {
        *self.current_world_state.write().unwrap() = state;
    }

    /// Update a specific world state value
    pub fn update_state(&self, key: String, value: StateValue) {
        self.current_world_state.write().unwrap().insert(key, value);
    }

    /// Get current world state
    pub fn get_world_state(&self) -> WorldState {
        self.current_world_state.read().unwrap().clone()
    }

    /// Get planning statistics
    pub fn get_stats(&self) -> PlanningStats {
        self.stats.read().unwrap().clone()
    }

    /// Select the highest priority achievable goal
    pub fn select_goal(&self) -> Option<GoapGoal> {
        let goals = self.goals.read().unwrap();
        let world_state = self.current_world_state.read().unwrap();
        
        goals.iter()
            .filter(|goal| !goal.is_satisfied(&world_state))
            .max_by(|a, b| a.priority.partial_cmp(&b.priority).unwrap_or(Ordering::Equal))
            .cloned()
    }

    /// Plan action sequence to achieve a goal using A* pathfinding
    pub fn plan(&self, goal: &GoapGoal) -> Option<GoapPlan> {
        let start_time = std::time::Instant::now();
        let mut stats = PlanningStats::default();

        let world_state = self.current_world_state.read().unwrap().clone();
        
        // Early exit if goal already satisfied
        if goal.is_satisfied(&world_state) {
            return Some(GoapPlan {
                actions: Vec::new(),
                total_cost: 0.0,
                goal_name: goal.name.clone(),
            });
        }

        let actions = self.actions.read().unwrap().clone();
        
        // Priority queue for A* (min-heap by total cost)
        let mut open_set = BinaryHeap::new();
        let mut closed_set = HashSet::new();

        let start_node = PlanNode {
            world_state: world_state.clone(),
            action: None,
            cost: 0.0,
            heuristic: goal.heuristic(&world_state),
            parent: None,
        };

        open_set.push(start_node);

        let mut iterations = 0;
        while let Some(current) = open_set.pop() {
            iterations += 1;
            stats.nodes_explored = iterations;

            if iterations > self.max_iterations {
                break;
            }

            // Check if goal reached
            if goal.is_satisfied(&current.world_state) {
                let plan = self.reconstruct_plan(current, goal.name.clone());
                stats.planning_time_ms = start_time.elapsed().as_millis();
                stats.plan_length = plan.actions.len();
                stats.total_cost = plan.total_cost;
                *self.stats.write().unwrap() = stats;
                return Some(plan);
            }

            // Generate state hash for closed set
            let state_hash = self.hash_world_state(&current.world_state);
            if closed_set.contains(&state_hash) {
                continue;
            }
            closed_set.insert(state_hash);

            // Explore neighbors (apply valid actions)
            for action in &actions {
                if action.is_valid(&current.world_state) {
                    let new_state = action.apply(&current.world_state);
                    let new_cost = current.cost + action.cost;
                    let new_heuristic = goal.heuristic(&new_state);

                    let new_node = PlanNode {
                        world_state: new_state,
                        action: Some(action.clone()),
                        cost: new_cost,
                        heuristic: new_heuristic,
                        parent: Some(Box::new(current.clone())),
                    };

                    open_set.push(new_node);
                }
            }
        }

        // No plan found
        stats.planning_time_ms = start_time.elapsed().as_millis();
        *self.stats.write().unwrap() = stats;
        None
    }

    /// Plan for the highest priority goal
    pub fn plan_best(&self) -> Option<GoapPlan> {
        let goal = self.select_goal()?;
        self.plan(&goal)
    }

    /// Reconstruct action sequence from goal node
    fn reconstruct_plan(&self, mut node: PlanNode, goal_name: String) -> GoapPlan {
        let mut actions = VecDeque::new();
        let total_cost = node.cost;

        while let Some(action) = node.action {
            actions.push_front(action);
            if let Some(parent) = node.parent {
                node = *parent;
            } else {
                break;
            }
        }

        GoapPlan {
            actions: actions.into_iter().collect(),
            total_cost,
            goal_name,
        }
    }

    /// Hash world state for closed set detection
    fn hash_world_state(&self, state: &WorldState) -> String {
        use std::collections::BTreeMap;
        let sorted: BTreeMap<_, _> = state.iter().collect();
        format!("{:?}", sorted)
    }

    /// Clear all actions, goals, and reset state
    pub fn reset(&self) {
        self.actions.write().unwrap().clear();
        self.goals.write().unwrap().clear();
        self.current_world_state.write().unwrap().clear();
        *self.stats.write().unwrap() = PlanningStats::default();
    }
}

impl Default for GoapPlanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_preconditions() {
        let action = GoapAction::new("test")
            .with_precondition("has_weapon", StateValue::Bool(true));

        let mut state = HashMap::new();
        state.insert("has_weapon".to_string(), StateValue::Bool(true));
        assert!(action.is_valid(&state));

        state.insert("has_weapon".to_string(), StateValue::Bool(false));
        assert!(!action.is_valid(&state));
    }

    #[test]
    fn test_action_effects() {
        let action = GoapAction::new("pickup_weapon")
            .with_effect("has_weapon", StateValue::Bool(true));

        let state = HashMap::new();
        let new_state = action.apply(&state);
        
        assert_eq!(
            new_state.get("has_weapon").unwrap().as_bool().unwrap(),
            true
        );
    }

    #[test]
    fn test_goal_satisfaction() {
        let goal = GoapGoal::new("be_armed", 1.0)
            .with_condition("has_weapon", StateValue::Bool(true));

        let mut state = HashMap::new();
        assert!(!goal.is_satisfied(&state));

        state.insert("has_weapon".to_string(), StateValue::Bool(true));
        assert!(goal.is_satisfied(&state));
    }

    #[test]
    fn test_simple_planning() {
        let planner = GoapPlanner::new();

        // Define actions
        let pickup = GoapAction::new("pickup_weapon")
            .with_cost(1.0)
            .with_effect("has_weapon", StateValue::Bool(true));

        planner.add_action(pickup);

        // Define goal
        let goal = GoapGoal::new("be_armed", 1.0)
            .with_condition("has_weapon", StateValue::Bool(true));

        // Plan
        let plan = planner.plan(&goal).expect("Should find plan");
        
        assert_eq!(plan.actions.len(), 1);
        assert_eq!(plan.actions[0].name, "pickup_weapon");
    }

    #[test]
    fn test_multi_step_planning() {
        let planner = GoapPlanner::new();

        // Actions: go to armory -> unlock door -> pickup weapon
        let go_to_armory = GoapAction::new("go_to_armory")
            .with_cost(2.0)
            .with_effect("at_armory", StateValue::Bool(true));

        let unlock_door = GoapAction::new("unlock_door")
            .with_cost(1.0)
            .with_precondition("at_armory", StateValue::Bool(true))
            .with_precondition("has_key", StateValue::Bool(true))
            .with_effect("door_unlocked", StateValue::Bool(true));

        let pickup = GoapAction::new("pickup_weapon")
            .with_cost(1.0)
            .with_precondition("at_armory", StateValue::Bool(true))
            .with_precondition("door_unlocked", StateValue::Bool(true))
            .with_effect("has_weapon", StateValue::Bool(true));

        planner.add_actions(vec![go_to_armory, unlock_door, pickup]);

        // Set initial state
        let mut initial_state = HashMap::new();
        initial_state.insert("has_key".to_string(), StateValue::Bool(true));
        planner.set_world_state(initial_state);

        // Define goal
        let goal = GoapGoal::new("be_armed", 1.0)
            .with_condition("has_weapon", StateValue::Bool(true));

        // Plan
        let plan = planner.plan(&goal).expect("Should find plan");
        
        assert_eq!(plan.actions.len(), 3);
        assert_eq!(plan.actions[0].name, "go_to_armory");
        assert_eq!(plan.actions[1].name, "unlock_door");
        assert_eq!(plan.actions[2].name, "pickup_weapon");
    }

    #[test]
    fn test_goal_selection() {
        let planner = GoapPlanner::new();

        let low_priority = GoapGoal::new("low", 0.5)
            .with_condition("state_a", StateValue::Bool(true));

        let high_priority = GoapGoal::new("high", 0.9)
            .with_condition("state_b", StateValue::Bool(true));

        planner.add_goal(low_priority);
        planner.add_goal(high_priority);

        let selected = planner.select_goal().unwrap();
        assert_eq!(selected.name, "high");
    }
}
