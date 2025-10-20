//! GameWorld - The container for all game state and systems

use super::code_dna::{CodeDNA, PhysicsLaw};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameWorld {
    pub setting: String,
    pub technology: String,
    pub physics_laws: Vec<PhysicsLaw>,
    pub themes: Vec<String>,
    pub time_scale: f32,
    pub entropy_rate: f32,
    pub natural_laws: Vec<String>,
    pub entities: HashMap<String, Entity>,
    pub elapsed_time: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub properties: HashMap<String, f32>,
}

impl GameWorld {
    pub fn new() -> Self {
        Self {
            setting: String::new(),
            technology: String::new(),
            physics_laws: vec![],
            themes: vec![],
            time_scale: 1.0,
            entropy_rate: 0.01,
            natural_laws: vec![],
            entities: HashMap::new(),
            elapsed_time: 0.0,
        }
    }
    
    pub fn from_dna(dna: &CodeDNA) -> Self {
        let mut world = Self::new();
        dna.apply_to_world(&mut world);
        world
    }
    
    pub fn set_setting(&mut self, setting: &str) {
        self.setting = setting.to_string();
    }
    
    pub fn set_technology(&mut self, technology: &str) {
        self.technology = technology.to_string();
    }
    
    pub fn set_time_scale(&mut self, time_scale: f32) {
        self.time_scale = time_scale;
    }
    
    pub fn set_entropy_rate(&mut self, entropy_rate: f32) {
        self.entropy_rate = entropy_rate;
    }
    
    pub fn add_physics_law(&mut self, law: PhysicsLaw) {
        self.physics_laws.push(law);
    }
    
    pub fn add_theme(&mut self, theme: String) {
        if !self.themes.contains(&theme) {
            self.themes.push(theme);
        }
    }
    
    pub fn add_natural_law(&mut self, law: String) {
        if !self.natural_laws.contains(&law) {
            self.natural_laws.push(law);
        }
    }
    
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id.clone(), entity);
    }
    
    pub fn update(&mut self, delta_time: f32) {
        self.elapsed_time += delta_time * self.time_scale;
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
    fn test_create_world() {
        let world = GameWorld::new();
        assert_eq!(world.time_scale, 1.0);
    }
    
    #[test]
    fn test_world_from_dna() {
        let dna = CodeDNA::fantasy_default();
        let world = GameWorld::from_dna(&dna);
        assert_eq!(world.setting, "High Fantasy");
    }
}
