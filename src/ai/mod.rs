//! AI Systems Module
//!
//! This module provides all AI-driven adaptive systems for ARCADIA including:
//! - Neo-cortex higher-order reasoning
//! - Autopoetic processing (self-organization)
//! - Evolutionary feedback systems
//! - Self-awareness and adaptive perspectives
//! - Emotion-adaptive experiences
//! - Symbolic/sub-symbolic computing

pub mod neo_cortex;
pub mod autopoetic;
pub mod evolutionary;
pub mod self_awareness;
pub mod emotion;
pub mod symbolic;
pub mod goap;

// Re-export main types for convenience
pub use neo_cortex::{
    NeoCortexReasoning,
    CognitiveLevel,
    DecisionContext,
    Decision,
    ProblemSolvingStrategy,
};

pub use autopoetic::{
    AutopoeticProcessing,
    HealthStatus,
    MaintenanceType,
    EmergentPattern,
    SystemComponent,
};

pub use evolutionary::{
    EvolutionaryFeedback,
    Genome,
    Gene,
    EvolutionConfig,
    FitnessCriteria,
    EvolvedTrait,
};

pub use self_awareness::{
    SelfAwareness,
    EntityRole,
    ConsciousnessState,
    StateSnapshot,
    Belief,
    MetaThought,
    MetaThoughtType,
};

pub use emotion::{
    EmotionAdaptiveExperiences,
    EmotionalState,
    EmotionalProfile,
    ArousalLevel,
    EnvironmentalFactor,
    DifficultySettings,
};

pub use symbolic::{
    SymbolicComputing,
    Concept,
    ConceptValue,
    Relationship,
    RelationType,
    InferenceRule,
    Query,
    QueryResult,
};

pub use goap::{
    GoapPlanner,
    GoapAction,
    GoapGoal,
    GoapPlan,
    WorldState,
    StateValue,
    PlanningStats,
};

use uuid::Uuid;
use std::sync::Arc;
use parking_lot::RwLock;

/// Integrated AI system that combines all AI modules
pub struct IntegratedAISystem {
    /// Neo-cortex reasoning system
    pub neo_cortex: Arc<NeoCortexReasoning>,

    /// Autopoetic processing system
    pub autopoetic: Arc<AutopoeticProcessing>,

    /// Evolutionary feedback system
    pub evolutionary: Arc<EvolutionaryFeedback>,

    /// Self-awareness system
    pub self_awareness: Arc<RwLock<SelfAwareness>>,

    /// Emotion-adaptive system
    pub emotion: Arc<EmotionAdaptiveExperiences>,

    /// Symbolic computing system
    pub symbolic: Arc<SymbolicComputing>,

    /// GOAP planner for goal-oriented action planning
    pub goap: Arc<GoapPlanner>,
}

impl IntegratedAISystem {
    /// Create a new integrated AI system
    pub fn new(entity_id: Uuid, entity_name: String) -> Self {
        let evolution_config = EvolutionConfig {
            population_size: 100,
            mutation_rate: 0.01,
            crossover_rate: 0.7,
            elitism_count: 10,
            max_generations: 1000,
        };

        Self {
            neo_cortex: Arc::new(NeoCortexReasoning::new()),
            autopoetic: Arc::new(AutopoeticProcessing::new()),
            evolutionary: Arc::new(EvolutionaryFeedback::new(evolution_config)),
            self_awareness: Arc::new(RwLock::new(SelfAwareness::new(entity_id, entity_name))),
            emotion: Arc::new(EmotionAdaptiveExperiences::new()),
            symbolic: Arc::new(SymbolicComputing::new()),
            goap: Arc::new(GoapPlanner::new().with_max_iterations(1000)),
        }
    }

    /// Update all AI systems in a single tick
    pub fn update(&self, delta_time: f64) {
        // Simulate system degradation
        self.autopoetic.simulate_degradation(delta_time);

        // Perform self-maintenance
        self.autopoetic.perform_self_maintenance();

        // Detect emergent patterns
        self.autopoetic.detect_emergent_patterns();

        // Self-organize
        self.autopoetic.self_organize();
    }

    /// Make a decision using integrated AI systems
    pub fn make_integrated_decision(&self, context: DecisionContext) -> Decision {
        // Use neo-cortex for primary decision making
        let decision = self.neo_cortex.make_decision(context.clone());

        // Record decision for evolutionary learning
        use std::collections::HashMap;
        let mut interaction_context = HashMap::new();
        interaction_context.insert("priority".to_string(), context.priority);

        self.evolutionary.record_interaction(
            self.self_awareness.read().get_entity_id(),
            decision.action.clone(),
            interaction_context,
            decision.confidence,
        );

        decision
    }

    /// Get system health status
    pub fn get_system_health(&self) -> HealthStatus {
        self.autopoetic.get_health_status()
    }

    /// Get self-awareness level
    pub fn get_awareness_level(&self) -> f64 {
        self.self_awareness.read().assess_self_awareness_level()
    }

    /// Get entity description
    pub fn describe_entity(&self) -> String {
        self.self_awareness.read().describe_self()
    }
}

impl Default for IntegratedAISystem {
    fn default() -> Self {
        Self::new(Uuid::new_v4(), "DefaultEntity".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integrated_system_creation() {
        let system = IntegratedAISystem::new(Uuid::new_v4(), "TestEntity".to_string());
        assert_eq!(system.self_awareness.read().get_entity_name(), "TestEntity");
    }

    #[test]
    fn test_system_update() {
        let system = IntegratedAISystem::new(Uuid::new_v4(), "TestEntity".to_string());

        // Add a component to autopoetic system
        system.autopoetic.add_component("TestComponent".to_string(), vec![]);

        // Update system
        system.update(1.0);

        // Check health
        let health = system.get_system_health();
        assert!(matches!(
            health,
            HealthStatus::Optimal | HealthStatus::Healthy | HealthStatus::Degraded
        ));
    }

    #[test]
    fn test_integrated_decision() {
        use std::collections::HashMap;

        let system = IntegratedAISystem::new(Uuid::new_v4(), "DecisionMaker".to_string());

        let context = DecisionContext {
            id: Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            situation: "Test situation".to_string(),
            goals: vec!["Survive".to_string()],
            constraints: vec![],
            available_actions: vec!["Action A".to_string(), "Action B".to_string()],
            world_state: HashMap::new(),
            priority: 0.8,
        };

        let decision = system.make_integrated_decision(context);
        assert!(!decision.action.is_empty());
        assert!(decision.confidence >= 0.0 && decision.confidence <= 1.0);
    }
}
