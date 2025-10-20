// PARIS Framework - Perpetual Adaptive Regenerative Intelligence System
// Module: Core Framework Entry Point

pub mod feedback;
pub mod learning;
pub mod optimization;
pub mod layers;

use std::sync::Arc;
use tokio::sync::RwLock;

/// PARIS Framework Configuration
#[derive(Debug, Clone)]
pub struct ParisConfig {
    /// Feedback loop configuration
    pub feedback_config: feedback::FeedbackConfig,
    /// Learning system configuration
    pub learning_config: learning::LearningConfig,
    /// Optimization configuration
    pub optimization_config: optimization::OptimizationConfig,
    /// Layer architecture configuration
    pub layer_config: layers::LayerConfig,
}

/// Core PARIS Framework
///
/// Provides perpetual adaptation, regenerative feedback, and
/// continuous self-improvement for AI-driven game systems.
pub struct ParisFramework {
    config: ParisConfig,
    feedback: Arc<RwLock<feedback::FeedbackManager>>,
    learning: Arc<RwLock<learning::LearningManager>>,
    optimization: Arc<RwLock<optimization::OptimizationManager>>,
    layers: Arc<RwLock<layers::LayerManager>>,
}

impl ParisFramework {
    /// Create a new PARIS framework instance
    pub async fn new(config: ParisConfig) -> Result<Self, ParisError> {
        let feedback = Arc::new(RwLock::new(
            feedback::FeedbackManager::new(config.feedback_config.clone()).await?
        ));

        let learning = Arc::new(RwLock::new(
            learning::LearningManager::new(config.learning_config.clone()).await?
        ));

        let optimization = Arc::new(RwLock::new(
            optimization::OptimizationManager::new(config.optimization_config.clone()).await?
        ));

        let layers = Arc::new(RwLock::new(
            layers::LayerManager::new(config.layer_config.clone()).await?
        ));

        Ok(Self {
            config,
            feedback,
            learning,
            optimization,
            layers,
        })
    }

    /// Initialize the PARIS framework
    pub async fn initialize(&self) -> Result<(), ParisError> {
        // Initialize all subsystems in parallel
        let (feedback_result, learning_result, optimization_result, layers_result) = tokio::join!(
            self.feedback.write().await.initialize(),
            self.learning.write().await.initialize(),
            self.optimization.write().await.initialize(),
            self.layers.write().await.initialize()
        );

        feedback_result?;
        learning_result?;
        optimization_result?;
        layers_result?;

        Ok(())
    }

    /// Get feedback manager reference
    pub fn feedback(&self) -> Arc<RwLock<feedback::FeedbackManager>> {
        Arc::clone(&self.feedback)
    }

    /// Get learning manager reference
    pub fn learning(&self) -> Arc<RwLock<learning::LearningManager>> {
        Arc::clone(&self.learning)
    }

    /// Get optimization manager reference
    pub fn optimization(&self) -> Arc<RwLock<optimization::OptimizationManager>> {
        Arc::clone(&self.optimization)
    }

    /// Get layer manager reference
    pub fn layers(&self) -> Arc<RwLock<layers::LayerManager>> {
        Arc::clone(&self.layers)
    }

    /// Process a single adaptation cycle
    pub async fn process_cycle(&self) -> Result<CycleResult, ParisError> {
        // Collect feedback
        let feedback_data = self.feedback.read().await.collect_feedback().await?;

        // Learn from feedback
        let learning_result = self.learning.write().await
            .process_feedback(&feedback_data).await?;

        // Optimize based on learning
        let optimization_result = self.optimization.write().await
            .apply_optimizations(&learning_result).await?;

        // Update layers
        self.layers.write().await
            .update_layers(&optimization_result).await?;

        Ok(CycleResult {
            feedback_count: feedback_data.len(),
            learning_updates: learning_result.update_count,
            optimizations_applied: optimization_result.count,
            layers_updated: optimization_result.affected_layers.len(),
        })
    }

    /// Shutdown the PARIS framework gracefully
    pub async fn shutdown(&self) -> Result<(), ParisError> {
        let (feedback_result, learning_result, optimization_result, layers_result) = tokio::join!(
            self.feedback.write().await.shutdown(),
            self.learning.write().await.shutdown(),
            self.optimization.write().await.shutdown(),
            self.layers.write().await.shutdown()
        );

        feedback_result?;
        learning_result?;
        optimization_result?;
        layers_result?;

        Ok(())
    }
}

/// Cycle processing result
#[derive(Debug, Clone)]
pub struct CycleResult {
    pub feedback_count: usize,
    pub learning_updates: usize,
    pub optimizations_applied: usize,
    pub layers_updated: usize,
}

/// PARIS Framework Error Types
#[derive(Debug, thiserror::Error)]
pub enum ParisError {
    #[error("Feedback system error: {0}")]
    Feedback(#[from] feedback::FeedbackError),

    #[error("Learning system error: {0}")]
    Learning(#[from] learning::LearningError),

    #[error("Optimization error: {0}")]
    Optimization(#[from] optimization::OptimizationError),

    #[error("Layer system error: {0}")]
    Layer(#[from] layers::LayerError),

    #[error("Initialization error: {0}")]
    Initialization(String),
}
