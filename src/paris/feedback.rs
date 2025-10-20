// PARIS Framework - Regenerative Feedback Loop Module
// Handles feedback collection, analysis, and regenerative processes

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Feedback configuration
#[derive(Debug, Clone)]
pub struct FeedbackConfig {
    pub max_queue_size: usize,
    pub feedback_types: Vec<FeedbackType>,
    pub aggregation_interval_ms: u64,
    pub enable_filtering: bool,
    pub priority_threshold: f32,
}

/// Feedback types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeedbackType {
    /// Player behavior feedback
    PlayerBehavior,
    /// Performance metrics
    Performance,
    /// Game world state
    WorldState,
    /// AI decision quality
    AIDecision,
    /// User experience
    UserExperience,
    /// System health
    SystemHealth,
}

/// Feedback data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackData {
    pub id: String,
    pub feedback_type: FeedbackType,
    pub source: String,
    pub timestamp: i64,
    pub priority: f32,
    pub data: HashMap<String, f32>,
    pub metadata: HashMap<String, String>,
}

/// Aggregated feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedFeedback {
    pub feedback_type: FeedbackType,
    pub count: usize,
    pub time_window_ms: u64,
    pub metrics: HashMap<String, MetricAggregate>,
}

/// Metric aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAggregate {
    pub mean: f32,
    pub min: f32,
    pub max: f32,
    pub std_dev: f32,
    pub count: usize,
}

/// Feedback Manager
///
/// Manages regenerative feedback loops, collecting and processing
/// data from various sources for continuous system improvement.
pub struct FeedbackManager {
    config: FeedbackConfig,
    feedback_queue: VecDeque<FeedbackData>,
    aggregated_data: HashMap<FeedbackType, Vec<AggregatedFeedback>>,
    stats: FeedbackStats,
}

impl FeedbackManager {
    /// Create a new feedback manager
    pub async fn new(config: FeedbackConfig) -> Result<Self, FeedbackError> {
        Ok(Self {
            config,
            feedback_queue: VecDeque::new(),
            aggregated_data: HashMap::new(),
            stats: FeedbackStats::default(),
        })
    }

    /// Initialize the feedback manager
    pub async fn initialize(&mut self) -> Result<(), FeedbackError> {
        // Initialize aggregated data structures for each feedback type
        for feedback_type in &self.config.feedback_types {
            self.aggregated_data.insert(*feedback_type, Vec::new());
        }

        Ok(())
    }

    /// Submit feedback data
    pub async fn submit_feedback(&mut self, feedback: FeedbackData) -> Result<(), FeedbackError> {
        // Apply filtering if enabled
        if self.config.enable_filtering && feedback.priority < self.config.priority_threshold {
            self.stats.filtered_count += 1;
            return Ok(());
        }

        // Check queue size
        if self.feedback_queue.len() >= self.config.max_queue_size {
            // Remove oldest feedback
            self.feedback_queue.pop_front();
            self.stats.evicted_count += 1;
        }

        self.feedback_queue.push_back(feedback);
        self.stats.total_submitted += 1;

        Ok(())
    }

    /// Collect all pending feedback
    pub async fn collect_feedback(&self) -> Result<Vec<FeedbackData>, FeedbackError> {
        Ok(self.feedback_queue.iter().cloned().collect())
    }

    /// Process and aggregate feedback
    pub async fn process_feedback(&mut self) -> Result<Vec<AggregatedFeedback>, FeedbackError> {
        let mut results = Vec::new();

        // Group feedback by type
        let mut grouped: HashMap<FeedbackType, Vec<FeedbackData>> = HashMap::new();

        for feedback in &self.feedback_queue {
            grouped
                .entry(feedback.feedback_type)
                .or_insert_with(Vec::new)
                .push(feedback.clone());
        }

        // Aggregate each group
        for (feedback_type, data_points) in grouped {
            if let Some(aggregated) = self.aggregate_feedback(&data_points).await? {
                // Store aggregated data
                if let Some(agg_list) = self.aggregated_data.get_mut(&feedback_type) {
                    agg_list.push(aggregated.clone());
                }

                results.push(aggregated);
            }
        }

        // Clear processed feedback
        self.feedback_queue.clear();
        self.stats.total_processed += results.len() as u64;

        Ok(results)
    }

    /// Aggregate feedback data points
    async fn aggregate_feedback(
        &self,
        data_points: &[FeedbackData],
    ) -> Result<Option<AggregatedFeedback>, FeedbackError> {
        if data_points.is_empty() {
            return Ok(None);
        }

        let feedback_type = data_points[0].feedback_type;
        let mut metric_values: HashMap<String, Vec<f32>> = HashMap::new();

        // Collect all metric values
        for feedback in data_points {
            for (key, value) in &feedback.data {
                metric_values
                    .entry(key.clone())
                    .or_insert_with(Vec::new)
                    .push(*value);
            }
        }

        // Compute aggregates for each metric
        let mut metrics = HashMap::new();
        for (key, values) in metric_values {
            let aggregate = self.compute_aggregate(&values);
            metrics.insert(key, aggregate);
        }

        Ok(Some(AggregatedFeedback {
            feedback_type,
            count: data_points.len(),
            time_window_ms: self.config.aggregation_interval_ms,
            metrics,
        }))
    }

    /// Compute metric aggregate
    fn compute_aggregate(&self, values: &[f32]) -> MetricAggregate {
        if values.is_empty() {
            return MetricAggregate {
                mean: 0.0,
                min: 0.0,
                max: 0.0,
                std_dev: 0.0,
                count: 0,
            };
        }

        let count = values.len();
        let sum: f32 = values.iter().sum();
        let mean = sum / count as f32;

        let min = values.iter().copied().fold(f32::INFINITY, f32::min);
        let max = values.iter().copied().fold(f32::NEG_INFINITY, f32::max);

        let variance = values.iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f32>() / count as f32;
        let std_dev = variance.sqrt();

        MetricAggregate {
            mean,
            min,
            max,
            std_dev,
            count,
        }
    }

    /// Get feedback history for a type
    pub async fn get_history(
        &self,
        feedback_type: FeedbackType,
    ) -> Result<Vec<AggregatedFeedback>, FeedbackError> {
        Ok(self.aggregated_data
            .get(&feedback_type)
            .map(|v| v.clone())
            .unwrap_or_default())
    }

    /// Clear feedback history
    pub async fn clear_history(&mut self, feedback_type: Option<FeedbackType>) -> Result<(), FeedbackError> {
        if let Some(ft) = feedback_type {
            if let Some(history) = self.aggregated_data.get_mut(&ft) {
                history.clear();
            }
        } else {
            for history in self.aggregated_data.values_mut() {
                history.clear();
            }
        }

        Ok(())
    }

    /// Get feedback statistics
    pub fn get_stats(&self) -> FeedbackStats {
        self.stats.clone()
    }

    /// Shutdown the feedback manager
    pub async fn shutdown(&mut self) -> Result<(), FeedbackError> {
        self.feedback_queue.clear();
        self.aggregated_data.clear();
        Ok(())
    }
}

/// Feedback statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FeedbackStats {
    pub total_submitted: u64,
    pub total_processed: u64,
    pub filtered_count: u64,
    pub evicted_count: u64,
}

/// Feedback error types
#[derive(Debug, thiserror::Error)]
pub enum FeedbackError {
    #[error("Queue full")]
    QueueFull,

    #[error("Invalid feedback: {0}")]
    InvalidFeedback(String),

    #[error("Aggregation failed: {0}")]
    AggregationFailed(String),

    #[error("Processing failed: {0}")]
    ProcessingFailed(String),
}
