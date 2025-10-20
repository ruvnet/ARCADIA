//! Self-Awareness Module
//!
//! This module implements state monitoring, role understanding, context awareness,
//! and meta-reasoning capabilities for AI entities.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use parking_lot::RwLock;
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;

/// Role of an entity within the game world
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityRole {
    Player,
    Ally,
    Enemy,
    Neutral,
    Guardian,
    Merchant,
    QuestGiver,
    Boss,
    Custom(String),
}

/// State of consciousness for the entity
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ConsciousnessState {
    Dormant,
    Aware,
    Focused,
    Reflective,
    Transcendent,
}

/// Internal state snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub timestamp: DateTime<Utc>,
    pub physical_state: HashMap<String, f64>,
    pub emotional_state: HashMap<String, f64>,
    pub cognitive_state: HashMap<String, f64>,
    pub environmental_context: HashMap<String, f64>,
}

/// Belief about the world or self
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Belief {
    pub id: Uuid,
    pub statement: String,
    pub confidence: f64,
    pub evidence_count: usize,
    pub last_updated: DateTime<Utc>,
}

/// Meta-reasoning thought process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaThought {
    pub id: Uuid,
    pub thought_type: MetaThoughtType,
    pub content: String,
    pub depth: usize,
    pub timestamp: DateTime<Utc>,
}

/// Types of meta-thoughts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetaThoughtType {
    SelfReflection,
    PurposeQuestioning,
    CapabilityAssessment,
    EmotionalAnalysis,
    ContextualUnderstanding,
    FutureProjection,
}

/// Relationship understanding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub target_id: Uuid,
    pub target_name: String,
    pub relationship_type: String,
    pub strength: f64,
    pub history_count: usize,
    pub last_interaction: DateTime<Utc>,
}

/// Self-awareness system for AI entities
pub struct SelfAwareness {
    /// Unique entity identifier
    entity_id: Uuid,

    /// Entity name
    entity_name: Arc<RwLock<String>>,

    /// Current role(s) in the world
    roles: Arc<RwLock<Vec<EntityRole>>>,

    /// State of consciousness
    consciousness_state: Arc<RwLock<ConsciousnessState>>,

    /// Recent state history
    state_history: Arc<RwLock<VecDeque<StateSnapshot>>>,

    /// Beliefs about self and world
    beliefs: Arc<RwLock<HashMap<String, Belief>>>,

    /// Meta-reasoning thoughts
    meta_thoughts: Arc<RwLock<Vec<MetaThought>>>,

    /// Relationship map
    relationships: Arc<RwLock<HashMap<Uuid, Relationship>>>,

    /// Self-model: what the entity thinks about itself
    self_model: Arc<RwLock<HashMap<String, f64>>>,

    /// Context awareness: understanding of current situation
    context_awareness: Arc<RwLock<HashMap<String, String>>>,

    /// Goals and motivations
    motivations: Arc<RwLock<Vec<String>>>,

    /// Max history size
    max_history_size: usize,
}

impl SelfAwareness {
    /// Create a new self-awareness system
    pub fn new(entity_id: Uuid, entity_name: String) -> Self {
        let mut self_model = HashMap::new();
        self_model.insert("capability".to_string(), 0.5);
        self_model.insert("confidence".to_string(), 0.5);
        self_model.insert("autonomy".to_string(), 0.5);
        self_model.insert("adaptability".to_string(), 0.5);

        Self {
            entity_id,
            entity_name: Arc::new(RwLock::new(entity_name)),
            roles: Arc::new(RwLock::new(vec![EntityRole::Neutral])),
            consciousness_state: Arc::new(RwLock::new(ConsciousnessState::Aware)),
            state_history: Arc::new(RwLock::new(VecDeque::new())),
            beliefs: Arc::new(RwLock::new(HashMap::new())),
            meta_thoughts: Arc::new(RwLock::new(Vec::new())),
            relationships: Arc::new(RwLock::new(HashMap::new())),
            self_model: Arc::new(RwLock::new(self_model)),
            context_awareness: Arc::new(RwLock::new(HashMap::new())),
            motivations: Arc::new(RwLock::new(Vec::new())),
            max_history_size: 100,
        }
    }

    /// Get entity ID
    pub fn get_entity_id(&self) -> Uuid {
        self.entity_id
    }

    /// Get entity name
    pub fn get_entity_name(&self) -> String {
        self.entity_name.read().clone()
    }

