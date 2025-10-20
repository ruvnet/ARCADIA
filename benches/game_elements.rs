//! Benchmarks for Game Elements performance

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use arcadia::game_elements::*;
use arcadia::code_dna::CodeDNA;

fn benchmark_functional_component_creation(c: &mut Criterion) {
    c.bench_function("functional_component_new", |b| {
        b.iter(|| {
            FunctionalComponent::new(
                black_box("comp1".to_string()),
                black_box(ComponentType::Character),
                black_box((1.0, 2.0, 3.0)),
            );
        });
    });
}

fn benchmark_functional_component_update(c: &mut Criterion) {
    c.bench_function("functional_component_update_position", |b| {
        let mut component = FunctionalComponent::new(
            "comp1".to_string(),
            ComponentType::Character,
            (0.0, 0.0, 0.0),
        );
        b.iter(|| {
            component.update_position(black_box((1.0, 1.0, 1.0)));
        });
    });
}

fn benchmark_game_elements_creation(c: &mut Criterion) {
    c.bench_function("game_elements_new", |b| {
        let dna = CodeDNA::default_scifi();
        b.iter(|| {
            GameElements::new(black_box(dna.clone()));
        });
    });
}

fn benchmark_game_elements_update(c: &mut Criterion) {
    let mut group = c.benchmark_group("game_elements_update");

    for component_count in [10, 100, 1000].iter() {
        let dna = CodeDNA::default_scifi();
        let mut game = GameElements::new(dna);

        for i in 0..*component_count {
            let component = FunctionalComponent::new(
                format!("comp_{}", i),
                ComponentType::Character,
                (i as f32, 0.0, 0.0),
            );
            game.add_component(component);
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(component_count),
            component_count,
            |b, _| {
                b.iter(|| {
                    game.update(black_box(1.0));
                });
            },
        );
    }

    group.finish();
}

fn benchmark_entropy_system(c: &mut Criterion) {
    c.bench_function("entropy_add_object", |b| {
        let mut entropy = Entropy::new(0.01);
        b.iter(|| {
            entropy.add_object(black_box("obj1".to_string()), black_box(1.0));
        });
    });

    c.bench_function("entropy_update_1000_objects", |b| {
        let mut entropy = Entropy::new(0.01);
        for i in 0..1000 {
            entropy.add_object(format!("obj_{}", i), 1.0);
        }
        b.iter(|| {
            entropy.update(black_box(0.1));
        });
    });
}

fn benchmark_social_constructs(c: &mut Criterion) {
    c.bench_function("social_add_faction", |b| {
        let mut social = SocialConstructs::new();
        b.iter(|| {
            social.add_faction(
                black_box("Faction".to_string()),
                black_box(0.5),
                black_box(0.7),
            );
        });
    });

    c.bench_function("social_update_reputation", |b| {
        let mut social = SocialConstructs::new();
        social.add_faction("Faction1".to_string(), 0.5, 0.7);
        b.iter(|| {
            social.update_reputation(
                black_box("player1".to_string()),
                black_box("Faction1"),
                black_box(0.1),
            );
        });
    });
}

criterion_group!(
    benches,
    benchmark_functional_component_creation,
    benchmark_functional_component_update,
    benchmark_game_elements_creation,
    benchmark_game_elements_update,
    benchmark_entropy_system,
    benchmark_social_constructs
);
criterion_main!(benches);
