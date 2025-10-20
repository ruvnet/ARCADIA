//! CodeDNA - The genetic blueprint for game world generation and evolution

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// CodeDNA represents the fundamental genetic information of the game world
/// It defines the core characteristics that shape the entire game experience
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeDNA {
    /// Setting/world theme (e.g., "cyberpunk", "fantasy", "post-apocalyptic")
    pub setting: String,
    
    /// Technology level and type
    pub technology: String,
    
    /// Physics laws that govern the world
    pub physics_laws: Vec<PhysicsLaw>,
    
    /// Narrative themes
    pub themes: Vec<String>,
    
    /// Time scale factor (1.0 = normal time)
    pub time_scale: f32,
    
    /// Rate of entropy/decay in the world
    pub entropy_rate: f32,
    
    /// Natural laws and constraints
    pub natural_laws: Vec<String>,
    
    /// Story elements and progression
    pub storyline: Storyline,
    
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsLaw {
    pub name: String,
    pub description: String,
    pub parameters: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storyline {
    pub acts: Vec<Act>,
    pub current_act: usize,
    pub plot_points: Vec<PlotPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Act {
    pub id: String,
    pub name: String,
    pub description: String,
    pub objectives: Vec<String>,
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotPoint {
    pub id: String,
    pub description: String,
    pub triggered: bool,
    pub consequences: Vec<String>,
}

impl CodeDNA {
    /// Create a new CodeDNA instance with basic parameters
    pub fn new(
        setting: &str,
        technology: &str,
        physics_laws: Vec<String>,
        themes: Vec<String>,
        time_scale: f32,
        entropy_rate: f32,
        natural_laws: Vec<String>,
    ) -> Self {
        Self {
            setting: setting.to_string(),
            technology: technology.to_string(),
            physics_laws: physics_laws
                .into_iter()
                .map(|name| PhysicsLaw {
                    name,
                    description: String::new(),
                    parameters: HashMap::new(),
                })
                .collect(),
            themes,
            time_scale,
            entropy_rate,
            natural_laws,
            storyline: Storyline {
                acts: vec![],
                current_act: 0,
                plot_points: vec![],
            },
            metadata: HashMap::new(),
        }
    }
    
    /// Create a default fantasy setting
    pub fn fantasy_default() -> Self {
        Self::new(
            "High Fantasy",
            "Medieval with Magic",
            vec![
                "Mana Conservation".to_string(),
                "Gravity".to_string(),
                "Elemental Interactions".to_string(),
            ],
            vec![
                "Good vs Evil".to_string(),
                "Hero's Journey".to_string(),
                "Ancient Prophecy".to_string(),
            ],
            1.0,
            0.01,
            vec![
                "Magic follows intent".to_string(),
                "Balance of elements".to_string(),
                "Karma and consequence".to_string(),
            ],
        )
    }
    
    /// Create a default cyberpunk setting
    pub fn cyberpunk_default() -> Self {
        Self::new(
            "Cyberpunk Dystopia",
            "High-tech Low-life",
            vec![
                "Neural Network Physics".to_string(),
                "Quantum Computing".to_string(),
                "Augmentation Limits".to_string(),
            ],
            vec![
                "Corporate Control".to_string(),
                "Transhumanism".to_string(),
                "Digital vs Physical".to_string(),
            ],
            1.2, // Slightly faster pace
            0.05, // Higher entropy (more chaos)
            vec![
                "Technology corrupts".to_string(),
                "Information is power".to_string(),
                "Identity is fluid".to_string(),
            ],
        )
    }
    
    /// Apply the CodeDNA to a game world, modifying its properties
    pub fn apply_to_world(&self, world: &mut crate::game_components::GameWorld) {
        world.set_setting(&self.setting);
        world.set_technology(&self.technology);
        world.set_time_scale(self.time_scale);
        world.set_entropy_rate(self.entropy_rate);
        
        // Apply physics laws
        for law in &self.physics_laws {
            world.add_physics_law(law.clone());
        }
        
        // Apply themes
        for theme in &self.themes {
            world.add_theme(theme.clone());
        }
        
        // Apply natural laws
        for law in &self.natural_laws {
            world.add_natural_law(law.clone());
        }
    }
    
    /// Mutate the DNA based on player actions or events
    pub fn mutate(&mut self, mutation_type: MutationType, intensity: f32) {
        match mutation_type {
            MutationType::IncreaseEntropy => {
                self.entropy_rate = (self.entropy_rate + 0.01 * intensity).min(1.0);
            }
            MutationType::DecreaseEntropy => {
                self.entropy_rate = (self.entropy_rate - 0.01 * intensity).max(0.0);
            }
            MutationType::TimeAcceleration => {
                self.time_scale = (self.time_scale * (1.0 + 0.1 * intensity)).min(10.0);
            }
            MutationType::TimeDeceleration => {
                self.time_scale = (self.time_scale * (1.0 - 0.1 * intensity)).max(0.1);
            }
            MutationType::AddTheme(theme) => {
                if !self.themes.contains(&theme) {
                    self.themes.push(theme);
                }
            }
            MutationType::RemoveTheme(theme) => {
                self.themes.retain(|t| t != &theme);
            }
        }
    }
    
    /// Get the current narrative state
    pub fn get_current_act(&self) -> Option<&Act> {
        self.storyline.acts.get(self.storyline.current_act)
    }
    
    /// Progress to the next act
    pub fn advance_act(&mut self) -> bool {
        if self.storyline.current_act < self.storyline.acts.len() - 1 {
            self.storyline.current_act += 1;
            true
        } else {
            false
        }
    }
    
    /// Trigger a plot point
    pub fn trigger_plot_point(&mut self, id: &str) -> Option<Vec<String>> {
        if let Some(plot_point) = self
            .storyline
            .plot_points
            .iter_mut()
            .find(|p| p.id == id)
        {
            plot_point.triggered = true;
            Some(plot_point.consequences.clone())
        } else {
            None
        }
    }
    
    /// Calculate compatibility with another CodeDNA (for crossover/breeding)
    pub fn compatibility(&self, other: &CodeDNA) -> f32 {
        let mut score = 0.0;
        let mut factors = 0.0;
        
        // Compare themes
        let common_themes = self
            .themes
            .iter()
            .filter(|t| other.themes.contains(t))
            .count();
        score += (common_themes as f32 / self.themes.len().max(1) as f32) * 0.3;
        factors += 0.3;
        
        // Compare time scales
        let time_diff = (self.time_scale - other.time_scale).abs();
        score += ((1.0 - time_diff / 10.0).max(0.0)) * 0.2;
        factors += 0.2;
        
        // Compare entropy rates
        let entropy_diff = (self.entropy_rate - other.entropy_rate).abs();
        score += ((1.0 - entropy_diff).max(0.0)) * 0.2;
        factors += 0.2;
        
        // Compare natural laws
        let common_laws = self
            .natural_laws
            .iter()
            .filter(|l| other.natural_laws.contains(l))
            .count();
        score += (common_laws as f32 / self.natural_laws.len().max(1) as f32) * 0.3;
        factors += 0.3;
        
        score / factors
    }
}

#[derive(Debug, Clone)]
pub enum MutationType {
    IncreaseEntropy,
    DecreaseEntropy,
    TimeAcceleration,
    TimeDeceleration,
    AddTheme(String),
    RemoveTheme(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_fantasy_dna() {
        let dna = CodeDNA::fantasy_default();
        assert_eq!(dna.setting, "High Fantasy");
        assert_eq!(dna.time_scale, 1.0);
        assert!(dna.themes.contains(&"Good vs Evil".to_string()));
    }
    
    #[test]
    fn test_create_cyberpunk_dna() {
        let dna = CodeDNA::cyberpunk_default();
        assert_eq!(dna.setting, "Cyberpunk Dystopia");
        assert!(dna.entropy_rate > 0.01);
    }
    
    #[test]
    fn test_dna_mutation() {
        let mut dna = CodeDNA::fantasy_default();
        let original_entropy = dna.entropy_rate;
        
        dna.mutate(MutationType::IncreaseEntropy, 1.0);
        assert!(dna.entropy_rate > original_entropy);
    }
    
    #[test]
    fn test_dna_compatibility() {
        let dna1 = CodeDNA::fantasy_default();
        let dna2 = CodeDNA::fantasy_default();
        let dna3 = CodeDNA::cyberpunk_default();
        
        let comp_same = dna1.compatibility(&dna2);
        let comp_diff = dna1.compatibility(&dna3);
        
        assert!(comp_same > comp_diff);
    }
}