    /// Set entity name
    pub fn set_entity_name(&self, name: String) {
        *self.entity_name.write() = name;
    }

    /// Add a role
    pub fn add_role(&self, role: EntityRole) {
        let mut roles = self.roles.write();
        if !roles.contains(&role) {
            roles.push(role);
        }
    }

    /// Remove a role
    pub fn remove_role(&self, role: &EntityRole) -> bool {
        let mut roles = self.roles.write();
        if let Some(pos) = roles.iter().position(|r| r == role) {
            roles.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get all current roles
    pub fn get_roles(&self) -> Vec<EntityRole> {
        self.roles.read().clone()
    }

    /// Check if entity has a specific role
    pub fn has_role(&self, role: &EntityRole) -> bool {
        self.roles.read().contains(role)
    }

    /// Set consciousness state
    pub fn set_consciousness_state(&self, state: ConsciousnessState) {
        *self.consciousness_state.write() = state;

        // Higher consciousness states trigger meta-reasoning
        if matches!(state, ConsciousnessState::Reflective | ConsciousnessState::Transcendent) {
            self.engage_meta_reasoning();
        }
    }

    /// Get consciousness state
    pub fn get_consciousness_state(&self) -> ConsciousnessState {
        *self.consciousness_state.read()
    }

    /// Record current state snapshot
    pub fn record_state(&self, snapshot: StateSnapshot) {
        let mut history = self.state_history.write();
        history.push_back(snapshot.clone());

        // Maintain max size
        while history.len() > self.max_history_size {
            history.pop_front();
        }

        drop(history);

        // Analyze state changes
        self.analyze_state_changes(&snapshot);
    }

    /// Analyze changes in state
    fn analyze_state_changes(&self, current: &StateSnapshot) {
        let history = self.state_history.read();

        if let Some(previous) = history.iter().rev().nth(1) {
            // Check for significant changes
            for (key, value) in &current.emotional_state {
                if let Some(prev_value) = previous.emotional_state.get(key) {
                    let change = (value - prev_value).abs();
                    if change > 0.3 {
                        self.form_belief(
                            format!("Emotional state '{}' changed significantly", key),
                            0.8,
                        );
                    }
                }
            }
        }
    }

    /// Add or update a belief
    pub fn form_belief(&self, statement: String, confidence: f64) {
        let mut beliefs = self.beliefs.write();

        if let Some(belief) = beliefs.get_mut(&statement) {
            // Update existing belief
            belief.confidence = (belief.confidence * 0.7 + confidence * 0.3).clamp(0.0, 1.0);
            belief.evidence_count += 1;
            belief.last_updated = Utc::now();
        } else {
            // Create new belief
            let belief = Belief {
                id: Uuid::new_v4(),
                statement: statement.clone(),
                confidence: confidence.clamp(0.0, 1.0),
                evidence_count: 1,
                last_updated: Utc::now(),
            };
            beliefs.insert(statement, belief);
        }
    }

    /// Get a belief
    pub fn get_belief(&self, statement: &str) -> Option<Belief> {
        self.beliefs.read().get(statement).cloned()
    }

    /// Get all beliefs
    pub fn get_all_beliefs(&self) -> Vec<Belief> {
        self.beliefs.read().values().cloned().collect()
    }

    /// Challenge a belief with counter-evidence
    pub fn challenge_belief(&self, statement: &str, counter_confidence: f64) {
        if let Some(belief) = self.beliefs.write().get_mut(statement) {
            belief.confidence *= 1.0 - counter_confidence;
            belief.confidence = belief.confidence.clamp(0.0, 1.0);
            belief.last_updated = Utc::now();
        }
    }

    /// Engage in meta-reasoning
    pub fn engage_meta_reasoning(&self) {
        let consciousness = *self.consciousness_state.read();

        // Self-reflection
        self.add_meta_thought(
            MetaThoughtType::SelfReflection,
            format!("I am {} in state {:?}", self.get_entity_name(), consciousness),
            1,
        );

        // Purpose questioning
        let roles = self.roles.read();
        if !roles.is_empty() {
            self.add_meta_thought(
                MetaThoughtType::PurposeQuestioning,
                format!("My current roles are: {:?}", roles),
                1,
            );
        }

        // Capability assessment
        let self_model = self.self_model.read();
        let avg_capability = self_model.values().sum::<f64>() / self_model.len() as f64;
        self.add_meta_thought(
            MetaThoughtType::CapabilityAssessment,
            format!("My overall capability level is {:.2}", avg_capability),
            2,
        );
    }

    /// Add a meta-thought
    pub fn add_meta_thought(&self, thought_type: MetaThoughtType, content: String, depth: usize) {
        let thought = MetaThought {
            id: Uuid::new_v4(),
            thought_type,
            content,
            depth,
            timestamp: Utc::now(),
        };

        self.meta_thoughts.write().push(thought);

        // Keep only recent thoughts
        let mut thoughts = self.meta_thoughts.write();
        if thoughts.len() > 50 {
            thoughts.drain(0..10);
        }
    }

    /// Get recent meta-thoughts
    pub fn get_recent_meta_thoughts(&self, count: usize) -> Vec<MetaThought> {
        self.meta_thoughts.read()
            .iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }

    /// Update self-model attribute
    pub fn update_self_model(&self, attribute: String, value: f64) {
        self.self_model.write().insert(attribute, value.clamp(0.0, 1.0));
    }

    /// Get self-model
    pub fn get_self_model(&self) -> HashMap<String, f64> {
        self.self_model.read().clone()
    }

    /// Record a relationship
    pub fn record_relationship(&self, target_id: Uuid, target_name: String, relationship_type: String, strength: f64) {
        let mut relationships = self.relationships.write();

        if let Some(rel) = relationships.get_mut(&target_id) {
            // Update existing relationship
            rel.strength = (rel.strength * 0.6 + strength * 0.4).clamp(-1.0, 1.0);
            rel.history_count += 1;
            rel.last_interaction = Utc::now();
        } else {
            // Create new relationship
            let rel = Relationship {
                target_id,
                target_name,
                relationship_type,
                strength: strength.clamp(-1.0, 1.0),
                history_count: 1,
                last_interaction: Utc::now(),
            };
            relationships.insert(target_id, rel);
        }
    }

    /// Get relationship with another entity
    pub fn get_relationship(&self, target_id: Uuid) -> Option<Relationship> {
        self.relationships.read().get(&target_id).cloned()
    }

    /// Get all relationships
    pub fn get_all_relationships(&self) -> Vec<Relationship> {
        self.relationships.read().values().cloned().collect()
    }

    /// Update context awareness
    pub fn update_context(&self, key: String, value: String) {
        self.context_awareness.write().insert(key, value);
    }

    /// Get context
    pub fn get_context(&self, key: &str) -> Option<String> {
        self.context_awareness.read().get(key).cloned()
    }

    /// Get full context
    pub fn get_full_context(&self) -> HashMap<String, String> {
        self.context_awareness.read().clone()
    }

    /// Understand current situation
    pub fn understand_situation(&self) -> String {
        let roles = self.roles.read();
        let context = self.context_awareness.read();
        let consciousness = *self.consciousness_state.read();

        let mut understanding = Vec::new();

        understanding.push(format!("I am {} with consciousness level {:?}", self.get_entity_name(), consciousness));

        if !roles.is_empty() {
            understanding.push(format!("I serve as {:?}", roles));
        }

        if let Some(location) = context.get("location") {
            understanding.push(format!("I am currently at {}", location));
        }

        if let Some(objective) = context.get("objective") {
            understanding.push(format!("My current objective is: {}", objective));
        }

        let relationships = self.relationships.read();
        if !relationships.is_empty() {
            understanding.push(format!("I have {} known relationships", relationships.len()));
        }

        understanding.join(". ")
    }

    /// Add a motivation
    pub fn add_motivation(&self, motivation: String) {
        let mut motivations = self.motivations.write();
        if !motivations.contains(&motivation) {
            motivations.push(motivation);
        }
    }

    /// Get all motivations
    pub fn get_motivations(&self) -> Vec<String> {
        self.motivations.read().clone()
    }

    /// Reflect on past actions
    pub fn reflect_on_past(&self, time_window: Duration) -> Vec<String> {
        let history = self.state_history.read();
        let cutoff = Utc::now() - time_window;
        let mut reflections = Vec::new();

        let recent_states: Vec<_> = history.iter()
            .filter(|s| s.timestamp > cutoff)
            .collect();

        if recent_states.len() >= 2 {
            let first = recent_states.first().unwrap();
            let last = recent_states.last().unwrap();

            // Analyze emotional trajectory
            for (key, final_value) in &last.emotional_state {
                if let Some(initial_value) = first.emotional_state.get(key) {
                    let change = final_value - initial_value;
                    if change.abs() > 0.2 {
                        reflections.push(format!(
                            "My {} has {} by {:.2}",
                            key,
                            if change > 0.0 { "increased" } else { "decreased" },
                            change.abs()
                        ));
                    }
                }
            }
        }

        reflections
    }

    /// Assess self-awareness level
    pub fn assess_self_awareness_level(&self) -> f64 {
        let mut score = 0.0;

        // Factor 1: Number of beliefs
        let belief_count = self.beliefs.read().len() as f64;
        score += (belief_count / 10.0).min(0.2);

        // Factor 2: Meta-thought depth
        let max_depth = self.meta_thoughts.read()
            .iter()
            .map(|t| t.depth)
            .max()
            .unwrap_or(0) as f64;
        score += (max_depth / 5.0).min(0.2);

        // Factor 3: Relationship awareness
        let rel_count = self.relationships.read().len() as f64;
        score += (rel_count / 20.0).min(0.2);

        // Factor 4: Self-model completeness
        let model_size = self.self_model.read().len() as f64;
        score += (model_size / 10.0).min(0.2);

        // Factor 5: Context awareness
        let context_size = self.context_awareness.read().len() as f64;
        score += (context_size / 10.0).min(0.2);

        score.min(1.0)
    }

    /// Describe self
    pub fn describe_self(&self) -> String {
        let name = self.get_entity_name();
        let roles = self.roles.read();
        let consciousness = *self.consciousness_state.read();
        let awareness_level = self.assess_self_awareness_level();

        format!(
            "I am {}, a {:?} entity with consciousness state {:?}. My self-awareness level is {:.2}. I currently serve {} role(s) in this world.",
            name,
            roles.first().unwrap_or(&EntityRole::Neutral),
            consciousness,
            awareness_level,
            roles.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_awareness_creation() {
        let entity_id = Uuid::new_v4();
        let sa = SelfAwareness::new(entity_id, "Test Entity".to_string());

        assert_eq!(sa.get_entity_id(), entity_id);
        assert_eq!(sa.get_entity_name(), "Test Entity");
        assert_eq!(sa.get_consciousness_state(), ConsciousnessState::Aware);
    }

    #[test]
    fn test_role_management() {
        let sa = SelfAwareness::new(Uuid::new_v4(), "NPC".to_string());

        sa.add_role(EntityRole::Guardian);
        assert!(sa.has_role(&EntityRole::Guardian));

        sa.add_role(EntityRole::QuestGiver);
        assert_eq!(sa.get_roles().len(), 3); // Neutral + Guardian + QuestGiver

        sa.remove_role(&EntityRole::Neutral);
        assert_eq!(sa.get_roles().len(), 2);
    }

    #[test]
    fn test_belief_system() {
        let sa = SelfAwareness::new(Uuid::new_v4(), "Thinker".to_string());

        sa.form_belief("The world is vast".to_string(), 0.9);
        let belief = sa.get_belief("The world is vast");
        assert!(belief.is_some());
        assert_eq!(belief.unwrap().confidence, 0.9);

        sa.challenge_belief("The world is vast", 0.5);
        let challenged = sa.get_belief("The world is vast");
        assert!(challenged.unwrap().confidence < 0.9);
    }

    #[test]
    fn test_meta_reasoning() {
        let sa = SelfAwareness::new(Uuid::new_v4(), "Philosopher".to_string());

        sa.set_consciousness_state(ConsciousnessState::Reflective);
        let thoughts = sa.get_recent_meta_thoughts(10);
        assert!(!thoughts.is_empty());
    }

    #[test]
    fn test_relationship_tracking() {
        let sa = SelfAwareness::new(Uuid::new_v4(), "Social Entity".to_string());
        let friend_id = Uuid::new_v4();

        sa.record_relationship(friend_id, "Friend".to_string(), "ally".to_string(), 0.8);
        let rel = sa.get_relationship(friend_id);
        assert!(rel.is_some());
        assert_eq!(rel.unwrap().strength, 0.8);
    }

    #[test]
    fn test_situation_understanding() {
        let sa = SelfAwareness::new(Uuid::new_v4(), "Aware NPC".to_string());

        sa.add_role(EntityRole::Guardian);
        sa.update_context("location".to_string(), "Ancient Temple".to_string());
        sa.update_context("objective".to_string(), "Protect the artifact".to_string());

        let understanding = sa.understand_situation();
        assert!(understanding.contains("Aware NPC"));
        assert!(understanding.contains("Guardian"));
    }
}
