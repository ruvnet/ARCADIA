//! Experience Replay Buffer
//!
//! Implements a circular buffer for storing and sampling agent experiences
//! for reinforcement learning and training.

use super::{AgentDbError, AgentExperience};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

/// Priority for experience replay
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    /// Normal priority
    Normal,
    /// High priority (important experiences)
    High,
    /// Critical priority (rare or significant events)
    Critical,
}

/// Experience replay buffer entry
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReplayEntry {
    experience: AgentExperience,
    priority: Priority,
    sample_count: usize,
}

/// Experience Replay Buffer
///
/// Stores experiences in a circular buffer and supports:
/// - Random sampling
/// - Priority-based sampling
/// - Batch sampling for training
pub struct ExperienceReplay {
    /// Maximum buffer size
    capacity: usize,
    /// Circular buffer of experiences
    buffer: Vec<ReplayEntry>,
    /// Current write position
    position: usize,
    /// Total experiences added (may exceed capacity)
    total_added: usize,
    /// Statistics
    stats: ReplayStats,
}

impl ExperienceReplay {
    /// Create a new experience replay buffer
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            buffer: Vec::with_capacity(capacity),
            position: 0,
            total_added: 0,
            stats: ReplayStats::default(),
        }
    }

    /// Add an experience to the buffer
    pub fn add(&mut self, experience: AgentExperience) -> Result<(), AgentDbError> {
        self.add_with_priority(experience, Priority::Normal)
    }

    /// Add an experience with a specific priority
    pub fn add_with_priority(
        &mut self,
        experience: AgentExperience,
        priority: Priority,
    ) -> Result<(), AgentDbError> {
        let entry = ReplayEntry {
            experience,
            priority,
            sample_count: 0,
        };

        if self.buffer.len() < self.capacity {
            // Buffer not full yet, just append
            self.buffer.push(entry);
            self.position = self.buffer.len();
        } else {
            // Buffer full, overwrite oldest entry (circular buffer)
            self.buffer[self.position] = entry;
            self.position = (self.position + 1) % self.capacity;
        }

        self.total_added += 1;
        self.stats.total_added += 1;

        // Update priority counts
        match priority {
            Priority::Normal => self.stats.normal_priority_count += 1,
            Priority::High => self.stats.high_priority_count += 1,
            Priority::Critical => self.stats.critical_priority_count += 1,
        }

        Ok(())
    }

    /// Sample a random batch of experiences
    pub fn sample(&mut self, batch_size: usize) -> Result<Vec<AgentExperience>, AgentDbError> {
        if self.buffer.is_empty() {
            return Err(AgentDbError::ReplayError("Buffer is empty".to_string()));
        }

        let actual_batch_size = batch_size.min(self.buffer.len());
        let mut rng = thread_rng();

        let mut sampled_indices: Vec<usize> = (0..self.buffer.len()).collect();
        sampled_indices.shuffle(&mut rng);

        let experiences: Vec<AgentExperience> = sampled_indices
            .iter()
            .take(actual_batch_size)
            .map(|&idx| {
                self.buffer[idx].sample_count += 1;
                self.buffer[idx].experience.clone()
            })
            .collect();

        self.stats.total_samples += actual_batch_size;

        Ok(experiences)
    }

    /// Sample with priority bias (higher priority experiences more likely)
    pub fn sample_prioritized(&mut self, batch_size: usize) -> Result<Vec<AgentExperience>, AgentDbError> {
        if self.buffer.is_empty() {
            return Err(AgentDbError::ReplayError("Buffer is empty".to_string()));
        }

        let actual_batch_size = batch_size.min(self.buffer.len());

        // Calculate weights based on priority
        let weights: Vec<f32> = self
            .buffer
            .iter()
            .map(|entry| match entry.priority {
                Priority::Normal => 1.0,
                Priority::High => 3.0,
                Priority::Critical => 10.0,
            })
            .collect();

        // Weighted sampling
        let mut rng = thread_rng();
        let mut experiences = Vec::new();

        for _ in 0..actual_batch_size {
            let idx = weighted_sample(&weights, &mut rng);
            self.buffer[idx].sample_count += 1;
            experiences.push(self.buffer[idx].experience.clone());
        }

        self.stats.total_samples += actual_batch_size;
        self.stats.prioritized_samples += actual_batch_size;

        Ok(experiences)
    }

    /// Get most recent experiences
    pub fn get_recent(&self, count: usize) -> Vec<AgentExperience> {
        let actual_count = count.min(self.buffer.len());
        let start_idx = if self.buffer.len() < self.capacity {
            // Buffer not full yet
            self.buffer.len().saturating_sub(actual_count)
        } else {
            // Buffer full, get from circular position
            (self.position + self.capacity - actual_count) % self.capacity
        };

        let mut experiences = Vec::new();
        for i in 0..actual_count {
            let idx = (start_idx + i) % self.buffer.len();
            experiences.push(self.buffer[idx].experience.clone());
        }

        experiences
    }

    /// Get experiences by agent ID
    pub fn get_by_agent(&self, agent_id: &str) -> Vec<AgentExperience> {
        self.buffer
            .iter()
            .filter(|entry| entry.experience.agent_id == agent_id)
            .map(|entry| entry.experience.clone())
            .collect()
    }

    /// Get high-reward experiences (top percentile)
    pub fn get_high_reward(&self, percentile: f32) -> Vec<AgentExperience> {
        if self.buffer.is_empty() {
            return Vec::new();
        }

        let mut sorted: Vec<&ReplayEntry> = self.buffer.iter().collect();
        sorted.sort_by(|a, b| {
            b.experience
                .reward
                .partial_cmp(&a.experience.reward)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let cutoff = (sorted.len() as f32 * percentile).ceil() as usize;
        sorted
            .iter()
            .take(cutoff)
            .map(|entry| entry.experience.clone())
            .collect()
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.position = 0;
        self.stats = ReplayStats::default();
    }

    /// Get buffer size
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get statistics
    pub fn get_stats(&self) -> ReplayStats {
        self.stats.clone()
    }
}

/// Replay buffer statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReplayStats {
    pub total_added: usize,
    pub total_samples: usize,
    pub prioritized_samples: usize,
    pub normal_priority_count: usize,
    pub high_priority_count: usize,
    pub critical_priority_count: usize,
}

