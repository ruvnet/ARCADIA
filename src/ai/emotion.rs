//! Emotion-Adaptive Experiences Module
//!
//! This module implements player emotion detection, adaptive difficulty,
//! environmental manipulation, and psychological state optimization.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use parking_lot::RwLock;
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use rand::Rng;

/// Primary emotional states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EmotionalState {
    Joy,
    Sadness,
    Anger,
    Fear,
    Surprise,
    Disgust,
    Anticipation,
    Trust,
    Neutral,
}

/// Psychological arousal level
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ArousalLevel {
    Low,
    Moderate,
    High,
    Extreme,
}

/// Environmental factors that can be manipulated
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnvironmentalFactor {
    TimeOfDay,
    Weather,
    Lighting,
    Music,
    SoundEffects,
    ColorPalette,
    CrowdDensity,
    Temperature,
}

/// Player emotional profile over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalProfile {
    pub player_id: Uuid,
    pub current_state: HashMap<EmotionalState, f64>,
    pub arousal_level: ArousalLevel,
    pub valence: f64, // Positive to negative (-1.0 to 1.0)
    pub stress_level: f64,
    pub engagement_level: f64,
    pub last_updated: DateTime<Utc>,
}

/// Emotional measurement from player behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalMeasurement {
    pub timestamp: DateTime<Utc>,
    pub detected_emotion: EmotionalState,
    pub intensity: f64,
    pub confidence: f64,
    pub source: MeasurementSource,
}

/// Source of emotional measurement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MeasurementSource {
    InputPattern,
    PerformanceMetrics,
    DecisionSpeed,
    RetryBehavior,
    ExplorationPattern,
    BiometricSensor,
}

/// Adaptation action to take based on emotion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationAction {
    pub id: Uuid,
    pub action_type: AdaptationType,
    pub target_emotion: EmotionalState,
    pub intensity: f64,
    pub duration: Duration,
    pub applied_at: DateTime<Utc>,
}

/// Types of adaptations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdaptationType {
    DifficultyAdjustment(f64),
    EnvironmentChange(EnvironmentalFactor, String),
    PacingModification(f64),
    RewardTiming(Duration),
    ChallengeType(String),
    SupportLevel(f64),
}

/// Difficulty adjustment parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultySettings {
    pub enemy_health: f64,
    pub enemy_damage: f64,
    pub enemy_count: f64,
    pub puzzle_complexity: f64,
    pub resource_availability: f64,
    pub time_pressure: f64,
}

/// Emotion-adaptive experiences system
pub struct EmotionAdaptiveExperiences {
    /// Player emotional profiles
    player_profiles: Arc<RwLock<HashMap<Uuid, EmotionalProfile>>>,

    /// Measurement history
    measurements: Arc<RwLock<VecDeque<EmotionalMeasurement>>>,

    /// Active adaptations
    active_adaptations: Arc<RwLock<Vec<AdaptationAction>>>,

    /// Difficulty settings per player
    difficulty_settings: Arc<RwLock<HashMap<Uuid, DifficultySettings>>>,

    /// Environmental state
    environment_state: Arc<RwLock<HashMap<EnvironmentalFactor, String>>>,

    /// Optimal emotional targets for engagement
    optimal_targets: Arc<RwLock<HashMap<EmotionalState, f64>>>,

    /// Adaptation sensitivity
    adaptation_sensitivity: f64,

    /// Max measurement history
    max_measurement_history: usize,
}

impl EmotionAdaptiveExperiences {
    /// Create a new emotion-adaptive system
    pub fn new() -> Self {
        let mut optimal_targets = HashMap::new();
        optimal_targets.insert(EmotionalState::Joy, 0.6);
        optimal_targets.insert(EmotionalState::Anticipation, 0.7);
        optimal_targets.insert(EmotionalState::Surprise, 0.4);
        optimal_targets.insert(EmotionalState::Fear, 0.3);
        optimal_targets.insert(EmotionalState::Anger, 0.2);

        Self {
            player_profiles: Arc::new(RwLock::new(HashMap::new())),
            measurements: Arc::new(RwLock::new(VecDeque::new())),
            active_adaptations: Arc::new(RwLock::new(Vec::new())),
            difficulty_settings: Arc::new(RwLock::new(HashMap::new())),
            environment_state: Arc::new(RwLock::new(HashMap::new())),
            optimal_targets: Arc::new(RwLock::new(optimal_targets)),
            adaptation_sensitivity: 0.5,
            max_measurement_history: 500,
        }
    }

