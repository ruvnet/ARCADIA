// PARIS Framework - Adaptive Learning Systems Module
// Handles continuous learning, pattern recognition, and adaptation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::feedback::AggregatedFeedback;

/// Learning configuration
#[derive(Debug, Clone)]
pub struct LearningConfig {
    pub learning_rate: f32,
    pub adaptation_threshold: f32,
    pub pattern_window_size: usize,
    pub enable_online_learning: bool,
    pub enable_transfer_learning: bool,
    pub model_update_frequency: u64,
}

/// Learning algorithm types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LearningAlgorithm {
    /// Supervised learning
    Supervised,
    /// Unsupervised learning
    Unsupervised,
    /// Reinforcement learning
    Reinforcement,
    /// Transfer learning
    Transfer,
    /// Meta-learning
    Meta,
}

/// Learning pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPattern {
    pub id: String,
    pub pattern_type: String,
    pub confidence: f32,
    pub occurrences: usize,
    pub features: HashMap<String, f32>,
    pub timestamp: i64,
}

/// Learning result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningResult {
    pub update_count: usize,
    pub patterns_discovered: Vec<LearningPattern>,
    pub model_updates: Vec<ModelUpdate>,
    pub performance_delta: f32,
}

/// Model update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelUpdate {
    pub model_id: String,
    pub parameter_updates: HashMap<String, f32>,
    pub timestamp: i64,
}

/// Learning Manager
///
/// Manages adaptive learning systems, pattern recognition,
/// and continuous model improvement for the PARIS framework.
pub struct LearningManager {
    config: LearningConfig,
    patterns: HashMap<String, LearningPattern>,
    models: HashMap<String, AIModel>,
    learning_history: Vec<LearningResult>,
    stats: LearningStats,
}

impl LearningManager {
    /// Create a new learning manager
    pub async fn new(config: LearningConfig) -> Result<Self, LearningError> {
        Ok(Self {
            config,
            patterns: HashMap::new(),
            models: HashMap::new(),
            learning_history: Vec::new(),
            stats: LearningStats::default(),
        })
    }

    /// Initialize the learning manager
    pub async fn initialize(&mut self) -> Result<(), LearningError> {
        // Initialize default models
        self.register_model(
            "player_behavior".to_string(),
            AIModel::new(LearningAlgorithm::Supervised),
        ).await?;

        self.register_model(
            "world_generation".to_string(),
            AIModel::new(LearningAlgorithm::Reinforcement),
        ).await?;

        Ok(())
    }

    /// Register an AI model
    pub async fn register_model(
        &mut self,
        model_id: String,
        model: AIModel,
    ) -> Result<(), LearningError> {
        self.models.insert(model_id, model);
        Ok(())
    }

    /// Process feedback and learn
    pub async fn process_feedback(
        &mut self,
        feedback: &[AggregatedFeedback],
    ) -> Result<LearningResult, LearningError> {
        let mut update_count = 0;
        let mut patterns_discovered = Vec::new();
        let mut model_updates = Vec::new();

        // Analyze feedback for patterns
        for fb in feedback {
            if let Some(patterns) = self.detect_patterns(fb).await? {
                patterns_discovered.extend(patterns);
            }
        }

        // Update models based on feedback
        if self.config.enable_online_learning {
            for (model_id, model) in self.models.iter_mut() {
                if let Some(update) = model.update_from_feedback(feedback, &self.config).await? {
                    model_updates.push(ModelUpdate {
                        model_id: model_id.clone(),
                        parameter_updates: update,
                        timestamp: Self::current_timestamp(),
                    });
                    update_count += 1;
                }
            }
        }

        // Store discovered patterns
        for pattern in &patterns_discovered {
            self.patterns.insert(pattern.id.clone(), pattern.clone());
        }

        let result = LearningResult {
            update_count,
            patterns_discovered,
            model_updates,
            performance_delta: self.compute_performance_delta(),
        };

        self.learning_history.push(result.clone());
        self.stats.total_learning_cycles += 1;
        self.stats.patterns_discovered += result.patterns_discovered.len() as u64;

        Ok(result)
    }

