//! Benchmarks for Vector Index performance

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use arcadia::vector_index::*;

fn create_test_vectors(count: usize, dimension: usize) -> Vec<Vector> {
    (0..count)
        .map(|i| {
            let data: Vec<f32> = (0..dimension).map(|j| (i * j) as f32).collect();
            Vector::new(format!("vec_{}", i), data)
        })
        .collect()
}

fn benchmark_vector_insertion(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector_insertion");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let vectors = create_test_vectors(size, 128);
            b.iter(|| {
                let config = VectorIndexConfig::new("http://localhost".to_string(), "key".to_string());
                let mut index = VectorIndex::new(config);
                for vec in &vectors {
                    index.insert(vec.clone()).unwrap();
                }
            });
        });
    }

    group.finish();
}

fn benchmark_vector_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector_search");

    for size in [100, 1000, 10000].iter() {
        let config = VectorIndexConfig::new("http://localhost".to_string(), "key".to_string());
        let mut index = VectorIndex::new(config);
        let vectors = create_test_vectors(*size, 128);
        for vec in vectors {
            index.insert(vec).unwrap();
        }

        let query = Vector::new("query".to_string(), vec![1.0; 128]);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                index.search(black_box(&query), black_box(10)).unwrap();
            });
        });
    }

    group.finish();
}

fn benchmark_cosine_similarity(c: &mut Criterion) {
    let mut group = c.benchmark_group("cosine_similarity");

    for dim in [64, 128, 256, 512].iter() {
        let vec1 = Vector::new("vec1".to_string(), vec![1.0; *dim]);
        let vec2 = Vector::new("vec2".to_string(), vec![0.5; *dim]);

        group.bench_with_input(BenchmarkId::from_parameter(dim), dim, |b, _| {
            b.iter(|| {
                vec1.cosine_similarity(black_box(&vec2)).unwrap();
            });
        });
    }

    group.finish();
}

fn benchmark_vector_normalization(c: &mut Criterion) {
    c.bench_function("vector_normalize_128d", |b| {
        let mut vec = Vector::new("vec".to_string(), vec![2.0; 128]);
        b.iter(|| {
            black_box(&mut vec).normalize();
        });
    });
}

criterion_group!(
    benches,
    benchmark_vector_insertion,
    benchmark_vector_search,
    benchmark_cosine_similarity,
    benchmark_vector_normalization
);
criterion_main!(benches);
