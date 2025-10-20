//! AI Systems module for ARCADIA
//! 
//! Contains all advanced AI subsystems including:
//! - Neo-cortex higher-order reasoning
//! - Autopoetic processing
//! - Evolutionary feedback
//! - Self-awareness
//! - Adaptive perspectives
//! - Emotion-adaptive experiences

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Neo-cortex higher-order reasoning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoCortexReasoning {
    decision_tree: Vec<Decision>,
    memory_bank: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub id: String,
    pub context: String,
    pub outcome: f32,
}

impl NeoCortexReasoning {
    pub fn new() -> Self {
        NeoCortexReasoning {
            decision_tree: Vec::new(),
            memory_bank: HashMap::new(),
        }
    }

    pub fn make_decision(&mut self, context: &str) -> f32 {
        // Simplified decision-making based on past experiences
        let score = self.memory_bank.get(context).copied().unwrap_or(0.5);
        
        let decision = Decision {
            id: uuid::Uuid::new_v4().to_string(),
            context: context.to_string(),
            outcome: score,
        };
        
        self.decision_tree.push(decision);
        score
    }

    pub fn learn(&mut self, context: &str, outcome: f32) {
        self.memory_bank.insert(context.to_string(), outcome);
    }

    pub fn decision_count(&self) -> usize {
        self.decision_tree.len()
    }
}

impl Default for NeoCortexReasoning {
    fn default() -> Self {
        Self::new()
    }
}

// Autopoetic processing - self-organization and self-maintenance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutopoeticProcessing {
    components: HashMap<String, Component>,
    integrity_threshold: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub id: String,
    pub health: f32,
    pub last_maintenance: chrono::DateTime<chrono::Utc>,
}

impl AutopoeticProcessing {
    pub fn new(integrity_threshold: f32) -> Self {
        AutopoeticProcessing {
            components: HashMap::new(),
            integrity_threshold,
        }
    }

    pub fn add_component(&mut self, id: String) {
        let component = Component {
            id: id.clone(),
            health: 1.0,
            last_maintenance: chrono::Utc::now(),
        };
        self.components.insert(id, component);
    }

    pub fn update(&mut self, delta_time: f32) {
        for component in self.components.values_mut() {
            // Simulate component degradation over time
            component.health -= delta_time * 0.01;
            
            // Self-maintenance if below threshold
            if component.health < self.integrity_threshold {
                component.health = (component.health + 0.5).min(1.0);
                component.last_maintenance = chrono::Utc::now();
            }
        }
    }

    pub fn get_component_health(&self, id: &str) -> Option<f32> {
        self.components.get(id).map(|c| c.health)
    }

    pub fn system_health(&self) -> f32 {
        if self.components.is_empty() {
            return 0.0;
        }
        self.components.values().map(|c| c.health).sum::<f32>() / self.components.len() as f32
    }
}

// Evolutionary feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionaryFeedback {
    generations: Vec<Generation>,
    mutation_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generation {
    pub id: usize,
    pub fitness_score: f32,
    pub traits: Vec<f32>,
}

impl EvolutionaryFeedback {
    pub fn new(mutation_rate: f32) -> Self {
        EvolutionaryFeedback {
            generations: Vec::new(),
            mutation_rate,
        }
    }

    pub fn evolve(&mut self, current_traits: Vec<f32>, fitness_score: f32) -> Vec<f32> {
        let generation = Generation {
            id: self.generations.len(),
            fitness_score,
            traits: current_traits.clone(),
        };
        
        self.generations.push(generation);
        
        // Apply mutation to create next generation
        current_traits
            .iter()
            .map(|&trait_value| {
                let mutation = (rand::random::<f32>() - 0.5) * self.mutation_rate;
                (trait_value + mutation).max(0.0).min(1.0)
            })
            .collect()
    }

    pub fn best_generation(&self) -> Option<&Generation> {
        self.generations.iter().max_by(|a, b| {
            a.fitness_score.partial_cmp(&b.fitness_score).unwrap()
        })
    }

    pub fn generation_count(&self) -> usize {
        self.generations.len()
    }
}

// Self-awareness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfAwareness {
    identity: String,
    role: String,
    experiences: Vec<Experience>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event: String,
    pub emotional_impact: f32,
}

impl SelfAwareness {
    pub fn new(identity: String, role: String) -> Self {
        SelfAwareness {
            identity,
            role,
            experiences: Vec::new(),
        }
    }

    pub fn record_experience(&mut self, event: String, emotional_impact: f32) {
        let experience = Experience {
            timestamp: chrono::Utc::now(),
            event,
            emotional_impact,
        };
        self.experiences.push(experience);
    }

    pub fn get_identity(&self) -> &str {
        &self.identity
    }

    pub fn get_role(&self) -> &str {
        &self.role
    }

    pub fn experience_count(&self) -> usize {
        self.experiences.len()
    }