    /// Initialize a player profile
    pub fn initialize_player(&self, player_id: Uuid) {
        let mut current_state = HashMap::new();
        current_state.insert(EmotionalState::Neutral, 1.0);

        let profile = EmotionalProfile {
            player_id,
            current_state,
            arousal_level: ArousalLevel::Moderate,
            valence: 0.0,
            stress_level: 0.0,
            engagement_level: 0.5,
            last_updated: Utc::now(),
        };

        self.player_profiles.write().insert(player_id, profile);

        // Initialize default difficulty
        let default_difficulty = DifficultySettings {
            enemy_health: 1.0,
            enemy_damage: 1.0,
            enemy_count: 1.0,
            puzzle_complexity: 1.0,
            resource_availability: 1.0,
            time_pressure: 1.0,
        };

        self.difficulty_settings.write().insert(player_id, default_difficulty);
    }

    /// Detect emotion from player behavior
    pub fn detect_emotion(&self, player_id: Uuid, source: MeasurementSource, metrics: HashMap<String, f64>) -> EmotionalState {
        let emotion = match source {
            MeasurementSource::InputPattern => {
                self.analyze_input_pattern(&metrics)
            }
            MeasurementSource::PerformanceMetrics => {
                self.analyze_performance(&metrics)
            }
            MeasurementSource::DecisionSpeed => {
                self.analyze_decision_speed(&metrics)
            }
            MeasurementSource::RetryBehavior => {
                self.analyze_retry_behavior(&metrics)
            }
            MeasurementSource::ExplorationPattern => {
                self.analyze_exploration(&metrics)
            }
            MeasurementSource::BiometricSensor => {
                self.analyze_biometrics(&metrics)
            }
        };

        let intensity = metrics.get("intensity").copied().unwrap_or(0.5);
        let confidence = metrics.get("confidence").copied().unwrap_or(0.7);

        self.record_measurement(player_id, emotion, intensity, confidence, source);
        emotion
    }

    /// Record an emotional measurement
    fn record_measurement(&self, player_id: Uuid, emotion: EmotionalState, intensity: f64, confidence: f64, source: MeasurementSource) {
        let measurement = EmotionalMeasurement {
            timestamp: Utc::now(),
            detected_emotion: emotion,
            intensity: intensity.clamp(0.0, 1.0),
            confidence: confidence.clamp(0.0, 1.0),
            source,
        };

        let mut measurements = self.measurements.write();
        measurements.push_back(measurement.clone());

        while measurements.len() > self.max_measurement_history {
            measurements.pop_front();
        }

        drop(measurements);

        // Update player profile
        self.update_player_profile(player_id, emotion, intensity, confidence);
    }

    /// Update player emotional profile
    fn update_player_profile(&self, player_id: Uuid, emotion: EmotionalState, intensity: f64, confidence: f64) {
        let mut profiles = self.player_profiles.write();

        if let Some(profile) = profiles.get_mut(&player_id) {
            // Update emotional state with weighted average
            let current = profile.current_state.get(&emotion).copied().unwrap_or(0.0);
            let new_value = (current * 0.7) + (intensity * confidence * 0.3);
            profile.current_state.insert(emotion, new_value.clamp(0.0, 1.0));

            // Decay other emotions
            for (state, value) in profile.current_state.iter_mut() {
                if *state != emotion {
                    *value *= 0.9;
                }
            }

            // Update arousal level
            profile.arousal_level = self.calculate_arousal_level(&profile.current_state);

            // Update valence
            profile.valence = self.calculate_valence(&profile.current_state);

            // Update stress
            profile.stress_level = self.calculate_stress_level(&profile.current_state);

            // Update engagement
            profile.engagement_level = self.calculate_engagement(&profile.current_state);

            profile.last_updated = Utc::now();
        }

        drop(profiles);

        // Trigger adaptation if needed
        self.evaluate_adaptation_need(player_id);
    }

