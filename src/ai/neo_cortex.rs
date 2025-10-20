//! Neo-Cortex Higher-Order Reasoning Module
//!
//! This module implements sophisticated decision-making, problem-solving,
//! and strategic planning capabilities for AI-driven NPCs and game systems.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use parking_lot::RwLock;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Represents different cognitive processing levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CognitiveLevel {
    /// Reactive - immediate responses
    Reactive,
    /// Tactical - short-term planning
    Tactical,
    /// Strategic - long-term planning
    Strategic,
    /// Abstract - conceptual reasoning
    Abstract,
}

/// Represents a decision context with all relevant information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionContext {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub situation: String,
    pub goals: Vec<String>,
    pub constraints: Vec<String>,
    pub available_actions: Vec<String>,
    pub world_state: HashMap<String, f64>,
    pub priority: f64,
}

/// Represents a decision outcome with reasoning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub id: Uuid,
    pub context_id: Uuid,
    pub action: String,
    pub confidence: f64,
    pub reasoning: String,
    pub expected_utility: f64,
    pub timestamp: DateTime<Utc>,
}

/// Strategy for problem-solving
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemSolvingStrategy {
    pub name: String,
    pub steps: Vec<String>,
    pub success_rate: f64,
    pub adaptations: Vec<String>,
}

/// Memory of past decisions and their outcomes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionMemory {
    pub decision: Decision,
    pub actual_outcome: f64,
    pub success: bool,
    pub lessons_learned: Vec<String>,
}

/// Neo-cortex higher-order reasoning system
pub struct NeoCortexReasoning {
    /// Current cognitive processing level
    cognitive_level: Arc<RwLock<CognitiveLevel>>,

    /// Decision history for learning
    decision_history: Arc<RwLock<VecDeque<DecisionMemory>>>,

    /// Active strategies for problem-solving
    strategies: Arc<RwLock<HashMap<String, ProblemSolvingStrategy>>>,

    /// Goal hierarchy (higher values = higher priority)
    goals: Arc<RwLock<HashMap<String, f64>>>,

    /// Knowledge base for context-aware reasoning
    knowledge_base: Arc<RwLock<HashMap<String, Vec<String>>>>,

    /// Maximum decision history to maintain
    max_history_size: usize,
}

impl NeoCortexReasoning {
    /// Create a new neo-cortex reasoning system
    pub fn new() -> Self {
        Self {
            cognitive_level: Arc::new(RwLock::new(CognitiveLevel::Tactical)),
            decision_history: Arc::new(RwLock::new(VecDeque::new())),
            strategies: Arc::new(RwLock::new(HashMap::new())),
            goals: Arc::new(RwLock::new(HashMap::new())),
            knowledge_base: Arc::new(RwLock::new(HashMap::new())),
            max_history_size: 1000,
        }
    }

    /// Set the cognitive processing level
    pub fn set_cognitive_level(&self, level: CognitiveLevel) {
        *self.cognitive_level.write() = level;
    }

    /// Get current cognitive level
    pub fn get_cognitive_level(&self) -> CognitiveLevel {
        *self.cognitive_level.read()
    }

    /// Add a goal with priority
    pub fn add_goal(&self, goal: String, priority: f64) {
        self.goals.write().insert(goal, priority);
    }

    /// Remove a goal
    pub fn remove_goal(&self, goal: &str) -> Option<f64> {
        self.goals.write().remove(goal)
    }

    /// Update goal priority
    pub fn update_goal_priority(&self, goal: &str, new_priority: f64) -> bool {
        if let Some(priority) = self.goals.write().get_mut(goal) {
            *priority = new_priority;
            true
        } else {
            false
        }
    }

    /// Add knowledge to the knowledge base
    pub fn add_knowledge(&self, category: String, facts: Vec<String>) {
        self.knowledge_base.write()
            .entry(category)
            .or_insert_with(Vec::new)
            .extend(facts);
    }

    /// Register a problem-solving strategy
    pub fn register_strategy(&self, strategy: ProblemSolvingStrategy) {
        self.strategies.write().insert(strategy.name.clone(), strategy);
    }

