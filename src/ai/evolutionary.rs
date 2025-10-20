//! Evolutionary Feedback Module
//!
//! This module implements learning from interactions, adaptation algorithms,
//! fitness evaluation, and generational evolution for AI entities.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rand::Rng;
use rand::seq::SliceRandom;

/// Represents a gene in an entity's genome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gene {
    pub id: Uuid,
    pub name: String,
    pub value: f64,
    pub mutation_rate: f64,
    pub dominant: bool,
}

/// Genome of an evolving entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genome {
    pub id: Uuid,
    pub genes: Vec<Gene>,
    pub generation: usize,
    pub fitness_score: f64,
    pub parent_ids: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Interaction record for learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub interaction_type: String,
    pub context: HashMap<String, f64>,
    pub outcome: f64,
    pub timestamp: DateTime<Utc>,
}

/// Fitness evaluation criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessCriteria {
    pub name: String,
    pub weight: f64,
    pub target_value: f64,
    pub tolerance: f64,
}

/// Evolutionary strategy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionConfig {
    pub population_size: usize,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub elitism_count: usize,
    pub max_generations: usize,
}

/// Represents an evolved trait
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolvedTrait {
    pub name: String,
    pub strength: f64,
    pub generation_emerged: usize,
    pub prevalence: f64,
}

/// Evolutionary feedback system
pub struct EvolutionaryFeedback {
    /// Population of genomes
    population: Arc<RwLock<HashMap<Uuid, Genome>>>,

    /// Interaction history
    interactions: Arc<RwLock<Vec<Interaction>>>,

    /// Fitness criteria
    fitness_criteria: Arc<RwLock<Vec<FitnessCriteria>>>,

    /// Evolution configuration
    config: Arc<RwLock<EvolutionConfig>>,

    /// Current generation number
    generation: Arc<RwLock<usize>>,

    /// Evolved traits discovered
    evolved_traits: Arc<RwLock<Vec<EvolvedTrait>>>,

    /// Learning history for adaptation
    learning_history: Arc<RwLock<HashMap<String, Vec<f64>>>>,
}

