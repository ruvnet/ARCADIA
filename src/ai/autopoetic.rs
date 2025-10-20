//! Autopoetic Processing Module
//!
//! This module implements self-organization, self-maintenance, and dynamic
//! adaptation systems that allow game elements to evolve and maintain themselves.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rand::Rng;

/// Health status of a system component
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum HealthStatus {
    Optimal,
    Healthy,
    Degraded,
    Critical,
    Failed,
}

/// Type of system maintenance required
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaintenanceType {
    Repair,
    Optimize,
    Reorganize,
    Adapt,
    Regenerate,
}

/// Self-organizing pattern detected in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentPattern {
    pub id: Uuid,
    pub name: String,
    pub components: Vec<String>,
    pub strength: f64,
    pub stability: f64,
    pub discovered_at: DateTime<Utc>,
}

/// Represents a component within the autopoetic system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemComponent {
    pub id: Uuid,
    pub name: String,
    pub health: f64,
    pub efficiency: f64,
    pub dependencies: Vec<Uuid>,
    pub last_maintenance: DateTime<Utc>,
    pub active: bool,
}

/// Maintenance action for system components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceAction {
    pub id: Uuid,
    pub component_id: Uuid,
    pub maintenance_type: MaintenanceType,
    pub scheduled_time: DateTime<Utc>,
    pub priority: f64,
    pub estimated_duration: f64,
}

/// Adaptation rule for dynamic system changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationRule {
    pub id: Uuid,
    pub trigger_condition: String,
    pub adaptation_response: String,
    pub strength: f64,
    pub activation_count: usize,
}

/// Autopoetic processing system for self-organization and maintenance
pub struct AutopoeticProcessing {
    /// All system components
    components: Arc<RwLock<HashMap<Uuid, SystemComponent>>>,

    /// Maintenance queue
    maintenance_queue: Arc<RwLock<Vec<MaintenanceAction>>>,

    /// Emergent patterns discovered
    emergent_patterns: Arc<RwLock<Vec<EmergentPattern>>>,

    /// Adaptation rules
    adaptation_rules: Arc<RwLock<HashMap<String, AdaptationRule>>>,

    /// System-wide health metrics
    system_health: Arc<RwLock<f64>>,

    /// Self-organization threshold
    organization_threshold: f64,

    /// Maintenance interval in hours
    maintenance_interval: f64,
}

impl AutopoeticProcessing {
    /// Create a new autopoetic processing system
    pub fn new() -> Self {
        Self {
            components: Arc::new(RwLock::new(HashMap::new())),
            maintenance_queue: Arc::new(RwLock::new(Vec::new())),
            emergent_patterns: Arc::new(RwLock::new(Vec::new())),
            adaptation_rules: Arc::new(RwLock::new(HashMap::new())),
            system_health: Arc::new(RwLock::new(1.0)),
            organization_threshold: 0.7,
            maintenance_interval: 24.0,
        }
    }

    /// Add a component to the system
    pub fn add_component(&self, name: String, dependencies: Vec<Uuid>) -> Uuid {
        let component = SystemComponent {
            id: Uuid::new_v4(),
            name,
            health: 1.0,
            efficiency: 1.0,
            dependencies,
            last_maintenance: Utc::now(),
            active: true,
        };

        let id = component.id;
        self.components.write().insert(id, component);
        self.update_system_health();
        id
    }

    /// Remove a component from the system
    pub fn remove_component(&self, id: Uuid) -> Option<SystemComponent> {
        let component = self.components.write().remove(&id);
        if component.is_some() {
            self.update_system_health();
        }
        component
    }

    /// Get component by ID
    pub fn get_component(&self, id: Uuid) -> Option<SystemComponent> {
        self.components.read().get(&id).cloned()
    }

    /// Update component health
    pub fn update_component_health(&self, id: Uuid, health: f64) -> bool {
        if let Some(component) = self.components.write().get_mut(&id) {
            component.health = health.clamp(0.0, 1.0);
            self.update_system_health();

            // Schedule maintenance if health is low
            if health < 0.5 {
                self.schedule_maintenance(id, MaintenanceType::Repair, 0.8);
            }
            true
        } else {
            false
        }
    }

