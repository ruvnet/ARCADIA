//! Integration tests for AgentDB module
//!
//! Tests the complete AgentDB system including:
//! - Memory persistence
//! - Learning database
//! - Experience replay
//! - Cross-module integration

use arcadia::agentdb::{
    AgentDbConfig, AgentDbManager, AgentExperience,
    experience_replay::{ExperienceReplay, Priority},
    learning_database::{LearningDatabase, PatternType},
    memory_persistence::MemoryPersistence,
};
use std::collections::HashMap;

#[tokio::test]
async fn test_complete_agentdb_workflow() {
    // Create configuration
    let config = AgentDbConfig {
        db_name: "test_workflow".to_string(),
        vector_dim: 128,
        wasm_enabled: false,
        max_memory_mb: 100,
        replay_buffer_size: 1000,
        auto_save_interval: 300,
        enable_compression: true,
    };

    // Initialize manager
    let mut manager = AgentDbManager::new(config).await.unwrap();
    manager.initialize().await.unwrap();

    // Add multiple experiences
    for i in 0..50 {
        let experience = create_test_experience(
            &format!("exp_{}", i),
            "agent_1",
            if i % 2 == 0 { 1.0 } else { -0.5 },
        );

        manager.store_experience("agent_1", experience).await.unwrap();
    }

    // Get statistics
    let stats = manager.get_stats();
    assert_eq!(stats.total_experiences, 50);

    // Query similar experiences
    let query_vector = vec![0.5; 128];
    let results = manager.query_similar_experiences(query_vector, 5).await.unwrap();
    assert!(results.len() <= 5);

    // Sample from replay buffer
    let samples = manager.sample_replay_batch(10).unwrap();
    assert_eq!(samples.len(), 10);

    // Get agent memory
    let memories = manager.get_agent_memory("agent_1").await.unwrap();
    assert_eq!(memories.len(), 50);

    // Save and shutdown
    manager.save().await.unwrap();
    manager.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_memory_persistence() {
    let config = AgentDbConfig {
        wasm_enabled: false,
        ..Default::default()
    };

    let mut persistence = MemoryPersistence::new(&config).await.unwrap();
    persistence.initialize().await.unwrap();

    // Store experiences
    for i in 0..20 {
        let exp = create_test_experience(&format!("exp_{}", i), "agent_1", i as f32);
        persistence.store("agent_1", &exp).await.unwrap();
    }

    // Retrieve all
    let retrieved = persistence.retrieve("agent_1").await.unwrap();
    assert_eq!(retrieved.len(), 20);

    // Retrieve recent
    let recent = persistence.retrieve_recent("agent_1", 5).await.unwrap();
    assert_eq!(recent.len(), 5);

    // Check memory usage
    let usage = persistence.get_memory_usage_mb();
    assert!(usage > 0.0);

    // Get stats
    let stats = persistence.get_stats();
    assert_eq!(stats.total_agents, 1);
    assert_eq!(stats.total_experiences, 20);

    persistence.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_learning_database() {
    let config = AgentDbConfig {
        vector_dim: 128,
        ..Default::default()
    };

    let mut db = LearningDatabase::new(&config).await.unwrap();
    db.initialize().await.unwrap();

    // Add positive reward experiences
    for i in 0..100 {
        let exp = create_test_experience(
            &format!("success_{}", i),
            "agent_1",
            1.5,
        );
        db.add_experience(&exp).await.unwrap();
    }

    // Add negative reward experiences
    for i in 0..50 {
        let exp = create_test_experience(
            &format!("failure_{}", i),
            "agent_1",
            -1.0,
        );
        db.add_experience(&exp).await.unwrap();
    }

    // Query similar
    let query = vec![0.5; 128];
    let results = db.query_similar(query, 10).await.unwrap();
    assert_eq!(results.len(), 10);

    // Get successful experiences
    let successful = db.get_successful_experiences().await.unwrap();
    assert_eq!(successful.len(), 100);

    // Get failed experiences
    let failed = db.get_failed_experiences().await.unwrap();
    assert_eq!(failed.len(), 50);

    // Check patterns
    let patterns = db.get_patterns().await;
    assert!(!patterns.is_empty(), "Should have detected patterns");

    let stats = db.get_stats();
    assert_eq!(stats.total_experiences, 150);

    db.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_experience_replay_buffer() {
    let mut replay = ExperienceReplay::new(100);

    // Add normal priority experiences
    for i in 0..50 {
        let exp = create_test_experience(&format!("exp_{}", i), "agent_1", i as f32);
        replay.add(exp).unwrap();
    }

    assert_eq!(replay.len(), 50);

    // Add high priority experiences
    for i in 50..60 {
        let exp = create_test_experience(&format!("high_{}", i), "agent_1", 10.0);
        replay.add_with_priority(exp, Priority::High).unwrap();
    }

    // Add critical priority experience
    let critical = create_test_experience("critical", "agent_1", 100.0);
    replay.add_with_priority(critical, Priority::Critical).unwrap();

    assert_eq!(replay.len(), 61);

    // Random sampling
    let samples = replay.sample(10).unwrap();
    assert_eq!(samples.len(), 10);

    // Prioritized sampling should favor high/critical
    let prioritized = replay.sample_prioritized(20).unwrap();
    assert_eq!(prioritized.len(), 20);

    // Get recent
    let recent = replay.get_recent(5);
    assert_eq!(recent.len(), 5);

    // Get by agent
    let agent_exps = replay.get_by_agent("agent_1");
    assert_eq!(agent_exps.len(), 61);

    // Get high reward
    let high_reward = replay.get_high_reward(0.2);
    assert!(!high_reward.is_empty());

    // Check stats
    let stats = replay.get_stats();
    assert_eq!(stats.total_added, 61);
    assert_eq!(stats.normal_priority_count, 50);
    assert_eq!(stats.high_priority_count, 10);
    assert_eq!(stats.critical_priority_count, 1);
}

#[tokio::test]
async fn test_circular_buffer_behavior() {
    let mut replay = ExperienceReplay::new(10);

    // Fill buffer
    for i in 0..10 {
        let exp = create_test_experience(&format!("exp_{}", i), "agent_1", i as f32);
        replay.add(exp).unwrap();
    }

    assert_eq!(replay.len(), 10);

    // Add more to test circular behavior
    for i in 10..20 {
        let exp = create_test_experience(&format!("exp_{}", i), "agent_1", i as f32);
        replay.add(exp).unwrap();
    }

    // Should still be 10 (capacity)
    assert_eq!(replay.len(), 10);

    // Recent experiences should be from 10-19
    let recent = replay.get_recent(5);
    assert_eq!(recent.len(), 5);
}

#[tokio::test]
async fn test_vector_similarity_search() {
    let config = AgentDbConfig {
        vector_dim: 10,
        ..Default::default()
    };

    let mut db = LearningDatabase::new(&config).await.unwrap();
    db.initialize().await.unwrap();

    // Create experiences with known vectors
    let exp1 = AgentExperience {
        id: "exp1".to_string(),
        agent_id: "agent_1".to_string(),
        state_vector: vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        action: "test".to_string(),
        reward: 1.0,
        next_state_vector: vec![0.0; 10],
        done: false,
        metadata: HashMap::new(),
        timestamp: chrono::Utc::now().timestamp(),
    };

    let exp2 = AgentExperience {
        id: "exp2".to_string(),
        agent_id: "agent_1".to_string(),
        state_vector: vec![0.9, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        action: "test".to_string(),
        reward: 1.0,
        next_state_vector: vec![0.0; 10],
        done: false,
        metadata: HashMap::new(),
        timestamp: chrono::Utc::now().timestamp(),
    };

    let exp3 = AgentExperience {
        id: "exp3".to_string(),
        agent_id: "agent_1".to_string(),
        state_vector: vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        action: "test".to_string(),
        reward: 1.0,
        next_state_vector: vec![0.0; 10],
        done: false,
        metadata: HashMap::new(),
        timestamp: chrono::Utc::now().timestamp(),
    };

    db.add_experience(&exp1).await.unwrap();
    db.add_experience(&exp2).await.unwrap();
    db.add_experience(&exp3).await.unwrap();

    // Query with vector similar to exp1
    let query = vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let results = db.query_similar(query, 2).await.unwrap();

    assert_eq!(results.len(), 2);
    // First result should be exp1 (exact match)
    assert_eq!(results[0].id, "exp1");
    // Second should be exp2 (similar)
    assert_eq!(results[1].id, "exp2");
}

#[tokio::test]
async fn test_multi_agent_storage() {
    let config = AgentDbConfig::default();
    let mut manager = AgentDbManager::new(config).await.unwrap();
    manager.initialize().await.unwrap();

    // Add experiences for multiple agents
    for agent_num in 1..=3 {
        let agent_id = format!("agent_{}", agent_num);
        for i in 0..10 {
            let exp = create_test_experience(
                &format!("{}exp{}", agent_id, i),
                &agent_id,
                i as f32,
            );
            manager.store_experience(&agent_id, exp).await.unwrap();
        }
    }

    // Verify each agent has their experiences
    for agent_num in 1..=3 {
        let agent_id = format!("agent_{}", agent_num);
        let memories = manager.get_agent_memory(&agent_id).await.unwrap();
        assert_eq!(memories.len(), 10);
    }

    let stats = manager.get_stats();
    assert_eq!(stats.total_experiences, 30);
}

// Helper function to create test experiences
fn create_test_experience(id: &str, agent_id: &str, reward: f32) -> AgentExperience {
    AgentExperience {
        id: id.to_string(),
        agent_id: agent_id.to_string(),
        state_vector: vec![0.5; 128],
        action: "test_action".to_string(),
        reward,
        next_state_vector: vec![0.6; 128],
        done: false,
        metadata: HashMap::new(),
        timestamp: chrono::Utc::now().timestamp(),
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[tokio::test]
    async fn test_large_scale_storage() {
        let config = AgentDbConfig {
            vector_dim: 128,
            max_memory_mb: 100,
            ..Default::default()
        };

        let mut manager = AgentDbManager::new(config).await.unwrap();
        manager.initialize().await.unwrap();

        // Store 1000 experiences
        let start = std::time::Instant::now();
        for i in 0..1000 {
            let exp = create_test_experience(&format!("exp_{}", i), "agent_1", i as f32);
            manager.store_experience("agent_1", exp).await.unwrap();
        }
        let duration = start.elapsed();

        println!("Stored 1000 experiences in {:?}", duration);
        assert!(duration.as_secs() < 5, "Should store 1000 experiences in under 5 seconds");

        let stats = manager.get_stats();
        assert!(stats.total_experiences >= 1000);
    }

    #[tokio::test]
    async fn test_query_performance() {
        let config = AgentDbConfig {
            vector_dim: 128,
            ..Default::default()
        };

        let mut db = LearningDatabase::new(&config).await.unwrap();
        db.initialize().await.unwrap();

        // Add 500 experiences
        for i in 0..500 {
            let exp = create_test_experience(&format!("exp_{}", i), "agent_1", i as f32);
            db.add_experience(&exp).await.unwrap();
        }

        // Measure query performance
        let query = vec![0.5; 128];
        let start = std::time::Instant::now();
        let _results = db.query_similar(query, 10).await.unwrap();
        let duration = start.elapsed();

        println!("Query on 500 experiences took {:?}", duration);
        assert!(duration.as_millis() < 100, "Query should take less than 100ms");
    }
}