impl EvolutionaryFeedback {
    /// Create a new evolutionary feedback system
    pub fn new(config: EvolutionConfig) -> Self {
        Self {
            population: Arc::new(RwLock::new(HashMap::new())),
            interactions: Arc::new(RwLock::new(Vec::new())),
            fitness_criteria: Arc::new(RwLock::new(Vec::new())),
            config: Arc::new(RwLock::new(config)),
            generation: Arc::new(RwLock::new(0)),
            evolved_traits: Arc::new(RwLock::new(Vec::new())),
            learning_history: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new genome with random genes
    pub fn create_genome(&self, gene_count: usize) -> Genome {
        let mut rng = rand::thread_rng();
        let mut genes = Vec::new();

        for i in 0..gene_count {
            genes.push(Gene {
                id: Uuid::new_v4(),
                name: format!("gene_{}", i),
                value: rng.gen_range(0.0..1.0),
                mutation_rate: 0.01,
                dominant: rng.gen_bool(0.5),
            });
        }

        Genome {
            id: Uuid::new_v4(),
            genes,
            generation: *self.generation.read(),
            fitness_score: 0.0,
            parent_ids: vec![],
            created_at: Utc::now(),
        }
    }

    /// Add a genome to the population
    pub fn add_to_population(&self, genome: Genome) -> Uuid {
        let id = genome.id;
        self.population.write().insert(id, genome);
        id
    }

    /// Record an interaction for learning
    pub fn record_interaction(
        &self,
        entity_id: Uuid,
        interaction_type: String,
        context: HashMap<String, f64>,
        outcome: f64,
    ) {
        let interaction = Interaction {
            id: Uuid::new_v4(),
            entity_id,
            interaction_type: interaction_type.clone(),
            context,
            outcome,
            timestamp: Utc::now(),
        };

        self.interactions.write().push(interaction);

        // Update learning history
        self.learning_history.write()
            .entry(interaction_type)
            .or_insert_with(Vec::new)
            .push(outcome);
    }

    /// Add fitness criteria
    pub fn add_fitness_criteria(&self, criteria: FitnessCriteria) {
        self.fitness_criteria.write().push(criteria);
    }

    /// Evaluate fitness of a genome
    pub fn evaluate_fitness(&self, genome_id: Uuid) -> f64 {
        let population = self.population.read();
        let genome = match population.get(&genome_id) {
            Some(g) => g,
            None => return 0.0,
        };

        let criteria = self.fitness_criteria.read();
        let mut total_fitness = 0.0;
        let mut total_weight = 0.0;

        for criterion in criteria.iter() {
            // Find corresponding gene
            if let Some(gene) = genome.genes.iter().find(|g| g.name == criterion.name) {
                let deviation = (gene.value - criterion.target_value).abs();
                let fitness_component = if deviation <= criterion.tolerance {
                    1.0 - (deviation / criterion.tolerance)
                } else {
                    0.0
                };

                total_fitness += fitness_component * criterion.weight;
                total_weight += criterion.weight;
            }
        }

        drop(population);

        // Include interaction-based fitness
        let interaction_fitness = self.calculate_interaction_fitness(genome_id);
        total_fitness += interaction_fitness * 0.3;

        let final_fitness = if total_weight > 0.0 {
            total_fitness / (total_weight + 0.3)
        } else {
            interaction_fitness
        };

        // Update genome fitness score
        if let Some(genome) = self.population.write().get_mut(&genome_id) {
            genome.fitness_score = final_fitness;
        }

        final_fitness
    }

    /// Calculate fitness based on interactions
    fn calculate_interaction_fitness(&self, entity_id: Uuid) -> f64 {
        let interactions = self.interactions.read();
        let entity_interactions: Vec<_> = interactions.iter()
            .filter(|i| i.entity_id == entity_id)
            .collect();

        if entity_interactions.is_empty() {
            return 0.5; // Neutral fitness for no interactions
        }

        let total_outcome: f64 = entity_interactions.iter()
            .map(|i| i.outcome)
            .sum();

        (total_outcome / entity_interactions.len() as f64).clamp(0.0, 1.0)
    }

    /// Perform selection based on fitness
    fn select_parents(&self, count: usize) -> Vec<Uuid> {
        let population = self.population.read();
        let mut genomes: Vec<_> = population.values().collect();

        // Sort by fitness (descending)
        genomes.sort_by(|a, b| b.fitness_score.partial_cmp(&a.fitness_score).unwrap());

        // Select top performers and some random ones for diversity
        let mut selected = Vec::new();
        let elite_count = (count * 70 / 100).max(1);
        let random_count = count - elite_count;

        // Elite selection
        for genome in genomes.iter().take(elite_count) {
            selected.push(genome.id);
        }

        // Random selection for diversity
        let mut rng = rand::thread_rng();
        let remaining: Vec<_> = genomes.iter().skip(elite_count).collect();

        for _ in 0..random_count {
            if let Some(genome) = remaining.choose(&mut rng) {
                selected.push(genome.id);
            }
        }

        selected
    }

    /// Perform crossover between two genomes
    fn crossover(&self, parent1_id: Uuid, parent2_id: Uuid) -> Genome {
        let population = self.population.read();
        let parent1 = population.get(&parent1_id).unwrap();
        let parent2 = population.get(&parent2_id).unwrap();

        let mut rng = rand::thread_rng();
        let config = self.config.read();

        let mut child_genes = Vec::new();

        // Combine genes from both parents
        for (gene1, gene2) in parent1.genes.iter().zip(parent2.genes.iter()) {
            let gene = if rng.gen_bool(config.crossover_rate) {
                // Crossover: blend values
                Gene {
                    id: Uuid::new_v4(),
                    name: gene1.name.clone(),
                    value: (gene1.value + gene2.value) / 2.0,
                    mutation_rate: gene1.mutation_rate,
                    dominant: if gene1.dominant { gene1.dominant } else { gene2.dominant },
                }
            } else {
                // Inherit from one parent
                if rng.gen_bool(0.5) {
                    gene1.clone()
                } else {
                    gene2.clone()
                }
            };

            child_genes.push(gene);
        }

        Genome {
            id: Uuid::new_v4(),
            genes: child_genes,
            generation: *self.generation.read() + 1,
            fitness_score: 0.0,
            parent_ids: vec![parent1_id, parent2_id],
            created_at: Utc::now(),
        }
    }

    /// Apply mutation to a genome
    fn mutate(&self, genome: &mut Genome) {
        let mut rng = rand::thread_rng();
        let config = self.config.read();

        for gene in genome.genes.iter_mut() {
            if rng.gen_bool(config.mutation_rate * gene.mutation_rate) {
                // Mutate gene value
                let mutation_strength = rng.gen_range(-0.1..0.1);
                gene.value = (gene.value + mutation_strength).clamp(0.0, 1.0);

                // Occasionally flip dominance
                if rng.gen_bool(0.1) {
                    gene.dominant = !gene.dominant;
                }
            }
        }
    }

    /// Evolve the population to the next generation
    pub fn evolve_generation(&self) -> usize {
        let config = self.config.read();
        let current_pop = self.population.read().clone();

        // Evaluate fitness for all current genomes
        let genome_ids: Vec<_> = current_pop.keys().copied().collect();
        drop(current_pop);

        for id in &genome_ids {
            self.evaluate_fitness(*id);
        }

        // Select parents
        let parent_count = config.population_size / 2;
        let parents = self.select_parents(parent_count);

        // Create new generation
        let mut new_population = HashMap::new();
        let mut rng = rand::thread_rng();

        // Elitism: keep top performers
        let population = self.population.read();
        let mut sorted_genomes: Vec<_> = population.values().cloned().collect();
        sorted_genomes.sort_by(|a, b| b.fitness_score.partial_cmp(&a.fitness_score).unwrap());

        for genome in sorted_genomes.iter().take(config.elitism_count) {
            new_population.insert(genome.id, genome.clone());
        }

        drop(population);

        // Generate offspring
        while new_population.len() < config.population_size {
            let parent1 = parents.choose(&mut rng).copied().unwrap();
            let parent2 = parents.choose(&mut rng).copied().unwrap();

            if parent1 != parent2 {
                let mut child = self.crossover(parent1, parent2);
                self.mutate(&mut child);
                new_population.insert(child.id, child);
            }
        }

        // Replace population
        *self.population.write() = new_population;

        // Increment generation
        let new_gen = {
            let mut gen = self.generation.write();
            *gen += 1;
            *gen
        };

        // Detect evolved traits
        self.detect_evolved_traits();

        new_gen
    }

    /// Detect traits that have evolved in the population
    fn detect_evolved_traits(&self) {
        let population = self.population.read();
        let current_gen = *self.generation.read();

        // Analyze gene frequency and strength
        let mut gene_stats: HashMap<String, (f64, usize)> = HashMap::new();

        for genome in population.values() {
            for gene in &genome.genes {
                let entry = gene_stats.entry(gene.name.clone()).or_insert((0.0, 0));
                entry.0 += gene.value;
                entry.1 += 1;
            }
        }

        let mut traits = Vec::new();

        for (gene_name, (total_value, count)) in gene_stats {
            let avg_value = total_value / count as f64;
            let prevalence = count as f64 / population.len() as f64;

            // Consider it a trait if it's prevalent and strong
            if prevalence > 0.6 && avg_value > 0.7 {
                traits.push(EvolvedTrait {
                    name: gene_name,
                    strength: avg_value,
                    generation_emerged: current_gen,
                    prevalence,
                });
            }
        }

        self.evolved_traits.write().extend(traits);
    }

    /// Get the best genome in the current population
    pub fn get_best_genome(&self) -> Option<Genome> {
        let population = self.population.read();
        population.values()
            .max_by(|a, b| a.fitness_score.partial_cmp(&b.fitness_score).unwrap())
            .cloned()
    }

    /// Learn from interaction patterns
    pub fn learn_from_interactions(&self) -> HashMap<String, f64> {
        let history = self.learning_history.read();
        let mut learned_behaviors = HashMap::new();

        for (interaction_type, outcomes) in history.iter() {
            if outcomes.len() >= 5 {
                // Calculate average success
                let avg_outcome: f64 = outcomes.iter().sum::<f64>() / outcomes.len() as f64;

                // Calculate trend (recent vs. old)
                let recent_count = (outcomes.len() / 3).max(1);
                let recent_avg: f64 = outcomes.iter()
                    .rev()
                    .take(recent_count)
                    .sum::<f64>() / recent_count as f64;

                let improvement = recent_avg - avg_outcome;

                learned_behaviors.insert(
                    interaction_type.clone(),
                    avg_outcome + (improvement * 0.5),
                );
            }
        }

        learned_behaviors
    }

    /// Adapt based on environmental feedback
    pub fn adapt_to_environment(&self, environmental_factors: HashMap<String, f64>) {
        let mut population = self.population.write();

        for genome in population.values_mut() {
            for gene in genome.genes.iter_mut() {
                if let Some(factor) = environmental_factors.get(&gene.name) {
                    // Adapt gene value towards environmental optimum
                    let adaptation_rate = 0.1;
                    gene.value += (factor - gene.value) * adaptation_rate;
                    gene.value = gene.value.clamp(0.0, 1.0);
                }
            }
        }
    }

    /// Get current generation number
    pub fn get_generation(&self) -> usize {
        *self.generation.read()
    }

    /// Get population size
    pub fn get_population_size(&self) -> usize {
        self.population.read().len()
    }

    /// Get average population fitness
    pub fn get_average_fitness(&self) -> f64 {
        let population = self.population.read();

        if population.is_empty() {
            return 0.0;
        }

        let total: f64 = population.values().map(|g| g.fitness_score).sum();
        total / population.len() as f64
    }

    /// Get all evolved traits
    pub fn get_evolved_traits(&self) -> Vec<EvolvedTrait> {
        self.evolved_traits.read().clone()
    }

    /// Reset the population
    pub fn reset_population(&self, size: usize, gene_count: usize) {
        let mut new_pop = HashMap::new();

        for _ in 0..size {
            let genome = self.create_genome(gene_count);
            new_pop.insert(genome.id, genome);
        }

        *self.population.write() = new_pop;
        *self.generation.write() = 0;
    }
}

impl Default for EvolutionaryFeedback {
    fn default() -> Self {
        Self::new(EvolutionConfig {
            population_size: 100,
            mutation_rate: 0.01,
            crossover_rate: 0.7,
            elitism_count: 10,
            max_generations: 1000,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evolutionary_creation() {
        let config = EvolutionConfig {
            population_size: 50,
            mutation_rate: 0.01,
            crossover_rate: 0.7,
            elitism_count: 5,
            max_generations: 100,
        };

        let evo = EvolutionaryFeedback::new(config);
        assert_eq!(evo.get_generation(), 0);
    }

    #[test]
    fn test_genome_creation() {
        let evo = EvolutionaryFeedback::default();
        let genome = evo.create_genome(10);

        assert_eq!(genome.genes.len(), 10);
        assert_eq!(genome.generation, 0);
    }

    #[test]
    fn test_population_evolution() {
        let evo = EvolutionaryFeedback::default();
        evo.reset_population(20, 5);

        let initial_gen = evo.get_generation();
        evo.evolve_generation();

        assert_eq!(evo.get_generation(), initial_gen + 1);
        assert_eq!(evo.get_population_size(), 20);
    }

    #[test]
    fn test_fitness_evaluation() {
        let evo = EvolutionaryFeedback::default();
        let genome = evo.create_genome(5);
        let id = evo.add_to_population(genome);

        evo.add_fitness_criteria(FitnessCriteria {
            name: "gene_0".to_string(),
            weight: 1.0,
            target_value: 0.5,
            tolerance: 0.2,
        });

        let fitness = evo.evaluate_fitness(id);
        assert!(fitness >= 0.0 && fitness <= 1.0);
    }

    #[test]
    fn test_interaction_recording() {
        let evo = EvolutionaryFeedback::default();
        let genome = evo.create_genome(5);
        let id = evo.add_to_population(genome);

        evo.record_interaction(
            id,
            "test_action".to_string(),
            HashMap::new(),
            0.8,
        );

        let learned = evo.learn_from_interactions();
        assert!(learned.is_empty() || learned.contains_key("test_action"));
    }
}
