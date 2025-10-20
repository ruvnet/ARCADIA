use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, async_executor::FuturesExecutor};
use std::time::Duration;

// Import cache types
use moka::future::Cache;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Benchmark cache hit performance
fn bench_cache_hits(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_hits");

    let rt = tokio::runtime::Runtime::new().unwrap();

    // Moka cache
    group.bench_function("moka_cache_hit", |b| {
        let cache = rt.block_on(async {
            let cache = Cache::builder()
                .max_capacity(1000)
                .time_to_live(Duration::from_secs(300))
                .build();

            // Pre-populate
            for i in 0..100 {
                cache.insert(i, format!("value_{}", i)).await;
            }

            cache
        });

        b.to_async(FuturesExecutor).iter(|| async {
            black_box(cache.get(&50).await)
        });
    });

    // HashMap with Mutex (baseline)
    group.bench_function("hashmap_mutex_hit", |b| {
        let map = Arc::new(Mutex::new({
            let mut m = HashMap::new();
            for i in 0..100 {
                m.insert(i, format!("value_{}", i));
            }
            m
        }));

        b.iter(|| {
            let m = map.lock().unwrap();
            black_box(m.get(&50).cloned())
        });
    });

    group.finish();
}

/// Benchmark cache miss and insertion
fn bench_cache_misses(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_misses");

    let rt = tokio::runtime::Runtime::new().unwrap();

    group.bench_function("moka_cache_miss_insert", |b| {
        let cache = Cache::builder()
            .max_capacity(1000)
            .time_to_live(Duration::from_secs(300))
            .build();

        let mut counter = 0;

        b.to_async(FuturesExecutor).iter(|| async {
            counter += 1;
            cache.insert(counter, format!("value_{}", counter)).await;
            black_box(cache.get(&counter).await)
        });
    });

    group.finish();
}

/// Benchmark cache eviction
fn bench_cache_eviction(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_eviction");

    group.bench_function("lru_eviction", |b| {
        let cache = Cache::builder()
            .max_capacity(100)
            .build();

        let rt = tokio::runtime::Runtime::new().unwrap();

        b.iter(|| {
            rt.block_on(async {
                for i in 0..200 {
                    cache.insert(i, format!("value_{}", i)).await;
                }
                black_box(cache.entry_count())
            })
        });
    });

    group.finish();
}

/// Benchmark concurrent cache access
fn bench_concurrent_cache(c: &mut Criterion) {
    use rayon::prelude::*;

    let mut group = c.benchmark_group("concurrent_cache");

    group.bench_function("concurrent_reads", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let cache = rt.block_on(async {
            let cache = Cache::builder()
                .max_capacity(1000)
                .build();

            for i in 0..1000 {
                cache.insert(i, format!("value_{}", i)).await;
            }

            Arc::new(cache)
        });

        b.iter(|| {
            let handles: Vec<_> = (0..10).map(|_| {
                let cache = cache.clone();
                std::thread::spawn(move || {
                    rt.block_on(async {
                        for i in 0..100 {
                            black_box(cache.get(&i).await);
                        }
                    })
                })
            }).collect();

            for handle in handles {
                handle.join().unwrap();
            }
        });
    });

    group.finish();
}

/// Benchmark get_or_insert pattern
fn bench_get_or_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_or_insert");

    let rt = tokio::runtime::Runtime::new().unwrap();

    group.bench_function("get_or_insert_with", |b| {
        let cache = Cache::builder()
            .max_capacity(1000)
            .build();

        let mut counter = 0;

        b.to_async(FuturesExecutor).iter(|| async {
            counter += 1;
            let key = counter % 100; // Reuse some keys
            cache.get_or_insert_with(key, async move {
                // Simulate expensive computation
                format!("computed_value_{}", key)
            }).await;
            black_box(())
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_cache_hits,
    bench_cache_misses,
    bench_cache_eviction,
    bench_concurrent_cache,
    bench_get_or_insert
);
criterion_main!(benches);