    /// Detect patterns in feedback
    async fn detect_patterns(
        &self,
        feedback: &AggregatedFeedback,
    ) -> Result<Option<Vec<LearningPattern>>, LearningError> {
        let mut patterns = Vec::new();

        // Analyze metrics for patterns
        for (metric_name, aggregate) in &feedback.metrics {
            // Detect anomalies
            if aggregate.std_dev > self.config.adaptation_threshold {
                patterns.push(LearningPattern {
                    id: format!("{}_{}", metric_name, Self::current_timestamp()),
                    pattern_type: "anomaly".to_string(),
                    confidence: 0.8,
                    occurrences: aggregate.count,
                    features: HashMap::from([
                        ("mean".to_string(), aggregate.mean),
                        ("std_dev".to_string(), aggregate.std_dev),
                    ]),
                    timestamp: Self::current_timestamp(),
                });
            }

            // Detect trends
            if aggregate.mean > 0.7 {
                patterns.push(LearningPattern {
                    id: format!("trend_{}_{}", metric_name, Self::current_timestamp()),
                    pattern_type: "trend".to_string(),
                    confidence: 0.9,
                    occurrences: aggregate.count,
                    features: HashMap::from([
                        ("mean".to_string(), aggregate.mean),
                        ("direction".to_string(), 1.0),
                    ]),
                    timestamp: Self::current_timestamp(),
                });
            }
        }

        Ok(if patterns.is_empty() {
            None
        } else {
            Some(patterns)
        })
    }

    /// Get learned patterns
    pub async fn get_patterns(
        &self,
        pattern_type: Option<String>,
    ) -> Result<Vec<LearningPattern>, LearningError> {
        Ok(self.patterns.values()
            .filter(|p| pattern_type.as_ref().map_or(true, |t| &p.pattern_type == t))
            .cloned()
            .collect())
    }

    /// Apply transfer learning
    pub async fn transfer_learning(
        &mut self,
        source_model: &str,
        target_model: &str,
    ) -> Result<(), LearningError> {
        if !self.config.enable_transfer_learning {
            return Err(LearningError::TransferLearningDisabled);
        }

        if let (Some(source), Some(target)) = (
            self.models.get(source_model),
            self.models.get_mut(target_model),
        ) {
            target.transfer_from(source)?;
            self.stats.transfer_learning_count += 1;
            Ok(())
        } else {
            Err(LearningError::ModelNotFound("Transfer learning failed".to_string()))
        }
    }

    /// Compute performance delta
    fn compute_performance_delta(&self) -> f32 {
        if self.learning_history.len() < 2 {
            return 0.0;
        }

        // Simple delta based on pattern discovery rate
        let recent = &self.learning_history[self.learning_history.len() - 1];
        let previous = &self.learning_history[self.learning_history.len() - 2];

        recent.patterns_discovered.len() as f32 - previous.patterns_discovered.len() as f32
    }

    /// Get learning statistics
    pub fn get_stats(&self) -> LearningStats {
        self.stats.clone()
    }

    /// Shutdown the learning manager
    pub async fn shutdown(&mut self) -> Result<(), LearningError> {
        self.patterns.clear();
        self.models.clear();
        self.learning_history.clear();
        Ok(())
    }

    fn current_timestamp() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64
    }
}

/// AI Model representation
#[derive(Debug, Clone)]
pub struct AIModel {
    algorithm: LearningAlgorithm,
    parameters: HashMap<String, f32>,
    version: u64,
}

impl AIModel {
    pub fn new(algorithm: LearningAlgorithm) -> Self {
        Self {
            algorithm,
            parameters: HashMap::new(),
            version: 1,
        }
    }

    pub async fn update_from_feedback(
        &mut self,
        _feedback: &[AggregatedFeedback],
        config: &LearningConfig,
    ) -> Result<Option<HashMap<String, f32>>, LearningError> {
        // Simulate parameter updates
        let mut updates = HashMap::new();

        // Apply learning rate
        updates.insert("learning_rate".to_string(), config.learning_rate);
        updates.insert("version".to_string(), self.version as f32);

        self.version += 1;

        Ok(Some(updates))
    }

    pub fn transfer_from(&mut self, source: &AIModel) -> Result<(), LearningError> {
        // Copy parameters from source model
        self.parameters = source.parameters.clone();
        self.version = source.version + 1;
        Ok(())
    }
}

/// Learning statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LearningStats {
    pub total_learning_cycles: u64,
    pub patterns_discovered: u64,
    pub transfer_learning_count: u64,
    pub model_update_count: u64,
}

/// Learning error types
#[derive(Debug, thiserror::Error)]
pub enum LearningError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Pattern detection failed: {0}")]
    PatternDetectionFailed(String),

    #[error("Transfer learning is disabled")]
    TransferLearningDisabled,

    #[error("Learning failed: {0}")]
    LearningFailed(String),
}
