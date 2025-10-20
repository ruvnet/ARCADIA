//! Benchmarks for CodeDNA performance

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use arcadia::code_dna::*;

fn benchmark_code_dna_creation(c: &mut Criterion) {
    c.bench_function("code_dna_new", |b| {
        b.iter(|| {
            CodeDNA::new(
                black_box("Setting"),
                black_box("Tech"),
                black_box(&vec![String::from("Law1"), String::from("Law2")]),
                black_box(&vec![String::from("Theme1")]),
                black_box(1.0),
                black_box(0.1),
                black_box(&vec![String::from("Natural1")]),
            );
        });
    });
}

fn benchmark_code_dna_validation(c: &mut Criterion) {
    let dna = CodeDNA::default_scifi();
    c.bench_function("code_dna_validate", |b| {
        b.iter(|| {
            black_box(&dna).validate().unwrap();
        });
    });
}

fn benchmark_code_dna_apply(c: &mut Criterion) {
    let dna = CodeDNA::default_fantasy();
    c.bench_function("code_dna_apply_to_world", |b| {
        let mut world = GameWorld::new();
        b.iter(|| {
            black_box(&dna).apply_to_game_world(black_box(&mut world));
        });
    });
}

fn benchmark_default_configs(c: &mut Criterion) {
    c.bench_function("code_dna_default_scifi", |b| {
        b.iter(|| {
            CodeDNA::default_scifi();
        });
    });

    c.bench_function("code_dna_default_fantasy", |b| {
        b.iter(|| {
            CodeDNA::default_fantasy();
        });
    });
}

criterion_group!(
    benches,
    benchmark_code_dna_creation,
    benchmark_code_dna_validation,
    benchmark_code_dna_apply,
    benchmark_default_configs
);
criterion_main!(benches);