    /// Make a decision based on context using multi-criteria analysis
    pub fn make_decision(&self, context: DecisionContext) -> Decision {
        let cognitive_level = *self.cognitive_level.read();

        // Evaluate each action based on current goals and context
        let mut action_scores: Vec<(String, f64, String)> = context.available_actions.iter()
            .map(|action| {
                let (score, reasoning) = self.evaluate_action(action, &context, cognitive_level);
                (action.clone(), score, reasoning)
            })
            .collect();

        // Sort by score (descending)
        action_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Select the best action
        let (best_action, score, reasoning) = action_scores.first()
            .cloned()
            .unwrap_or_else(|| ("wait".to_string(), 0.0, "No suitable action found".to_string()));

        let decision = Decision {
            id: Uuid::new_v4(),
            context_id: context.id,
            action: best_action,
            confidence: self.calculate_confidence(score, &context),
            reasoning,
            expected_utility: score,
            timestamp: Utc::now(),
        };

        decision
    }

    /// Evaluate an action's utility given the context
    fn evaluate_action(&self, action: &str, context: &DecisionContext, level: CognitiveLevel) -> (f64, String) {
        let goals = self.goals.read();
        let mut total_score = 0.0;
        let mut reasoning_parts = Vec::new();

        // Match action against active goals
        for (goal, priority) in goals.iter() {
            if self.action_supports_goal(action, goal, context) {
                let goal_contribution = priority * 0.5;
                total_score += goal_contribution;
                reasoning_parts.push(format!("Supports goal '{}' (+{:.2})", goal, goal_contribution));
            }
        }

        // Apply cognitive level modifiers
        match level {
            CognitiveLevel::Reactive => {
                // Favor immediate actions
                if action.contains("immediate") || action.contains("quick") {
                    total_score *= 1.2;
                    reasoning_parts.push("Reactive bonus applied".to_string());
                }
            }
            CognitiveLevel::Tactical => {
                // Consider short-term consequences
                let short_term_benefit = self.estimate_short_term_benefit(action, context);
                total_score += short_term_benefit;
                reasoning_parts.push(format!("Tactical benefit: +{:.2}", short_term_benefit));
            }
            CognitiveLevel::Strategic => {
                // Consider long-term consequences
                let long_term_benefit = self.estimate_long_term_benefit(action, context);
                total_score += long_term_benefit;
                reasoning_parts.push(format!("Strategic benefit: +{:.2}", long_term_benefit));
            }
            CognitiveLevel::Abstract => {
                // Consider abstract patterns and principles
                let pattern_match = self.match_historical_patterns(action);
                total_score += pattern_match;
                reasoning_parts.push(format!("Pattern matching: +{:.2}", pattern_match));
            }
        }

        // Apply constraint penalties
        for constraint in &context.constraints {
            if self.action_violates_constraint(action, constraint) {
                total_score *= 0.5;
                reasoning_parts.push(format!("Violates constraint '{}' (-50%)", constraint));
            }
        }

        // Factor in context priority
        total_score *= context.priority;

        let reasoning = reasoning_parts.join("; ");
        (total_score, reasoning)
    }

    /// Check if an action supports a given goal
    fn action_supports_goal(&self, action: &str, goal: &str, context: &DecisionContext) -> bool {
        // Simple heuristic: check if goal keywords appear in action
        let goal_lower = goal.to_lowercase();
        let action_lower = action.to_lowercase();

        goal_lower.split_whitespace().any(|word| action_lower.contains(word))
            || context.goals.iter().any(|g| {
                let g_lower = g.to_lowercase();
                action_lower.contains(&g_lower) || g_lower.contains(&action_lower)
            })
    }

    /// Check if action violates a constraint
    fn action_violates_constraint(&self, action: &str, constraint: &str) -> bool {
        // Check for explicit violations
        let action_lower = action.to_lowercase();
        let constraint_lower = constraint.to_lowercase();

        if constraint_lower.starts_with("no ") || constraint_lower.starts_with("avoid ") {
            let forbidden = constraint_lower
                .trim_start_matches("no ")
                .trim_start_matches("avoid ");
            action_lower.contains(forbidden)
        } else {
            false
        }
    }

    /// Estimate short-term benefit of an action
    fn estimate_short_term_benefit(&self, action: &str, context: &DecisionContext) -> f64 {
        // Use world state to estimate immediate impact
        let mut benefit = 0.0;

        // Check historical success of similar actions
        let history = self.decision_history.read();
        let similar_decisions = history.iter()
            .filter(|mem| mem.decision.action.contains(action) || action.contains(&mem.decision.action))
            .take(10);

        let mut count = 0;
        for mem in similar_decisions {
            benefit += if mem.success { 0.1 } else { -0.05 };
            count += 1;
        }

        if count > 0 {
            benefit
        } else {
            0.0
        }
    }

