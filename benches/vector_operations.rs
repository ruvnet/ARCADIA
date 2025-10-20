use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use ndarray::{Array1, Array2};
use nalgebra::DVector;
use rayon::prelude::*;

/// Benchmark vector operations using ndarray
fn bench_ndarray_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("ndarray_operations");

    for size in [100, 1000, 10000].iter() {
        let vec1 = Array1::from_vec((0..*size).map(|x| x as f32).collect());
        let vec2 = Array1::from_vec((0..*size).map(|x| (x * 2) as f32).collect());

        group.bench_with_input(BenchmarkId::new("dot_product", size), size, |b, _| {
            b.iter(|| {
                black_box(vec1.dot(&vec2))
            });
        });

        group.bench_with_input(BenchmarkId::new("element_wise_add", size), size, |b, _| {
            b.iter(|| {
                black_box(&vec1 + &vec2)
            });
        });

        group.bench_with_input(BenchmarkId::new("norm", size), size, |b, _| {
            b.iter(|| {
                black_box(vec1.iter().map(|x| x * x).sum::<f32>().sqrt())
            });
        });
    }

    group.finish();
}

/// Benchmark parallel vector operations
fn bench_parallel_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_operations");

    for size in [1000, 10000, 100000].iter() {
        let data: Vec<f32> = (0..*size).map(|x| x as f32).collect();

        group.bench_with_input(BenchmarkId::new("sequential_sum", size), size, |b, _| {
            b.iter(|| {
                black_box(data.iter().sum::<f32>())
            });
        });

        group.bench_with_input(BenchmarkId::new("parallel_sum", size), size, |b, _| {
            b.iter(|| {
                black_box(data.par_iter().sum::<f32>())
            });
        });

        group.bench_with_input(BenchmarkId::new("sequential_map", size), size, |b, _| {
            b.iter(|| {
                black_box(data.iter().map(|x| x * 2.0).collect::<Vec<f32>>())
            });
        });

        group.bench_with_input(BenchmarkId::new("parallel_map", size), size, |b, _| {
            b.iter(|| {
                black_box(data.par_iter().map(|x| x * 2.0).collect::<Vec<f32>>())
            });
        });
    }

    group.finish();
}

/// Benchmark cosine similarity calculations
fn bench_cosine_similarity(c: &mut Criterion) {
    let mut group = c.benchmark_group("cosine_similarity");

    for dim in [128, 512, 1536].iter() {
        let vec1 = DVector::from_fn(*dim, |i, _| i as f64);
        let vec2 = DVector::from_fn(*dim, |i, _| (i * 2) as f64);

        group.bench_with_input(BenchmarkId::new("nalgebra", dim), dim, |b, _| {
            b.iter(|| {
                let dot = vec1.dot(&vec2);
                let norm1 = vec1.norm();
                let norm2 = vec2.norm();
                black_box(dot / (norm1 * norm2))
            });
        });
    }

    group.finish();
}

/// Benchmark matrix operations for batch processing
fn bench_matrix_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix_operations");

    for size in [10, 50, 100].iter() {
        let matrix = Array2::from_shape_fn((*size, *size), |(i, j)| (i + j) as f32);
        let vector = Array1::from_vec((0..*size).map(|x| x as f32).collect());

        group.bench_with_input(BenchmarkId::new("matrix_vector_mul", size), size, |b, _| {
            b.iter(|| {
                black_box(matrix.dot(&vector))
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_ndarray_operations,
    bench_parallel_operations,
    bench_cosine_similarity,
    bench_matrix_operations
);
criterion_main!(benches);
