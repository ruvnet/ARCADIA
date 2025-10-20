//! Game Elements module for ARCADIA
//! 
//! Manages all game elements including functional and non-functional components,
//! social constructs, multiplayer experiences, and more

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::ai_systems::*;
use crate::code_dna::CodeDNA;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameElement {
    pub element_type: String,
    pub properties: HashMap<String, String>,
}

impl GameElement {
    pub fn new(element_type: String) -> Self {
        GameElement {
            element_type,
            properties: HashMap::new(),
        }
    }

    pub fn with_property(mut self, key: String, value: String) -> Self {
        self.properties.insert(key, value);
        self
    }

    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }
}

// Functional components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionalComponent {
    pub id: String,
    pub component_type: ComponentType,
    pub position: (f32, f32, f32),
    pub properties: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComponentType {
    Character,
    Object,
    Location,
    Terrain,
}

impl FunctionalComponent {
    pub fn new(id: String, component_type: ComponentType, position: (f32, f32, f32)) -> Self {
        FunctionalComponent {
            id,
            component_type,
            position,
            properties: HashMap::new(),
        }
    }

    pub fn set_property(&mut self, key: String, value: f32) {
        self.properties.insert(key, value);
    }

    pub fn get_property(&self, key: &str) -> Option<f32> {
        self.properties.get(key).copied()
    }

    pub fn update_position(&mut self, delta: (f32, f32, f32)) {
        self.position.0 += delta.0;
        self.position.1 += delta.1;
        self.position.2 += delta.2;
    }
}

