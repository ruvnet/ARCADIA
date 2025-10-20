//! Performance metrics and monitoring for ARCADIA
//!
//! Provides real-time performance tracking for vector operations, AI decisions,
//! cache hit rates, and system resources.

use metrics::{counter, gauge, histogram};
use std::time::{Duration, Instant};

/// Initialize metrics system with Prometheus exporter
pub fn init_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let builder = metrics_exporter_prometheus::PrometheusBuilder::new();
    builder.install()?;
    Ok(())
}

/// Metrics for vector index operations
pub mod vector_metrics {
    use super::*;

    /// Record a vector embedding operation
    pub fn record_embedding(duration: Duration) {
        histogram!("arcadia_vector_embedding_duration_seconds").record(duration.as_secs_f64());
        counter!("arcadia_vector_embedding_total").increment(1);
    }

    /// Record a vector search operation
    pub fn record_search(duration: Duration, results: usize) {
        histogram!("arcadia_vector_search_duration_seconds").record(duration.as_secs_f64());
        histogram!("arcadia_vector_search_results").record(results as f64);
        counter!("arcadia_vector_search_total").increment(1);
    }

    /// Record vector dimension
    pub fn set_vector_dimension(dim: usize) {
        gauge!("arcadia_vector_dimension").set(dim as f64);
    }

    /// Record vector storage operation
    pub fn record_store(duration: Duration) {
        histogram!("arcadia_vector_store_duration_seconds").record(duration.as_secs_f64());
        counter!("arcadia_vector_store_total").increment(1);
    }

    /// Record a vector operation error
    pub fn record_error(operation: &str) {
        counter!("arcadia_vector_errors_total", "operation" => operation.to_string())
            .increment(1);
    }
}

/// Metrics for AI decision-making
pub mod ai_metrics {
    use super::*;

    /// Record AI decision latency
    pub fn record_decision(duration: Duration, decision_type: &str) {
        histogram!(
            "arcadia_ai_decision_duration_seconds",
            "type" => decision_type.to_string()
        )
        .record(duration.as_secs_f64());
        counter!("arcadia_ai_decisions_total", "type" => decision_type.to_string()).increment(1);
    }

    /// Record AI model inference
    pub fn record_inference(duration: Duration, model: &str) {
        histogram!(
            "arcadia_ai_inference_duration_seconds",
            "model" => model.to_string()
        )
        .record(duration.as_secs_f64());
        counter!("arcadia_ai_inference_total", "model" => model.to_string()).increment(1);
    }

    /// Record active AI agents
    pub fn set_active_agents(count: usize) {
        gauge!("arcadia_ai_active_agents").set(count as f64);
    }

    /// Record AI decision confidence
    pub fn record_confidence(confidence: f64, decision_type: &str) {
        histogram!(
            "arcadia_ai_decision_confidence",
            "type" => decision_type.to_string()
        )
        .record(confidence);
    }
}

/// Metrics for caching performance
pub mod cache_metrics {
    use super::*;

    /// Record cache hit
    pub fn record_hit(cache_type: &str) {
        counter!("arcadia_cache_hits_total", "type" => cache_type.to_string()).increment(1);
    }

    /// Record cache miss
    pub fn record_miss(cache_type: &str) {
        counter!("arcadia_cache_misses_total", "type" => cache_type.to_string()).increment(1);
    }

    /// Update cache size
    pub fn set_cache_size(cache_type: &str, size: u64) {
        gauge!("arcadia_cache_size_entries", "type" => cache_type.to_string()).set(size as f64);
    }

    /// Update cache utilization
    pub fn set_cache_utilization(cache_type: &str, utilization: f64) {
        gauge!("arcadia_cache_utilization_percent", "type" => cache_type.to_string())
            .set(utilization);
    }

    /// Record cache eviction
    pub fn record_eviction(cache_type: &str) {
        counter!("arcadia_cache_evictions_total", "type" => cache_type.to_string()).increment(1);
    }
}

/// Metrics for memory management
pub mod memory_metrics {
    use super::*;

    /// Update heap memory usage
    pub fn set_heap_usage(bytes: usize) {
        gauge!("arcadia_memory_heap_bytes").set(bytes as f64);
    }

    /// Update pool memory usage
    pub fn set_pool_usage(pool_name: &str, bytes: usize) {
        gauge!("arcadia_memory_pool_bytes", "pool" => pool_name.to_string()).set(bytes as f64);
    }