    /// Estimate long-term benefit of an action
    fn estimate_long_term_benefit(&self, action: &str, _context: &DecisionContext) -> f64 {
        // Look at longer-term patterns in decision history
        let history = self.decision_history.read();
        let similar_sequences = history.iter()
            .filter(|mem| mem.decision.action.contains(action))
            .take(20);

        let mut total_outcome = 0.0;
        let mut count = 0;

        for mem in similar_sequences {
            total_outcome += mem.actual_outcome;
            count += 1;
        }

        if count > 0 {
            total_outcome / count as f64
        } else {
            0.0
        }
    }

    /// Match current situation against historical patterns
    fn match_historical_patterns(&self, action: &str) -> f64 {
        let history = self.decision_history.read();
        let strategies = self.strategies.read();

        // Check if action aligns with successful strategies
        let mut pattern_score = 0.0;

        for strategy in strategies.values() {
            if strategy.steps.iter().any(|step| step.contains(action)) {
                pattern_score += strategy.success_rate * 0.2;
            }
        }

        // Boost for actions that led to success in the past
        let success_rate = history.iter()
            .filter(|mem| mem.decision.action == action)
            .map(|mem| if mem.success { 1.0 } else { 0.0 })
            .sum::<f64>() / (history.len() as f64).max(1.0);

        pattern_score += success_rate * 0.3;
        pattern_score
    }

    /// Calculate confidence in a decision
    fn calculate_confidence(&self, score: f64, context: &DecisionContext) -> f64 {
        let base_confidence = (score / (context.priority + 1.0)).min(1.0).max(0.0);

        // Adjust based on available information
        let info_completeness = context.world_state.len() as f64 / 10.0;
        let info_factor = (1.0 + info_completeness.min(1.0)) / 2.0;

        base_confidence * info_factor
    }

    /// Record the outcome of a decision for learning
    pub fn record_outcome(&self, decision: Decision, actual_outcome: f64, success: bool) {
        let lessons = self.extract_lessons(&decision, actual_outcome, success);

        let memory = DecisionMemory {
            decision,
            actual_outcome,
            success,
            lessons_learned: lessons,
        };

        let mut history = self.decision_history.write();
        history.push_back(memory);

        // Maintain maximum history size
        while history.len() > self.max_history_size {
            history.pop_front();
        }
    }

    /// Extract lessons from decision outcomes
    fn extract_lessons(&self, decision: &Decision, outcome: f64, success: bool) -> Vec<String> {
        let mut lessons = Vec::new();

        if success {
            lessons.push(format!("Action '{}' was effective (outcome: {:.2})", decision.action, outcome));

            if decision.confidence > 0.8 {
                lessons.push("High confidence decisions continue to perform well".to_string());
            }
        } else {
            lessons.push(format!("Action '{}' failed (outcome: {:.2})", decision.action, outcome));

            if decision.confidence > 0.7 {
                lessons.push("Need to recalibrate confidence metrics".to_string());
            }
        }

        // Check for unexpected outcomes
        let expectation_error = (outcome - decision.expected_utility).abs();
        if expectation_error > 0.5 {
            lessons.push(format!("Large expectation error ({:.2}), update utility estimation", expectation_error));
        }

        lessons
    }

    /// Perform strategic planning for complex goals
    pub fn plan_strategy(&self, goal: String, time_horizon: usize) -> ProblemSolvingStrategy {
        let mut steps = Vec::new();
        let knowledge = self.knowledge_base.read();

        // Break down the goal into sub-goals
        let sub_goals = self.decompose_goal(&goal);

        for (i, sub_goal) in sub_goals.iter().enumerate() {
            steps.push(format!("Step {}: {}", i + 1, sub_goal));

            // Add context-specific actions from knowledge base
            if let Some(related_facts) = knowledge.get(sub_goal) {
                for fact in related_facts.iter().take(2) {
                    steps.push(format!("  - Apply: {}", fact));
                }
            }
        }

        // Estimate success rate based on historical performance
        let success_rate = self.estimate_strategy_success(&goal);

        ProblemSolvingStrategy {
            name: goal.clone(),
            steps,
            success_rate,
            adaptations: vec![],
        }
    }

