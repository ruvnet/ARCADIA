//! Game Components module for ARCADIA
//! Defines all game elements including CodeDNA, components, and game systems

pub mod code_dna;
pub mod components;
pub mod game_world;

pub use code_dna::CodeDNA;
pub use components::*;
pub use game_world::GameWorld;