// Non-functional components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonFunctionalComponents {
    pub performance_target_fps: f32,
    pub reliability_uptime: f32,
    pub scalability_max_players: usize,
    pub security_level: SecurityLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl NonFunctionalComponents {
    pub fn new() -> Self {
        NonFunctionalComponents {
            performance_target_fps: 60.0,
            reliability_uptime: 0.99,
            scalability_max_players: 100,
            security_level: SecurityLevel::High,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.performance_target_fps <= 0.0 {
            return Err("Performance target FPS must be positive".to_string());
        }
        if self.reliability_uptime < 0.0 || self.reliability_uptime > 1.0 {
            return Err("Reliability uptime must be between 0 and 1".to_string());
        }
        Ok(())
    }
}

impl Default for NonFunctionalComponents {
    fn default() -> Self {
        Self::new()
    }
}

// Social constructs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialConstructs {
    factions: HashMap<String, Faction>,
    reputation_system: ReputationSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    pub name: String,
    pub alignment: f32, // -1.0 (evil) to 1.0 (good)
    pub influence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationSystem {
    player_reputations: HashMap<String, f32>,
}

impl SocialConstructs {
    pub fn new() -> Self {
        SocialConstructs {
            factions: HashMap::new(),
            reputation_system: ReputationSystem {
                player_reputations: HashMap::new(),
            },
        }
    }

    pub fn add_faction(&mut self, name: String, alignment: f32, influence: f32) {
        let faction = Faction {
            name: name.clone(),
            alignment,
            influence,
        };
        self.factions.insert(name, faction);
    }

    pub fn update_reputation(&mut self, player_id: String, faction: &str, delta: f32) {
        let key = format!("{}:{}", player_id, faction);
        let current = self.reputation_system.player_reputations
            .get(&key)
            .copied()
            .unwrap_or(0.0);
        self.reputation_system.player_reputations
            .insert(key, (current + delta).max(-1.0).min(1.0));
    }

    pub fn get_reputation(&self, player_id: &str, faction: &str) -> f32 {
        let key = format!("{}:{}", player_id, faction);
        self.reputation_system.player_reputations
            .get(&key)
            .copied()
            .unwrap_or(0.0)
    }
}

impl Default for SocialConstructs {
    fn default() -> Self {
        Self::new()
    }
}

// Entropy system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entropy {
    decay_rate: f32,
    objects: HashMap<String, EntropyObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyObject {
    pub id: String,
    pub durability: f32,
    pub age: f32,
}

impl Entropy {
    pub fn new(decay_rate: f32) -> Self {
        Entropy {
            decay_rate,
            objects: HashMap::new(),
        }
    }

    pub fn add_object(&mut self, id: String, durability: f32) {
        let obj = EntropyObject {
            id: id.clone(),
            durability,
            age: 0.0,
        };
        self.objects.insert(id, obj);
    }

    pub fn update(&mut self, delta_time: f32) {
        for obj in self.objects.values_mut() {
            obj.age += delta_time;
            obj.durability = (obj.durability - self.decay_rate * delta_time).max(0.0);
        }
    }

    pub fn get_durability(&self, id: &str) -> Option<f32> {
        self.objects.get(id).map(|obj| obj.durability)
    }

    pub fn cleanup_destroyed(&mut self) -> usize {
        let initial_count = self.objects.len();
        self.objects.retain(|_, obj| obj.durability > 0.0);
        initial_count - self.objects.len()
    }
}

// Main GameElements struct
#[derive(Debug)]
pub struct GameElements {
    pub code_dna: CodeDNA,
    pub functional_components: Vec<FunctionalComponent>,
    pub non_functional_components: NonFunctionalComponents,
    pub neo_cortex_reasoning: NeoCortexReasoning,
    pub autopoetic_processing: AutopoeticProcessing,
    pub evolutionary_feedback: EvolutionaryFeedback,
    pub self_awareness: SelfAwareness,
    pub adaptive_perspectives: AdaptivePerspectives,
    pub entropy: Entropy,
    pub emotion_adaptive_experiences: EmotionAdaptiveExperiences,
    pub social_constructs: SocialConstructs,
}

impl GameElements {
    pub fn new(code_dna: CodeDNA) -> Self {
        GameElements {
            code_dna,
            functional_components: Vec::new(),
            non_functional_components: NonFunctionalComponents::new(),
            neo_cortex_reasoning: NeoCortexReasoning::new(),
            autopoetic_processing: AutopoeticProcessing::new(0.5),
            evolutionary_feedback: EvolutionaryFeedback::new(0.1),
            self_awareness: SelfAwareness::new("Game".to_string(), "System".to_string()),
            adaptive_perspectives: AdaptivePerspectives::new(),
            entropy: Entropy::new(0.01),
            emotion_adaptive_experiences: EmotionAdaptiveExperiences::new(),
            social_constructs: SocialConstructs::new(),
        }
    }

    pub fn add_component(&mut self, component: FunctionalComponent) {
        self.functional_components.push(component);
    }

    pub fn update(&mut self, delta_time: f32) {
        self.autopoetic_processing.update(delta_time);
        self.entropy.update(delta_time);
    }

    pub fn component_count(&self) -> usize {
        self.functional_components.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_element_creation() {
        let element = GameElement::new("Character".to_string())
            .with_property("health".to_string(), "100".to_string());
        
        assert_eq!(element.element_type, "Character");
        assert_eq!(element.get_property("health"), Some(&"100".to_string()));
    }

    #[test]
    fn test_functional_component() {
        let mut component = FunctionalComponent::new(
            "player1".to_string(),
            ComponentType::Character,
            (0.0, 0.0, 0.0),
        );
        
        component.set_property("health".to_string(), 100.0);
        assert_eq!(component.get_property("health"), Some(100.0));
        
        component.update_position((1.0, 2.0, 3.0));
        assert_eq!(component.position, (1.0, 2.0, 3.0));
    }

    #[test]
    fn test_non_functional_components() {
        let nfc = NonFunctionalComponents::new();
        assert!(nfc.validate().is_ok());
        assert_eq!(nfc.performance_target_fps, 60.0);
    }

    #[test]
    fn test_social_constructs() {
        let mut social = SocialConstructs::new();
        social.add_faction("Knights".to_string(), 0.8, 0.6);
        
        social.update_reputation("player1".to_string(), "Knights", 0.5);
        assert_eq!(social.get_reputation("player1", "Knights"), 0.5);
    }

    #[test]
    fn test_entropy() {
        let mut entropy = Entropy::new(0.1);
        entropy.add_object("building1".to_string(), 1.0);
        
        entropy.update(5.0);
        let durability = entropy.get_durability("building1").unwrap();
        assert!(durability < 1.0);
    }

    #[test]
    fn test_game_elements() {
        let dna = CodeDNA::default_scifi();
        let mut game_elements = GameElements::new(dna);
        
        let component = FunctionalComponent::new(
            "player1".to_string(),
            ComponentType::Character,
            (0.0, 0.0, 0.0),
        );
        
        game_elements.add_component(component);
        assert_eq!(game_elements.component_count(), 1);
    }
}