/// Weighted random sampling helper
fn weighted_sample<R: rand::Rng>(weights: &[f32], rng: &mut R) -> usize {
    let total: f32 = weights.iter().sum();
    let mut choice = rng.gen_range(0.0..total);

    for (idx, &weight) in weights.iter().enumerate() {
        choice -= weight;
        if choice <= 0.0 {
            return idx;
        }
    }

    weights.len() - 1
}

/// Episode tracker for organizing experiences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub id: String,
    pub agent_id: String,
    pub experiences: Vec<AgentExperience>,
    pub total_reward: f32,
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub completed: bool,
}

impl Episode {
    /// Create a new episode
    pub fn new(id: String, agent_id: String) -> Self {
        Self {
            id,
            agent_id,
            experiences: Vec::new(),
            total_reward: 0.0,
            start_time: chrono::Utc::now().timestamp(),
            end_time: None,
            completed: false,
        }
    }

    /// Add an experience to the episode
    pub fn add_experience(&mut self, experience: AgentExperience) {
        self.total_reward += experience.reward;
        if experience.done {
            self.completed = true;
            self.end_time = Some(chrono::Utc::now().timestamp());
        }
        self.experiences.push(experience);
    }

    /// Get episode length
    pub fn len(&self) -> usize {
        self.experiences.len()
    }

    /// Check if episode is empty
    pub fn is_empty(&self) -> bool {
        self.experiences.is_empty()
    }

    /// Get average reward
    pub fn avg_reward(&self) -> f32 {
        if self.experiences.is_empty() {
            0.0
        } else {
            self.total_reward / self.experiences.len() as f32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_experience(id: &str, reward: f32) -> AgentExperience {
        AgentExperience {
            id: id.to_string(),
            agent_id: "test_agent".to_string(),
            state_vector: vec![0.1; 10],
            action: "test".to_string(),
            reward,
            next_state_vector: vec![0.2; 10],
            done: false,
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    #[test]
    fn test_replay_buffer_creation() {
        let buffer = ExperienceReplay::new(100);
        assert_eq!(buffer.capacity(), 100);
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_add_and_sample() {
        let mut buffer = ExperienceReplay::new(10);

        for i in 0..5 {
            let exp = create_test_experience(&format!("exp_{}", i), 1.0);
            buffer.add(exp).unwrap();
        }

        assert_eq!(buffer.len(), 5);

        let samples = buffer.sample(3).unwrap();
        assert_eq!(samples.len(), 3);
    }

    #[test]
    fn test_circular_buffer() {
        let mut buffer = ExperienceReplay::new(5);

        // Add more than capacity
        for i in 0..10 {
            let exp = create_test_experience(&format!("exp_{}", i), 1.0);
            buffer.add(exp).unwrap();
        }

        // Should only keep last 5
        assert_eq!(buffer.len(), 5);
    }

    #[test]
    fn test_priority_sampling() {
        let mut buffer = ExperienceReplay::new(10);

        // Add experiences with different priorities
        for i in 0..3 {
            let exp = create_test_experience(&format!("normal_{}", i), 1.0);
            buffer.add_with_priority(exp, Priority::Normal).unwrap();
        }

        for i in 0..2 {
            let exp = create_test_experience(&format!("high_{}", i), 1.0);
            buffer.add_with_priority(exp, Priority::High).unwrap();
        }

        let exp = create_test_experience("critical", 1.0);
        buffer.add_with_priority(exp, Priority::Critical).unwrap();

        let samples = buffer.sample_prioritized(3).unwrap();
        assert_eq!(samples.len(), 3);
    }

    #[test]
    fn test_high_reward_filtering() {
        let mut buffer = ExperienceReplay::new(10);

        for i in 0..10 {
            let reward = i as f32;
            let exp = create_test_experience(&format!("exp_{}", i), reward);
            buffer.add(exp).unwrap();
        }

        let top_30_percent = buffer.get_high_reward(0.3);
        assert_eq!(top_30_percent.len(), 3);

        // Check that highest rewards are included
        assert!(top_30_percent.iter().all(|exp| exp.reward >= 7.0));
    }

    #[test]
    fn test_episode_tracking() {
        let mut episode = Episode::new("ep_1".to_string(), "agent_1".to_string());

        let exp1 = create_test_experience("exp_1", 1.0);
        episode.add_experience(exp1);

        let mut exp2 = create_test_experience("exp_2", 2.0);
        exp2.done = true;
        episode.add_experience(exp2);

        assert_eq!(episode.len(), 2);
        assert_eq!(episode.total_reward, 3.0);
        assert_eq!(episode.avg_reward(), 1.5);
        assert!(episode.completed);
    }

    #[test]
    fn test_get_recent() {
        let mut buffer = ExperienceReplay::new(10);

        for i in 0..5 {
            let exp = create_test_experience(&format!("exp_{}", i), i as f32);
            buffer.add(exp).unwrap();
        }

        let recent = buffer.get_recent(3);
        assert_eq!(recent.len(), 3);
        // Most recent should be exp_4, exp_3, exp_2
        assert_eq!(recent[2].id, "exp_4");
    }
}
