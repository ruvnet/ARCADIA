//! CodeDNA - The genetic blueprint for game world configuration
//! 
//! This module defines the core attributes and rules of the game world including
//! storyline, theme components, physical laws, time, entropy, and natural laws.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeDNA {
    pub setting: String,
    pub technology: String,
    pub physics_laws: Vec<String>,
    pub themes: Vec<String>,
    pub time_scale: f32,
    pub entropy_rate: f32,
    pub natural_laws: Vec<String>,
}

impl CodeDNA {
    /// Creates a new CodeDNA instance with the specified parameters
    pub fn new(
        setting: &str,
        technology: &str,
        physics_laws: &[String],
        themes: &[String],
        time_scale: f32,
        entropy_rate: f32,
        natural_laws: &[String],
    ) -> Self {
        CodeDNA {
            setting: setting.to_string(),
            technology: technology.to_string(),
            physics_laws: physics_laws.to_vec(),
            themes: themes.to_vec(),
            time_scale,
            entropy_rate,
            natural_laws: natural_laws.to_vec(),
        }
    }

    /// Validates the CodeDNA configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.setting.is_empty() {
            return Err("Setting cannot be empty".to_string());
        }
        if self.time_scale <= 0.0 {
            return Err("Time scale must be positive".to_string());
        }
        if self.entropy_rate < 0.0 || self.entropy_rate > 1.0 {
            return Err("Entropy rate must be between 0.0 and 1.0".to_string());
        }
        Ok(())
    }

    /// Applies the CodeDNA attributes to a game world
    pub fn apply_to_game_world(&self, game_world: &mut GameWorld) {
        game_world.set_setting(&self.setting);
        game_world.set_technology(&self.technology);
        game_world.set_physics_laws(&self.physics_laws);
        game_world.set_themes(&self.themes);
        game_world.set_time_scale(self.time_scale);
        game_world.set_entropy_rate(self.entropy_rate);
        game_world.set_natural_laws(&self.natural_laws);
    }

    /// Creates a default sci-fi CodeDNA configuration
    pub fn default_scifi() -> Self {
        CodeDNA::new(
            "Futuristic Space Station",
            "Advanced FTL Technology",
            &[
                "Relativity".to_string(),
                "Quantum Mechanics".to_string(),
            ],
            &[
                "Space Exploration".to_string(),
                "First Contact".to_string(),
            ],
            1.0,
            0.1,
            &[
                "Thermodynamics".to_string(),
                "Conservation of Energy".to_string(),
            ],
        )
    }

    /// Creates a default fantasy CodeDNA configuration
    pub fn default_fantasy() -> Self {
        CodeDNA::new(
            "Medieval Fantasy Kingdom",
            "Magic and Sorcery",
            &[
                "Magic Laws".to_string(),
                "Medieval Physics".to_string(),
            ],
            &[
                "Quest for the Crown".to_string(),
                "Dragon Threat".to_string(),
            ],
            1.0,
            0.05,
            &[
                "Natural Selection".to_string(),
                "Magical Equilibrium".to_string(),
            ],
        )
    }
}

/// Placeholder for GameWorld - will be implemented in game_elements module
pub struct GameWorld {
    setting: String,
    technology: String,
    physics_laws: Vec<String>,
    themes: Vec<String>,
    time_scale: f32,
    entropy_rate: f32,
    natural_laws: Vec<String>,
}

impl GameWorld {
    pub fn new() -> Self {
        GameWorld {
            setting: String::new(),
            technology: String::new(),
            physics_laws: Vec::new(),
            themes: Vec::new(),
            time_scale: 1.0,
            entropy_rate: 0.1,
            natural_laws: Vec::new(),
        }
    }

    pub fn set_setting(&mut self, setting: &str) {
        self.setting = setting.to_string();
    }

    pub fn set_technology(&mut self, technology: &str) {
        self.technology = technology.to_string();
    }

    pub fn set_physics_laws(&mut self, laws: &[String]) {
        self.physics_laws = laws.to_vec();
    }

    pub fn set_themes(&mut self, themes: &[String]) {
        self.themes = themes.to_vec();
    }

    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale;
    }

    pub fn set_entropy_rate(&mut self, rate: f32) {
        self.entropy_rate = rate;
    }

    pub fn set_natural_laws(&mut self, laws: &[String]) {
        self.natural_laws = laws.to_vec();
    }

    pub fn get_setting(&self) -> &str {
        &self.setting
    }

    pub fn get_time_scale(&self) -> f32 {
        self.time_scale
    }

    pub fn get_entropy_rate(&self) -> f32 {
        self.entropy_rate
    }
}

impl Default for GameWorld {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_dna_creation() {
        let dna = CodeDNA::new(
            "Test Setting",
            "Test Tech",
            &[String::from("Law1")],
            &[String::from("Theme1")],
            1.0,
            0.5,
            &[String::from("Natural1")],
        );

        assert_eq!(dna.setting, "Test Setting");
        assert_eq!(dna.technology, "Test Tech");
        assert_eq!(dna.time_scale, 1.0);
        assert_eq!(dna.entropy_rate, 0.5);
    }

    #[test]
    fn test_code_dna_validation() {
        let valid_dna = CodeDNA::new(
            "Valid",
            "Tech",
            &[],
            &[],
            1.0,
            0.5,
            &[],
        );
        assert!(valid_dna.validate().is_ok());

        let invalid_dna = CodeDNA::new(
            "",
            "Tech",
            &[],
            &[],
            1.0,
            0.5,
            &[],
        );
        assert!(invalid_dna.validate().is_err());
    }

    #[test]
    fn test_default_scifi() {
        let dna = CodeDNA::default_scifi();
        assert_eq!(dna.setting, "Futuristic Space Station");
        assert!(dna.validate().is_ok());
    }

    #[test]
    fn test_default_fantasy() {
        let dna = CodeDNA::default_fantasy();
        assert_eq!(dna.setting, "Medieval Fantasy Kingdom");
        assert!(dna.validate().is_ok());
    }
}