    /// Analyze input patterns for emotion
    fn analyze_input_pattern(&self, metrics: &HashMap<String, f64>) -> EmotionalState {
        let rapid_inputs = metrics.get("rapid_inputs").copied().unwrap_or(0.0);
        let erratic_movement = metrics.get("erratic_movement").copied().unwrap_or(0.0);

        if rapid_inputs > 0.7 && erratic_movement > 0.6 {
            EmotionalState::Anger
        } else if erratic_movement > 0.7 {
            EmotionalState::Fear
        } else {
            EmotionalState::Neutral
        }
    }

    /// Analyze performance metrics for emotion
    fn analyze_performance(&self, metrics: &HashMap<String, f64>) -> EmotionalState {
        let success_rate = metrics.get("success_rate").copied().unwrap_or(0.5);
        let improvement = metrics.get("improvement").copied().unwrap_or(0.0);

        if success_rate > 0.8 && improvement > 0.1 {
            EmotionalState::Joy
        } else if success_rate < 0.3 {
            EmotionalState::Sadness
        } else {
            EmotionalState::Neutral
        }
    }

    /// Analyze decision speed for emotion
    fn analyze_decision_speed(&self, metrics: &HashMap<String, f64>) -> EmotionalState {
        let avg_decision_time = metrics.get("avg_decision_time").copied().unwrap_or(1.0);

        if avg_decision_time < 0.3 {
            EmotionalState::Anticipation
        } else if avg_decision_time > 5.0 {
            EmotionalState::Fear
        } else {
            EmotionalState::Neutral
        }
    }

    /// Analyze retry behavior for emotion
    fn analyze_retry_behavior(&self, metrics: &HashMap<String, f64>) -> EmotionalState {
        let retry_count = metrics.get("retry_count").copied().unwrap_or(0.0);
        let quit_attempts = metrics.get("quit_attempts").copied().unwrap_or(0.0);

        if retry_count > 5.0 && quit_attempts > 0.0 {
            EmotionalState::Anger
        } else if retry_count > 3.0 {
            EmotionalState::Sadness
        } else {
            EmotionalState::Neutral
        }
    }

    /// Analyze exploration patterns for emotion
    fn analyze_exploration(&self, metrics: &HashMap<String, f64>) -> EmotionalState {
        let exploration_rate = metrics.get("exploration_rate").copied().unwrap_or(0.5);

        if exploration_rate > 0.7 {
            EmotionalState::Anticipation
        } else if exploration_rate < 0.2 {
            EmotionalState::Fear
        } else {
            EmotionalState::Neutral
        }
    }

    /// Analyze biometric data for emotion
    fn analyze_biometrics(&self, metrics: &HashMap<String, f64>) -> EmotionalState {
        let heart_rate = metrics.get("heart_rate").copied().unwrap_or(70.0);
        let skin_conductance = metrics.get("skin_conductance").copied().unwrap_or(0.5);

        if heart_rate > 100.0 && skin_conductance > 0.7 {
            EmotionalState::Fear
        } else if heart_rate > 90.0 {
            EmotionalState::Anticipation
        } else {
            EmotionalState::Neutral
        }
    }

    /// Calculate arousal level from emotional state
    fn calculate_arousal_level(&self, state: &HashMap<EmotionalState, f64>) -> ArousalLevel {
        let high_arousal_emotions = [
            EmotionalState::Anger,
            EmotionalState::Fear,
            EmotionalState::Surprise,
        ];

        let total_arousal: f64 = high_arousal_emotions.iter()
            .map(|e| state.get(e).copied().unwrap_or(0.0))
            .sum();

        if total_arousal > 0.8 {
            ArousalLevel::Extreme
        } else if total_arousal > 0.5 {
            ArousalLevel::High
        } else if total_arousal > 0.2 {
            ArousalLevel::Moderate
        } else {
            ArousalLevel::Low
        }
    }