    /// Perform self-maintenance on all components
    pub fn perform_self_maintenance(&self) {
        let now = Utc::now();
        let mut components = self.components.write();
        let mut actions_needed = Vec::new();

        for (id, component) in components.iter() {
            // Check if maintenance is needed
            let hours_since_maintenance = (now - component.last_maintenance)
                .num_hours() as f64;

            if hours_since_maintenance > self.maintenance_interval {
                actions_needed.push((*id, MaintenanceType::Optimize));
            }

            // Health-based maintenance
            if component.health < 0.7 {
                actions_needed.push((*id, MaintenanceType::Repair));
            }

            // Efficiency-based optimization
            if component.efficiency < 0.6 {
                actions_needed.push((*id, MaintenanceType::Optimize));
            }
        }

        drop(components);

        // Schedule maintenance actions
        for (id, mtype) in actions_needed {
            self.schedule_maintenance(id, mtype, 0.7);
        }

        // Process maintenance queue
        self.process_maintenance_queue();
    }

    /// Schedule maintenance for a component
    pub fn schedule_maintenance(&self, component_id: Uuid, maintenance_type: MaintenanceType, priority: f64) {
        let action = MaintenanceAction {
            id: Uuid::new_v4(),
            component_id,
            maintenance_type: maintenance_type.clone(),
            scheduled_time: Utc::now(),
            priority,
            estimated_duration: self.estimate_maintenance_duration(&maintenance_type),
        };

        let mut queue = self.maintenance_queue.write();
        queue.push(action);

        // Sort by priority (descending)
        queue.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());
    }

    /// Process the maintenance queue
    fn process_maintenance_queue(&self) {
        let mut queue = self.maintenance_queue.write();
        let mut components = self.components.write();

        // Process high-priority items
        let to_process: Vec<_> = queue.drain(..).take(10).collect();

        for action in to_process {
            if let Some(component) = components.get_mut(&action.component_id) {
                match action.maintenance_type {
                    MaintenanceType::Repair => {
                        component.health = (component.health + 0.3).min(1.0);
                    }
                    MaintenanceType::Optimize => {
                        component.efficiency = (component.efficiency + 0.2).min(1.0);
                    }
                    MaintenanceType::Reorganize => {
                        // Reorganize can improve both health and efficiency
                        component.health = (component.health + 0.1).min(1.0);
                        component.efficiency = (component.efficiency + 0.15).min(1.0);
                    }
                    MaintenanceType::Adapt => {
                        // Adaptation improves efficiency
                        component.efficiency = (component.efficiency + 0.25).min(1.0);
                    }
                    MaintenanceType::Regenerate => {
                        // Full regeneration
                        component.health = 1.0;
                        component.efficiency = 1.0;
                    }
                }
                component.last_maintenance = Utc::now();
            }
        }

        drop(components);
        self.update_system_health();
    }

    /// Estimate maintenance duration
    fn estimate_maintenance_duration(&self, maintenance_type: &MaintenanceType) -> f64 {
        match maintenance_type {
            MaintenanceType::Repair => 2.0,
            MaintenanceType::Optimize => 1.5,
            MaintenanceType::Reorganize => 3.0,
            MaintenanceType::Adapt => 2.5,
            MaintenanceType::Regenerate => 5.0,
        }
    }

    /// Detect emergent patterns in the system
    pub fn detect_emergent_patterns(&self) -> Vec<EmergentPattern> {
        let components = self.components.read();
        let mut patterns = Vec::new();

        // Look for highly connected component groups
        let component_graph = self.build_component_graph(&components);

        // Detect clusters
        let clusters = self.find_clusters(&component_graph);

        for (i, cluster) in clusters.iter().enumerate() {
            if cluster.len() >= 3 {
                let strength = self.calculate_cluster_strength(cluster, &components);
                let stability = self.calculate_cluster_stability(cluster, &components);

                if strength > self.organization_threshold {
                    let pattern = EmergentPattern {
                        id: Uuid::new_v4(),
                        name: format!("Pattern-{}", i + 1),
                        components: cluster.iter()
                            .filter_map(|id| components.get(id).map(|c| c.name.clone()))
                            .collect(),
                        strength,
                        stability,
                        discovered_at: Utc::now(),
                    };
                    patterns.push(pattern);
                }
            }
        }

        // Store discovered patterns
        let mut emergent = self.emergent_patterns.write();
        emergent.extend(patterns.clone());

        patterns
    }

    /// Build a component dependency graph
    fn build_component_graph(&self, components: &HashMap<Uuid, SystemComponent>) -> HashMap<Uuid, Vec<Uuid>> {
        let mut graph = HashMap::new();

        for (id, component) in components.iter() {
            graph.insert(*id, component.dependencies.clone());
        }

        graph
    }

    /// Find clusters of related components
    fn find_clusters(&self, graph: &HashMap<Uuid, Vec<Uuid>>) -> Vec<Vec<Uuid>> {
        let mut clusters = Vec::new();
        let mut visited = std::collections::HashSet::new();

        for node in graph.keys() {
            if !visited.contains(node) {
                let cluster = self.dfs_cluster(*node, graph, &mut visited);
                if !cluster.is_empty() {
                    clusters.push(cluster);
                }
            }
        }

        clusters
    }

    /// Depth-first search to find a cluster
    fn dfs_cluster(
        &self,
        node: Uuid,
        graph: &HashMap<Uuid, Vec<Uuid>>,
        visited: &mut std::collections::HashSet<Uuid>,
    ) -> Vec<Uuid> {
        let mut cluster = Vec::new();
        let mut stack = vec![node];

        while let Some(current) = stack.pop() {
            if visited.insert(current) {
                cluster.push(current);

                if let Some(neighbors) = graph.get(&current) {
                    for neighbor in neighbors {
                        if !visited.contains(neighbor) {
                            stack.push(*neighbor);
                        }
                    }
                }
            }
        }

        cluster
    }

    /// Calculate cluster strength based on component health
    fn calculate_cluster_strength(&self, cluster: &[Uuid], components: &HashMap<Uuid, SystemComponent>) -> f64 {
        let total_health: f64 = cluster.iter()
            .filter_map(|id| components.get(id))
            .map(|c| c.health * c.efficiency)
            .sum();

        total_health / cluster.len() as f64
    }

    /// Calculate cluster stability
    fn calculate_cluster_stability(&self, cluster: &[Uuid], components: &HashMap<Uuid, SystemComponent>) -> f64 {
        let active_count = cluster.iter()
            .filter_map(|id| components.get(id))
            .filter(|c| c.active && c.health > 0.5)
            .count();

        active_count as f64 / cluster.len() as f64
    }

    /// Self-organize the system to improve efficiency
    pub fn self_organize(&self) {
        // Detect patterns
        let patterns = self.detect_emergent_patterns();

        // Reorganize based on patterns
        for pattern in &patterns {
            if pattern.strength > 0.8 && pattern.stability < 0.6 {
                // Strong but unstable - needs reorganization
                for component_name in &pattern.components {
                    if let Some((id, _)) = self.find_component_by_name(component_name) {
                        self.schedule_maintenance(id, MaintenanceType::Reorganize, 0.75);
                    }
                }
            }
        }

        // Optimize weak components
        let components = self.components.read();
        for (id, component) in components.iter() {
            if component.efficiency < 0.5 {
                drop(components);
                self.schedule_maintenance(*id, MaintenanceType::Optimize, 0.7);
                let components = self.components.read();
            }
        }
    }

    /// Find component by name
    fn find_component_by_name(&self, name: &str) -> Option<(Uuid, SystemComponent)> {
        self.components.read()
            .iter()
            .find(|(_, c)| c.name == name)
            .map(|(id, c)| (*id, c.clone()))
    }

    /// Add an adaptation rule
    pub fn add_adaptation_rule(&self, trigger: String, response: String, strength: f64) -> Uuid {
        let rule = AdaptationRule {
            id: Uuid::new_v4(),
            trigger_condition: trigger.clone(),
            adaptation_response: response,
            strength,
            activation_count: 0,
        };

        let id = rule.id;
        self.adaptation_rules.write().insert(trigger, rule);
        id
    }

    /// Apply adaptation rules based on current conditions
    pub fn apply_adaptations(&self, conditions: &HashMap<String, f64>) {
        let mut rules = self.adaptation_rules.write();

        for (trigger, rule) in rules.iter_mut() {
            // Check if condition is met
            if let Some(value) = conditions.get(trigger) {
                if *value > rule.strength {
                    // Apply adaptation
                    self.execute_adaptation(&rule.adaptation_response);
                    rule.activation_count += 1;
                }
            }
        }
    }

    /// Execute an adaptation response
    fn execute_adaptation(&self, response: &str) {
        // Parse and execute adaptation response
        if response.contains("increase_efficiency") {
            let components = self.components.read();
            let ids: Vec<_> = components.keys().copied().collect();
            drop(components);

            for id in ids {
                if let Some(mut comp) = self.components.write().get_mut(&id) {
                    comp.efficiency = (comp.efficiency * 1.1).min(1.0);
                }
            }
        } else if response.contains("repair_all") {
            let components = self.components.read();
            let ids: Vec<_> = components.keys().copied().collect();
            drop(components);

            for id in ids {
                self.schedule_maintenance(id, MaintenanceType::Repair, 0.6);
            }
        }
    }

    /// Update overall system health
    fn update_system_health(&self) {
        let components = self.components.read();

        if components.is_empty() {
            *self.system_health.write() = 0.0;
            return;
        }

        let total_health: f64 = components.values()
            .map(|c| c.health * c.efficiency)
            .sum();

        let avg_health = total_health / components.len() as f64;
        *self.system_health.write() = avg_health;
    }

    /// Get overall system health
    pub fn get_system_health(&self) -> f64 {
        *self.system_health.read()
    }

    /// Get health status
    pub fn get_health_status(&self) -> HealthStatus {
        let health = self.get_system_health();

        if health > 0.9 {
            HealthStatus::Optimal
        } else if health > 0.7 {
            HealthStatus::Healthy
        } else if health > 0.5 {
            HealthStatus::Degraded
        } else if health > 0.2 {
            HealthStatus::Critical
        } else {
            HealthStatus::Failed
        }
    }

    /// Simulate system degradation over time
    pub fn simulate_degradation(&self, delta_time: f64) {
        let mut rng = rand::thread_rng();
        let mut components = self.components.write();

        for component in components.values_mut() {
            if component.active {
                // Natural degradation
                let degradation = rng.gen_range(0.0..0.05) * delta_time;
                component.health = (component.health - degradation).max(0.0);

                // Efficiency degradation based on health
                if component.health < 0.5 {
                    component.efficiency = (component.efficiency - degradation * 0.5).max(0.0);
                }
            }
        }

        drop(components);
        self.update_system_health();
    }

    /// Get all emergent patterns
    pub fn get_emergent_patterns(&self) -> Vec<EmergentPattern> {
        self.emergent_patterns.read().clone()
    }

    /// Get maintenance queue status
    pub fn get_maintenance_queue_size(&self) -> usize {
        self.maintenance_queue.read().len()
    }

    /// Get all components
    pub fn get_all_components(&self) -> Vec<SystemComponent> {
        self.components.read().values().cloned().collect()
    }
}