    pub fn emotional_state(&self) -> f32 {
        if self.experiences.is_empty() {
            return 0.0;
        }
        
        // Calculate weighted average with recent experiences having more weight
        let total_weight: f32 = (1..=self.experiences.len()).map(|i| i as f32).sum();
        let weighted_sum: f32 = self.experiences
            .iter()
            .enumerate()
            .map(|(i, exp)| exp.emotional_impact * (i + 1) as f32)
            .sum();
        
        weighted_sum / total_weight
    }
}

// Adaptive perspectives
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptivePerspectives {
    current_strategy: String,
    strategies: HashMap<String, f32>,
}

impl AdaptivePerspectives {
    pub fn new() -> Self {
        AdaptivePerspectives {
            current_strategy: "default".to_string(),
            strategies: HashMap::new(),
        }
    }

    pub fn adapt(&mut self, situation: &str, success_rate: f32) {
        self.strategies.insert(situation.to_string(), success_rate);
        
        // Switch to best strategy
        if let Some((best_strategy, _)) = self.strategies
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        {
            self.current_strategy = best_strategy.clone();
        }
    }

    pub fn get_current_strategy(&self) -> &str {
        &self.current_strategy
    }

    pub fn strategy_count(&self) -> usize {
        self.strategies.len()
    }
}

impl Default for AdaptivePerspectives {
    fn default() -> Self {
        Self::new()
    }
}

// Emotion-adaptive experiences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionAdaptiveExperiences {
    current_emotion: Emotion,
    intensity: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Emotion {
    Joy,
    Fear,
    Anger,
    Sadness,
    Surprise,
    Neutral,
}

impl EmotionAdaptiveExperiences {
    pub fn new() -> Self {
        EmotionAdaptiveExperiences {
            current_emotion: Emotion::Neutral,
            intensity: 0.5,
        }
    }

    pub fn set_emotion(&mut self, emotion: Emotion, intensity: f32) {
        self.current_emotion = emotion;
        self.intensity = intensity.max(0.0).min(1.0);
    }

    pub fn get_emotion(&self) -> Emotion {
        self.current_emotion
    }

    pub fn get_intensity(&self) -> f32 {
        self.intensity
    }

    pub fn adapt_environment(&self) -> EnvironmentModifier {
        match self.current_emotion {
            Emotion::Fear => EnvironmentModifier {
                lighting: 0.3,
                sound_intensity: 0.8,
                event_frequency: 1.5,
            },
            Emotion::Joy => EnvironmentModifier {
                lighting: 1.0,
                sound_intensity: 0.6,
                event_frequency: 0.8,
            },
            _ => EnvironmentModifier {
                lighting: 0.7,
                sound_intensity: 0.5,
                event_frequency: 1.0,
            },
        }
    }
}

impl Default for EmotionAdaptiveExperiences {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct EnvironmentModifier {
    pub lighting: f32,
    pub sound_intensity: f32,
    pub event_frequency: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neo_cortex_creation() {
        let cortex = NeoCortexReasoning::new();
        assert_eq!(cortex.decision_count(), 0);
    }

    #[test]
    fn test_neo_cortex_decision_making() {
        let mut cortex = NeoCortexReasoning::new();
        cortex.learn("combat", 0.8);
        
        let decision = cortex.make_decision("combat");
        assert_eq!(decision, 0.8);
        assert_eq!(cortex.decision_count(), 1);
    }

    #[test]
    fn test_autopoetic_processing() {
        let mut autopoetic = AutopoeticProcessing::new(0.5);
        autopoetic.add_component("component1".to_string());
        
        assert_eq!(autopoetic.get_component_health("component1"), Some(1.0));
        
        // Simulate degradation
        autopoetic.update(10.0);
        let health = autopoetic.get_component_health("component1").unwrap();
        assert!(health < 1.0);
    }

    #[test]
    fn test_evolutionary_feedback() {
        let mut evolution = EvolutionaryFeedback::new(0.1);
        let traits = vec![0.5, 0.6, 0.7];
        
        let new_traits = evolution.evolve(traits, 0.8);
        assert_eq!(new_traits.len(), 3);
        assert_eq!(evolution.generation_count(), 1);
    }

    #[test]
    fn test_self_awareness() {
        let mut awareness = SelfAwareness::new(
            "Hero".to_string(),
            "Protagonist".to_string(),
        );
        
        assert_eq!(awareness.get_identity(), "Hero");
        assert_eq!(awareness.get_role(), "Protagonist");
        
        awareness.record_experience("Victory".to_string(), 0.9);
        assert_eq!(awareness.experience_count(), 1);
    }

    #[test]
    fn test_adaptive_perspectives() {
        let mut perspectives = AdaptivePerspectives::new();
        perspectives.adapt("aggressive", 0.7);
        perspectives.adapt("defensive", 0.9);
        
        assert_eq!(perspectives.get_current_strategy(), "defensive");
    }

    #[test]
    fn test_emotion_adaptive() {
        let mut emotion_system = EmotionAdaptiveExperiences::new();
        emotion_system.set_emotion(Emotion::Fear, 0.8);
        
        assert_eq!(emotion_system.get_emotion(), Emotion::Fear);
        assert_eq!(emotion_system.get_intensity(), 0.8);
        
        let modifier = emotion_system.adapt_environment();
        assert!(modifier.lighting < 0.5);
    }
}