    /// Calculate emotional valence (positive/negative)
    fn calculate_valence(&self, state: &HashMap<EmotionalState, f64>) -> f64 {
        let positive = state.get(&EmotionalState::Joy).copied().unwrap_or(0.0)
            + state.get(&EmotionalState::Trust).copied().unwrap_or(0.0)
            + state.get(&EmotionalState::Anticipation).copied().unwrap_or(0.0);

        let negative = state.get(&EmotionalState::Sadness).copied().unwrap_or(0.0)
            + state.get(&EmotionalState::Anger).copied().unwrap_or(0.0)
            + state.get(&EmotionalState::Fear).copied().unwrap_or(0.0);

        (positive - negative).clamp(-1.0, 1.0)
    }

    /// Calculate stress level
    fn calculate_stress_level(&self, state: &HashMap<EmotionalState, f64>) -> f64 {
        let stress_emotions = [
            EmotionalState::Anger,
            EmotionalState::Fear,
        ];

        let total: f64 = stress_emotions.iter()
            .map(|e| state.get(e).copied().unwrap_or(0.0))
            .sum();

        (total / stress_emotions.len() as f64).clamp(0.0, 1.0)
    }

    /// Calculate engagement level
    fn calculate_engagement(&self, state: &HashMap<EmotionalState, f64>) -> f64 {
        let engaged_emotions = [
            EmotionalState::Joy,
            EmotionalState::Anticipation,
            EmotionalState::Surprise,
        ];

        let total: f64 = engaged_emotions.iter()
            .map(|e| state.get(e).copied().unwrap_or(0.0))
            .sum();

        (total / engaged_emotions.len() as f64).clamp(0.0, 1.0)
    }

    /// Evaluate if adaptation is needed
    fn evaluate_adaptation_need(&self, player_id: Uuid) {
        let profiles = self.player_profiles.read();
        let profile = match profiles.get(&player_id) {
            Some(p) => p,
            None => return,
        };

        let targets = self.optimal_targets.read();

        // Check if emotional state deviates from optimal
        for (emotion, target) in targets.iter() {
            let current = profile.current_state.get(emotion).copied().unwrap_or(0.0);
            let deviation = (current - target).abs();

            if deviation > self.adaptation_sensitivity {
                drop(profiles);
                drop(targets);
                self.apply_adaptation(player_id, *emotion, deviation);
                return;
            }
        }

        // Check stress level
        if profile.stress_level > 0.7 {
            drop(profiles);
            drop(targets);
            self.reduce_stress(player_id);
        } else if profile.engagement_level < 0.3 {
            drop(profiles);
            drop(targets);
            self.increase_engagement(player_id);
        }
    }

    /// Apply emotional adaptation
    fn apply_adaptation(&self, player_id: Uuid, target_emotion: EmotionalState, intensity: f64) {
        let action = match target_emotion {
            EmotionalState::Joy => self.create_joy_adaptation(player_id, intensity),
            EmotionalState::Fear => self.create_fear_reduction(player_id, intensity),
            EmotionalState::Anger => self.create_calm_adaptation(player_id, intensity),
            EmotionalState::Anticipation => self.create_anticipation_boost(player_id, intensity),
            _ => return,
        };

        self.active_adaptations.write().push(action);
    }

    /// Create adaptation to increase joy
    fn create_joy_adaptation(&self, player_id: Uuid, intensity: f64) -> AdaptationAction {
        // Provide positive rewards or easier challenges
        AdaptationAction {
            id: Uuid::new_v4(),
            action_type: AdaptationType::DifficultyAdjustment(-0.1 * intensity),
            target_emotion: EmotionalState::Joy,
            intensity,
            duration: Duration::minutes(5),
            applied_at: Utc::now(),
        }
    }

    /// Create adaptation to reduce fear
    fn create_fear_reduction(&self, player_id: Uuid, intensity: f64) -> AdaptationAction {
        AdaptationAction {
            id: Uuid::new_v4(),
            action_type: AdaptationType::EnvironmentChange(
                EnvironmentalFactor::Lighting,
                "brighter".to_string(),
            ),
            target_emotion: EmotionalState::Fear,
            intensity,
            duration: Duration::minutes(10),
            applied_at: Utc::now(),
        }
    }

