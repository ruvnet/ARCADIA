//! Benchmarks for AI Systems performance

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use arcadia::ai_systems::*;

fn benchmark_neo_cortex_decision(c: &mut Criterion) {
    c.bench_function("neo_cortex_decision", |b| {
        let mut cortex = NeoCortexReasoning::new();
        cortex.learn("combat", 0.8);
        b.iter(|| {
            cortex.make_decision(black_box("combat"));
        });
    });
}

fn benchmark_neo_cortex_learning(c: &mut Criterion) {
    c.bench_function("neo_cortex_learning", |b| {
        let mut cortex = NeoCortexReasoning::new();
        b.iter(|| {
            cortex.learn(black_box("context"), black_box(0.75));
        });
    });
}

fn benchmark_evolutionary_feedback(c: &mut Criterion) {
    c.bench_function("evolutionary_evolve", |b| {
        let mut evolution = EvolutionaryFeedback::new(0.1);
        let traits = vec![0.5; 100];
        b.iter(|| {
            evolution.evolve(black_box(traits.clone()), black_box(0.8));
        });
    });
}

fn benchmark_autopoetic_update(c: &mut Criterion) {
    c.bench_function("autopoetic_update_100_components", |b| {
        let mut autopoetic = AutopoeticProcessing::new(0.5);
        for i in 0..100 {
            autopoetic.add_component(format!("component_{}", i));
        }
        b.iter(|| {
            autopoetic.update(black_box(1.0));
        });
    });
}

fn benchmark_self_awareness(c: &mut Criterion) {
    c.bench_function("self_awareness_record_experience", |b| {
        let mut awareness = SelfAwareness::new("NPC".to_string(), "Guard".to_string());
        b.iter(|| {
            awareness.record_experience(black_box("Event".to_string()), black_box(0.5));
        });
    });

    c.bench_function("self_awareness_emotional_state", |b| {
        let mut awareness = SelfAwareness::new("NPC".to_string(), "Guard".to_string());
        for i in 0..100 {
            awareness.record_experience(format!("Event {}", i), 0.5);
        }
        b.iter(|| {
            awareness.emotional_state();
        });
    });
}

fn benchmark_adaptive_perspectives(c: &mut Criterion) {
    c.bench_function("adaptive_perspectives_adapt", |b| {
        let mut perspectives = AdaptivePerspectives::new();
        b.iter(|| {
            perspectives.adapt(black_box("strategy"), black_box(0.7));
        });
    });
}

fn benchmark_emotion_adaptive(c: &mut Criterion) {
    c.bench_function("emotion_adapt_environment", |b| {
        let mut emotions = EmotionAdaptiveExperiences::new();
        emotions.set_emotion(Emotion::Fear, 0.8);
        b.iter(|| {
            emotions.adapt_environment();
        });
    });
}

criterion_group!(
    benches,
    benchmark_neo_cortex_decision,
    benchmark_neo_cortex_learning,
    benchmark_evolutionary_feedback,
    benchmark_autopoetic_update,
    benchmark_self_awareness,
    benchmark_adaptive_perspectives,
    benchmark_emotion_adaptive
);
criterion_main!(benches);
