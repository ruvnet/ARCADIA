//! Learning Database Module
//!
//! Vector-based learning database for storing and querying agent experiences
//! using semantic similarity and pattern recognition.

use super::{AgentDbConfig, AgentDbError, AgentExperience};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Learning pattern type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    /// Successful behavior pattern
    Success,
    /// Failed behavior pattern
    Failure,
    /// Exploration pattern
    Exploration,
    /// Exploitation pattern
    Exploitation,
    /// Anomaly pattern
    Anomaly,
}

/// Learning pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPattern {
    pub id: String,
    pub pattern_type: PatternType,
    pub experiences: Vec<String>, // Experience IDs
    pub centroid: Vec<f32>,        // Center of pattern cluster
    pub confidence: f32,
    pub occurrence_count: usize,
    pub avg_reward: f32,
    pub timestamp: i64,
}

/// Learning database for vector-based experience storage
pub struct LearningDatabase {
    config: AgentDbConfig,
    /// All experiences indexed by ID
    experiences: HashMap<String, AgentExperience>,
    /// Detected learning patterns
    patterns: HashMap<String, LearningPattern>,
    /// Vector index for similarity search (simplified)
    vector_index: Vec<(String, Vec<f32>)>, // (experience_id, vector)
    stats: LearningStats,
}

impl LearningDatabase {
    /// Create a new learning database
    pub async fn new(config: &AgentDbConfig) -> Result<Self, AgentDbError> {
        Ok(Self {
            config: config.clone(),
            experiences: HashMap::new(),
            patterns: HashMap::new(),
            vector_index: Vec::new(),
            stats: LearningStats::default(),
        })
    }

    /// Initialize the database
    pub async fn initialize(&mut self) -> Result<(), AgentDbError> {
        self.stats.initialized = true;
        Ok(())
    }

    /// Add an experience to the learning database
    pub async fn add_experience(&mut self, experience: &AgentExperience) -> Result<(), AgentDbError> {
        // Store experience
        self.experiences
            .insert(experience.id.clone(), experience.clone());

        // Add to vector index
        self.vector_index
            .push((experience.id.clone(), experience.state_vector.clone()));

        // Update statistics
        self.stats.total_experiences += 1;
        self.stats.total_reward += experience.reward;

        // Detect patterns periodically
        if self.experiences.len() % 100 == 0 {
            self.detect_patterns().await?;
        }

        Ok(())
    }

    /// Query similar experiences using vector similarity
    pub async fn query_similar(
        &self,
        query_vector: Vec<f32>,
        limit: usize,
    ) -> Result<Vec<AgentExperience>, AgentDbError> {
        if query_vector.len() != self.config.vector_dim {
            return Err(AgentDbError::QueryError(format!(
                "Query vector dimension {} does not match configured dimension {}",
                query_vector.len(),
                self.config.vector_dim
            )));
        }

        // Calculate similarity scores
        let mut scores: Vec<(f32, String)> = self
            .vector_index
            .iter()
            .map(|(id, vec)| {
                let similarity = cosine_similarity(&query_vector, vec);
                (similarity, id.clone())
            })
            .collect();

        // Sort by similarity descending
        scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        // Return top K experiences
        let results: Vec<AgentExperience> = scores
            .iter()
            .take(limit)
            .filter_map(|(_, id)| self.experiences.get(id).cloned())
            .collect();

        Ok(results)
    }

    /// Get experiences by pattern type
    pub async fn get_by_pattern(
        &self,
        pattern_type: PatternType,
    ) -> Result<Vec<AgentExperience>, AgentDbError> {
        let mut results = Vec::new();

        for pattern in self.patterns.values() {
            if std::mem::discriminant(&pattern.pattern_type) == std::mem::discriminant(&pattern_type) {
                for exp_id in &pattern.experiences {
                    if let Some(exp) = self.experiences.get(exp_id) {
                        results.push(exp.clone());
                    }
                }
            }
        }

        Ok(results)
    }

    /// Get successful experiences (positive reward)
    pub async fn get_successful_experiences(&self) -> Result<Vec<AgentExperience>, AgentDbError> {
        Ok(self
            .experiences
            .values()
            .filter(|exp| exp.reward > 0.0)
            .cloned()
            .collect())
    }

    /// Get failed experiences (negative reward)
    pub async fn get_failed_experiences(&self) -> Result<Vec<AgentExperience>, AgentDbError> {
        Ok(self
            .experiences
            .values()
            .filter(|exp| exp.reward < 0.0)
            .cloned()
            .collect())
    }