    /// Decompose a complex goal into sub-goals
    fn decompose_goal(&self, goal: &str) -> Vec<String> {
        // Simple heuristic decomposition
        let mut sub_goals = Vec::new();

        sub_goals.push(format!("Analyze requirements for '{}'", goal));
        sub_goals.push(format!("Gather resources for '{}'", goal));
        sub_goals.push(format!("Execute main action for '{}'", goal));
        sub_goals.push(format!("Verify achievement of '{}'", goal));

        sub_goals
    }

    /// Estimate the success rate of a strategy
    fn estimate_strategy_success(&self, goal: &str) -> f64 {
        let history = self.decision_history.read();

        let relevant_memories: Vec<_> = history.iter()
            .filter(|mem| mem.decision.reasoning.contains(goal))
            .collect();

        if relevant_memories.is_empty() {
            return 0.5; // Default moderate success rate
        }

        let successes = relevant_memories.iter()
            .filter(|mem| mem.success)
            .count();

        successes as f64 / relevant_memories.len() as f64
    }

    /// Adapt existing strategy based on new information
    pub fn adapt_strategy(&self, strategy_name: &str, adaptation: String) {
        if let Some(strategy) = self.strategies.write().get_mut(strategy_name) {
            strategy.adaptations.push(adaptation);

            // Update success rate based on recent performance
            let recent_success = self.estimate_strategy_success(&strategy.name);
            strategy.success_rate = (strategy.success_rate * 0.7) + (recent_success * 0.3);
        }
    }

    /// Get all current goals sorted by priority
    pub fn get_goals_by_priority(&self) -> Vec<(String, f64)> {
        let goals = self.goals.read();
        let mut goal_list: Vec<_> = goals.iter()
            .map(|(g, p)| (g.clone(), *p))
            .collect();

        goal_list.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        goal_list
    }

    /// Analyze context to determine optimal cognitive level
    pub fn determine_optimal_cognitive_level(&self, context: &DecisionContext) -> CognitiveLevel {
        // High priority situations may require different cognitive approaches
        if context.priority > 0.9 {
            CognitiveLevel::Strategic
        } else if context.constraints.len() > 3 {
            CognitiveLevel::Abstract
        } else if context.available_actions.len() <= 2 {
            CognitiveLevel::Reactive
        } else {
            CognitiveLevel::Tactical
        }
    }
}

impl Default for NeoCortexReasoning {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neo_cortex_creation() {
        let neo_cortex = NeoCortexReasoning::new();
        assert_eq!(neo_cortex.get_cognitive_level(), CognitiveLevel::Tactical);
    }

    #[test]
    fn test_goal_management() {
        let neo_cortex = NeoCortexReasoning::new();
        neo_cortex.add_goal("Survive".to_string(), 0.9);
        neo_cortex.add_goal("Explore".to_string(), 0.5);

        let goals = neo_cortex.get_goals_by_priority();
        assert_eq!(goals.len(), 2);
        assert_eq!(goals[0].0, "Survive");
        assert!(goals[0].1 > goals[1].1);
    }

    #[test]
    fn test_decision_making() {
        let neo_cortex = NeoCortexReasoning::new();
        neo_cortex.add_goal("Defend territory".to_string(), 0.8);

        let context = DecisionContext {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            situation: "Enemy approaching".to_string(),
            goals: vec!["Defend territory".to_string()],
            constraints: vec![],
            available_actions: vec![
                "Attack enemy".to_string(),
                "Retreat".to_string(),
                "Call for help".to_string(),
            ],
            world_state: HashMap::new(),
            priority: 0.8,
        };

        let decision = neo_cortex.make_decision(context);
        assert!(!decision.action.is_empty());
        assert!(decision.confidence >= 0.0 && decision.confidence <= 1.0);
    }

    #[test]
    fn test_strategy_planning() {
        let neo_cortex = NeoCortexReasoning::new();
        let strategy = neo_cortex.plan_strategy("Conquer region".to_string(), 10);

        assert_eq!(strategy.name, "Conquer region");
        assert!(!strategy.steps.is_empty());
        assert!(strategy.success_rate >= 0.0 && strategy.success_rate <= 1.0);
    }
}
