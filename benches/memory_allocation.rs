use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::collections::HashMap;
use dashmap::DashMap;
use smallvec::SmallVec;

/// Benchmark standard Vec vs SmallVec
fn bench_small_vec(c: &mut Criterion) {
    let mut group = c.benchmark_group("small_vec");

    for size in [4, 8, 16].iter() {
        group.bench_with_input(BenchmarkId::new("vec_allocation", size), size, |b, &s| {
            b.iter(|| {
                let mut v = Vec::with_capacity(s);
                for i in 0..s {
                    v.push(i);
                }
                black_box(v)
            });
        });

        group.bench_with_input(BenchmarkId::new("smallvec_allocation", size), size, |b, &s| {
            b.iter(|| {
                let mut v: SmallVec<[usize; 16]> = SmallVec::new();
                for i in 0..s {
                    v.push(i);
                }
                black_box(v)
            });
        });
    }

    group.finish();
}

/// Benchmark HashMap vs DashMap for concurrent access
fn bench_concurrent_maps(c: &mut Criterion) {
    use std::sync::{Arc, Mutex};

    let mut group = c.benchmark_group("concurrent_maps");

    // Standard HashMap with Mutex
    group.bench_function("hashmap_insert_mutex", |b| {
        let map = Arc::new(Mutex::new(HashMap::new()));
        b.iter(|| {
            let map = map.clone();
            let mut m = map.lock().unwrap();
            for i in 0..100 {
                m.insert(i, i * 2);
            }
        });
    });

    // DashMap (lock-free)
    group.bench_function("dashmap_insert", |b| {
        let map = Arc::new(DashMap::new());
        b.iter(|| {
            let map = map.clone();
            for i in 0..100 {
                map.insert(i, i * 2);
            }
        });
    });

    group.finish();
}

/// Benchmark allocation patterns
fn bench_allocation_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocation_patterns");

    group.bench_function("frequent_small_allocations", |b| {
        b.iter(|| {
            let mut vecs = Vec::new();
            for i in 0..1000 {
                let v = vec![i; 10];
                vecs.push(v);
            }
            black_box(vecs)
        });
    });

    group.bench_function("single_large_allocation", |b| {
        b.iter(|| {
            let v: Vec<usize> = (0..10000).collect();
            black_box(v)
        });
    });

    group.bench_function("preallocated_vec", |b| {
        b.iter(|| {
            let mut v = Vec::with_capacity(10000);
            for i in 0..10000 {
                v.push(i);
            }
            black_box(v)
        });
    });

    group.finish();
}

/// Benchmark string allocations
fn bench_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");

    group.bench_function("string_concatenation", |b| {
        b.iter(|| {
            let mut s = String::new();
            for i in 0..100 {
                s.push_str(&format!("item{}", i));
            }
            black_box(s)
        });
    });

    group.bench_function("string_preallocated", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(1000);
            for i in 0..100 {
                s.push_str(&format!("item{}", i));
            }
            black_box(s)
        });
    });

    group.bench_function("format_macro", |b| {
        b.iter(|| {
            let parts: Vec<String> = (0..100).map(|i| format!("item{}", i)).collect();
            black_box(parts.join(""))
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_small_vec,
    bench_concurrent_maps,
    bench_allocation_patterns,
    bench_string_operations
);
criterion_main!(benches);