    /// Record allocation
    pub fn record_allocation(bytes: usize) {
        counter!("arcadia_memory_allocations_total").increment(1);
        histogram!("arcadia_memory_allocation_size_bytes").record(bytes as f64);
    }

    /// Record deallocation
    pub fn record_deallocation(bytes: usize) {
        counter!("arcadia_memory_deallocations_total").increment(1);
        histogram!("arcadia_memory_deallocation_size_bytes").record(bytes as f64);
    }

    /// Set peak memory usage
    pub fn set_peak_usage(bytes: usize) {
        gauge!("arcadia_memory_peak_bytes").set(bytes as f64);
    }
}

/// Metrics for game world simulation
pub mod world_metrics {
    use super::*;

    /// Record simulation tick duration
    pub fn record_tick(duration: Duration) {
        histogram!("arcadia_world_tick_duration_seconds").record(duration.as_secs_f64());
        counter!("arcadia_world_ticks_total").increment(1);
    }

    /// Update entity count
    pub fn set_entity_count(count: usize) {
        gauge!("arcadia_world_entities").set(count as f64);
    }

    /// Record world update
    pub fn record_update(duration: Duration, component_type: &str) {
        histogram!(
            "arcadia_world_update_duration_seconds",
            "component" => component_type.to_string()
        )
        .record(duration.as_secs_f64());
    }

    /// Update world time
    pub fn set_world_time(time: f64) {
        gauge!("arcadia_world_time_seconds").set(time);
    }
}

/// Performance timer for automatic metrics recording
pub struct MetricsTimer {
    start: Instant,
    name: String,
    labels: Vec<(String, String)>,
}

impl MetricsTimer {
    /// Create a new metrics timer
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            start: Instant::now(),
            name: name.into(),
            labels: Vec::new(),
        }
    }

    /// Add a label to the timer
    pub fn with_label(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.labels.push((key.into(), value.into()));
        self
    }

    /// Get elapsed duration
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Record the timer as a histogram
    pub fn record(self) {
        let duration = self.elapsed();
        let mut hist = histogram!(self.name.clone());
        for (key, value) in &self.labels {
            hist = hist.with_label(key.clone(), value.clone());
        }
        hist.record(duration.as_secs_f64());
    }
}

impl Drop for MetricsTimer {
    fn drop(&mut self) {
        // Auto-record on drop if not explicitly recorded
        let duration = self.elapsed();
        let mut hist = histogram!(self.name.clone());
        for (key, value) in &self.labels {
            hist = hist.with_label(key.clone(), value.clone());
        }
        hist.record(duration.as_secs_f64());
    }
}

/// System-wide metrics snapshot
#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    /// Timestamp of the snapshot
    pub timestamp: Instant,
    /// Vector operations per second
    pub vector_ops_per_sec: f64,
    /// AI decisions per second
    pub ai_decisions_per_sec: f64,
    /// Cache hit rate percentage
    pub cache_hit_rate: f64,
    /// Current memory usage in bytes
    pub memory_usage_bytes: usize,
    /// Current entity count
    pub entity_count: usize,
}

impl MetricsSnapshot {
    /// Create a new empty metrics snapshot
    pub fn new() -> Self {
        Self {
            timestamp: Instant::now(),
            vector_ops_per_sec: 0.0,
            ai_decisions_per_sec: 0.0,
            cache_hit_rate: 0.0,
            memory_usage_bytes: 0,
            entity_count: 0,
        }
    }
}

impl Default for MetricsSnapshot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_timer() {
        let timer = MetricsTimer::new("test_operation");
        std::thread::sleep(Duration::from_millis(10));
        let elapsed = timer.elapsed();
        assert!(elapsed >= Duration::from_millis(10));
    }

    #[test]
    fn test_metrics_timer_with_labels() {
        let timer = MetricsTimer::new("test_operation")
            .with_label("operation", "read")
            .with_label("status", "success");

        std::thread::sleep(Duration::from_millis(5));
        assert!(timer.elapsed() >= Duration::from_millis(5));
    }

    #[test]
    fn test_metrics_snapshot() {
        let snapshot = MetricsSnapshot::new();
        assert_eq!(snapshot.vector_ops_per_sec, 0.0);
        assert_eq!(snapshot.cache_hit_rate, 0.0);
        assert_eq!(snapshot.entity_count, 0);
    }
}