impl Default for AutopoeticProcessing {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autopoetic_creation() {
        let autopoetic = AutopoeticProcessing::new();
        assert_eq!(autopoetic.get_system_health(), 0.0);
    }

    #[test]
    fn test_component_management() {
        let autopoetic = AutopoeticProcessing::new();

        let id1 = autopoetic.add_component("Component A".to_string(), vec![]);
        let id2 = autopoetic.add_component("Component B".to_string(), vec![id1]);

        assert!(autopoetic.get_component(id1).is_some());
        assert!(autopoetic.get_component(id2).is_some());
        assert!(autopoetic.get_system_health() > 0.0);
    }

    #[test]
    fn test_maintenance_scheduling() {
        let autopoetic = AutopoeticProcessing::new();
        let id = autopoetic.add_component("Test".to_string(), vec![]);

        autopoetic.schedule_maintenance(id, MaintenanceType::Repair, 0.8);
        assert!(autopoetic.get_maintenance_queue_size() > 0);
    }

    #[test]
    fn test_health_degradation() {
        let autopoetic = AutopoeticProcessing::new();
        let id = autopoetic.add_component("Test".to_string(), vec![]);

        let initial_health = autopoetic.get_system_health();
        autopoetic.update_component_health(id, 0.3);

        assert!(autopoetic.get_system_health() < initial_health);
        assert_eq!(autopoetic.get_health_status(), HealthStatus::Degraded);
    }

    #[test]
    fn test_emergent_patterns() {
        let autopoetic = AutopoeticProcessing::new();

        let id1 = autopoetic.add_component("A".to_string(), vec![]);
        let id2 = autopoetic.add_component("B".to_string(), vec![id1]);
        let id3 = autopoetic.add_component("C".to_string(), vec![id2]);

        let patterns = autopoetic.detect_emergent_patterns();
        assert!(!patterns.is_empty());
    }
}