    /// Detect learning patterns using clustering
    async fn detect_patterns(&mut self) -> Result<(), AgentDbError> {
        // Simplified pattern detection based on reward and action
        let mut action_rewards: HashMap<String, Vec<f32>> = HashMap::new();

        for exp in self.experiences.values() {
            action_rewards
                .entry(exp.action.clone())
                .or_insert_with(Vec::new)
                .push(exp.reward);
        }

        // Identify successful patterns
        for (action, rewards) in action_rewards.iter() {
            let avg_reward: f32 = rewards.iter().sum::<f32>() / rewards.len() as f32;

            if avg_reward > 0.5 {
                // Success pattern
                let pattern_id = format!("pattern_success_{}_{}", action, chrono::Utc::now().timestamp());

                // Find experiences with this action
                let exp_ids: Vec<String> = self
                    .experiences
                    .values()
                    .filter(|exp| exp.action == *action && exp.reward > 0.0)
                    .map(|exp| exp.id.clone())
                    .collect();

                // Calculate centroid
                let centroid = self.calculate_centroid(&exp_ids)?;

                let pattern = LearningPattern {
                    id: pattern_id.clone(),
                    pattern_type: PatternType::Success,
                    experiences: exp_ids.clone(),
                    centroid,
                    confidence: 0.8,
                    occurrence_count: exp_ids.len(),
                    avg_reward,
                    timestamp: chrono::Utc::now().timestamp(),
                };

                self.patterns.insert(pattern_id, pattern);
                self.stats.patterns_detected += 1;
            } else if avg_reward < -0.5 {
                // Failure pattern
                let pattern_id = format!("pattern_failure_{}_{}", action, chrono::Utc::now().timestamp());

                let exp_ids: Vec<String> = self
                    .experiences
                    .values()
                    .filter(|exp| exp.action == *action && exp.reward < 0.0)
                    .map(|exp| exp.id.clone())
                    .collect();

                let centroid = self.calculate_centroid(&exp_ids)?;

                let pattern = LearningPattern {
                    id: pattern_id.clone(),
                    pattern_type: PatternType::Failure,
                    experiences: exp_ids.clone(),
                    centroid,
                    confidence: 0.7,
                    occurrence_count: exp_ids.len(),
                    avg_reward,
                    timestamp: chrono::Utc::now().timestamp(),
                };

                self.patterns.insert(pattern_id, pattern);
                self.stats.patterns_detected += 1;
            }
        }

        Ok(())
    }

    /// Calculate centroid of experience vectors
    fn calculate_centroid(&self, exp_ids: &[String]) -> Result<Vec<f32>, AgentDbError> {
        if exp_ids.is_empty() {
            return Ok(vec![0.0; self.config.vector_dim]);
        }

        let mut centroid = vec![0.0; self.config.vector_dim];

        for exp_id in exp_ids {
            if let Some(exp) = self.experiences.get(exp_id) {
                for (i, val) in exp.state_vector.iter().enumerate() {
                    if i < centroid.len() {
                        centroid[i] += val;
                    }
                }
            }
        }

        // Average
        let count = exp_ids.len() as f32;
        for val in centroid.iter_mut() {
            *val /= count;
        }

        Ok(centroid)
    }

    /// Get learning patterns
    pub async fn get_patterns(&self) -> Vec<LearningPattern> {
        self.patterns.values().cloned().collect()
    }

    /// Get statistics
    pub fn get_stats(&self) -> LearningStats {
        self.stats.clone()
    }

    /// Save database
    pub async fn save(&self) -> Result<(), AgentDbError> {
        // TODO: Implement persistent storage
        Ok(())
    }

    /// Shutdown and cleanup
    pub async fn shutdown(&mut self) -> Result<(), AgentDbError> {
        self.save().await?;
        self.experiences.clear();
        self.patterns.clear();
        self.vector_index.clear();
        Ok(())
    }
}

/// Learning statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LearningStats {
    pub initialized: bool,
    pub total_experiences: usize,
    pub patterns_detected: usize,
    pub total_reward: f32,
    pub avg_reward: f32,
}

impl LearningStats {
    /// Update average reward
    pub fn update_avg_reward(&mut self) {
        if self.total_experiences > 0 {
            self.avg_reward = self.total_reward / self.total_experiences as f32;
        }
    }
}

/// Calculate cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let magnitude_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let magnitude_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        return 0.0;
    }

    dot_product / (magnitude_a * magnitude_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_learning_database_creation() {
        let config = AgentDbConfig::default();
        let db = LearningDatabase::new(&config).await;
        assert!(db.is_ok());
    }

    #[tokio::test]
    async fn test_add_and_query() {
        let config = AgentDbConfig {
            vector_dim: 10,
            ..Default::default()
        };

        let mut db = LearningDatabase::new(&config).await.unwrap();
        db.initialize().await.unwrap();

        let experience = AgentExperience {
            id: "exp_1".to_string(),
            agent_id: "agent_1".to_string(),
            state_vector: vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            action: "test".to_string(),
            reward: 1.0,
            next_state_vector: vec![0.0; 10],
            done: false,
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now().timestamp(),
        };

        db.add_experience(&experience).await.unwrap();

        let query = vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let results = db.query_similar(query, 1).await.unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "exp_1");
    }

    #[tokio::test]
    async fn test_pattern_detection() {
        let config = AgentDbConfig {
            vector_dim: 10,
            ..Default::default()
        };

        let mut db = LearningDatabase::new(&config).await.unwrap();
        db.initialize().await.unwrap();

        // Add multiple experiences with positive rewards
        for i in 0..150 {
            let experience = AgentExperience {
                id: format!("exp_{}", i),
                agent_id: "agent_1".to_string(),
                state_vector: vec![1.0; 10],
                action: "forward".to_string(),
                reward: 1.0,
                next_state_vector: vec![0.0; 10],
                done: false,
                metadata: HashMap::new(),
                timestamp: chrono::Utc::now().timestamp(),
            };

            db.add_experience(&experience).await.unwrap();
        }

        let patterns = db.get_patterns().await;
        assert!(!patterns.is_empty());
    }
}