    /// Create adaptation to reduce anger
    fn create_calm_adaptation(&self, player_id: Uuid, intensity: f64) -> AdaptationAction {
        AdaptationAction {
            id: Uuid::new_v4(),
            action_type: AdaptationType::PacingModification(-0.2 * intensity),
            target_emotion: EmotionalState::Anger,
            intensity,
            duration: Duration::minutes(3),
            applied_at: Utc::now(),
        }
    }

    /// Create adaptation to boost anticipation
    fn create_anticipation_boost(&self, player_id: Uuid, intensity: f64) -> AdaptationAction {
        AdaptationAction {
            id: Uuid::new_v4(),
            action_type: AdaptationType::ChallengeType("exploration".to_string()),
            target_emotion: EmotionalState::Anticipation,
            intensity,
            duration: Duration::minutes(15),
            applied_at: Utc::now(),
        }
    }

    /// Reduce player stress
    fn reduce_stress(&self, player_id: Uuid) {
        if let Some(settings) = self.difficulty_settings.write().get_mut(&player_id) {
            settings.enemy_damage *= 0.8;
            settings.time_pressure *= 0.7;
            settings.resource_availability *= 1.2;
        }
    }

    /// Increase player engagement
    fn increase_engagement(&self, player_id: Uuid) {
        if let Some(settings) = self.difficulty_settings.write().get_mut(&player_id) {
            settings.puzzle_complexity *= 1.1;
            settings.enemy_count *= 1.05;
        }
    }

    /// Adjust difficulty dynamically
    pub fn adjust_difficulty(&self, player_id: Uuid, adjustment: f64) {
        if let Some(settings) = self.difficulty_settings.write().get_mut(&player_id) {
            let factor = 1.0 + adjustment;
            settings.enemy_health *= factor;
            settings.enemy_damage *= factor;
            settings.puzzle_complexity *= factor;
        }
    }

    /// Manipulate environment
    pub fn manipulate_environment(&self, factor: EnvironmentalFactor, value: String) {
        self.environment_state.write().insert(factor, value);
    }

    /// Get player profile
    pub fn get_player_profile(&self, player_id: Uuid) -> Option<EmotionalProfile> {
        self.player_profiles.read().get(&player_id).cloned()
    }

    /// Get difficulty settings
    pub fn get_difficulty_settings(&self, player_id: Uuid) -> Option<DifficultySettings> {
        self.difficulty_settings.read().get(&player_id).cloned()
    }

    /// Get active adaptations
    pub fn get_active_adaptations(&self) -> Vec<AdaptationAction> {
        self.active_adaptations.read().clone()
    }
}

impl Default for EmotionAdaptiveExperiences {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotion_system_creation() {
        let emotion_system = EmotionAdaptiveExperiences::new();
        assert!(emotion_system.get_active_adaptations().is_empty());
    }

    #[test]
    fn test_player_initialization() {
        let system = EmotionAdaptiveExperiences::new();
        let player_id = Uuid::new_v4();

        system.initialize_player(player_id);
        let profile = system.get_player_profile(player_id);

        assert!(profile.is_some());
        assert_eq!(profile.unwrap().arousal_level, ArousalLevel::Moderate);
    }

    #[test]
    fn test_emotion_detection() {
        let system = EmotionAdaptiveExperiences::new();
        let player_id = Uuid::new_v4();

        system.initialize_player(player_id);

        let mut metrics = HashMap::new();
        metrics.insert("success_rate".to_string(), 0.9);
        metrics.insert("improvement".to_string(), 0.2);

        let emotion = system.detect_emotion(
            player_id,
            MeasurementSource::PerformanceMetrics,
            metrics,
        );

        assert_eq!(emotion, EmotionalState::Joy);
    }

    #[test]
    fn test_difficulty_adjustment() {
        let system = EmotionAdaptiveExperiences::new();
        let player_id = Uuid::new_v4();

        system.initialize_player(player_id);

        let initial = system.get_difficulty_settings(player_id).unwrap();
        system.adjust_difficulty(player_id, 0.2);
        let adjusted = system.get_difficulty_settings(player_id).unwrap();

        assert!(adjusted.enemy_health > initial.enemy_health);
    }
}
