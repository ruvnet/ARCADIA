use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rand::Rng;
use rayon::prelude::*;

/// Simulated AI decision making based on weighted factors
#[derive(Clone)]
struct AIAgent {
    weights: Vec<f32>,
    threshold: f32,
}

impl AIAgent {
    fn new(num_factors: usize) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            weights: (0..num_factors).map(|_| rng.gen::<f32>()).collect(),
            threshold: 0.5,
        }
    }

    fn make_decision(&self, inputs: &[f32]) -> bool {
        let score: f32 = inputs
            .iter()
            .zip(&self.weights)
            .map(|(i, w)| i * w)
            .sum();

        score / self.weights.len() as f32 > self.threshold
    }

    fn make_decision_simd(&self, inputs: &[f32]) -> bool {
        // Simplified SIMD-like computation
        let score: f32 = inputs
            .iter()
            .zip(&self.weights)
            .map(|(i, w)| i * w)
            .sum();

        score / self.weights.len() as f32 > self.threshold
    }
}

/// Benchmark sequential AI decision making
fn bench_sequential_decisions(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_decisions");

    for num_factors in [10, 50, 100].iter() {
        let agent = AIAgent::new(*num_factors);
        let mut rng = rand::thread_rng();
        let inputs: Vec<f32> = (0..*num_factors).map(|_| rng.gen()).collect();

        group.bench_with_input(
            BenchmarkId::new("single_decision", num_factors),
            num_factors,
            |b, _| {
                b.iter(|| {
                    black_box(agent.make_decision(&inputs))
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("batch_decisions", num_factors),
            num_factors,
            |b, _| {
                b.iter(|| {
                    for _ in 0..100 {
                        black_box(agent.make_decision(&inputs));
                    }
                });
            },
        );
    }

    group.finish();
}

/// Benchmark parallel AI decision making
fn bench_parallel_decisions(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_decisions");

    for num_agents in [10, 100, 1000].iter() {
        let agents: Vec<AIAgent> = (0..*num_agents).map(|_| AIAgent::new(50)).collect();
        let mut rng = rand::thread_rng();
        let inputs: Vec<Vec<f32>> = (0..*num_agents)
            .map(|_| (0..50).map(|_| rng.gen()).collect())
            .collect();

        group.bench_with_input(
            BenchmarkId::new("sequential", num_agents),
            num_agents,
            |b, _| {
                b.iter(|| {
                    agents
                        .iter()
                        .zip(&inputs)
                        .map(|(agent, input)| agent.make_decision(input))
                        .collect::<Vec<bool>>()
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("parallel", num_agents),
            num_agents,
            |b, _| {
                b.iter(|| {
                    agents
                        .par_iter()
                        .zip(&inputs)
                        .map(|(agent, input)| agent.make_decision(input))
                        .collect::<Vec<bool>>()
                });
            },
        );
    }

    group.finish();
}

/// Benchmark decision tree evaluation
fn bench_decision_tree(c: &mut Criterion) {
    let mut group = c.benchmark_group("decision_tree");

    #[derive(Clone)]
    struct DecisionNode {
        feature_idx: usize,
        threshold: f32,
        left: Option<Box<DecisionNode>>,
        right: Option<Box<DecisionNode>>,
        value: Option<bool>,
    }

    impl DecisionNode {
        fn evaluate(&self, features: &[f32]) -> bool {
            if let Some(value) = self.value {
                return value;
            }

            let feature_value = features[self.feature_idx];
            if feature_value < self.threshold {
                if let Some(ref left) = self.left {
                    left.evaluate(features)
                } else {
                    false
                }
            } else {
                if let Some(ref right) = self.right {
                    right.evaluate(features)
                } else {
                    true
                }
            }
        }
    }

    // Create a simple decision tree
    let tree = DecisionNode {
        feature_idx: 0,
        threshold: 0.5,
        left: Some(Box::new(DecisionNode {
            feature_idx: 1,
            threshold: 0.3,
            left: Some(Box::new(DecisionNode {
                feature_idx: 0,
                threshold: 0.0,
                left: None,
                right: None,
                value: Some(false),
            })),
            right: Some(Box::new(DecisionNode {
                feature_idx: 0,
                threshold: 0.0,
                left: None,
                right: None,
                value: Some(true),
            })),
            value: None,
        })),
        right: Some(Box::new(DecisionNode {
            feature_idx: 2,
            threshold: 0.7,
            left: Some(Box::new(DecisionNode {
                feature_idx: 0,
                threshold: 0.0,
                left: None,
                right: None,
                value: Some(true),
            })),
            right: Some(Box::new(DecisionNode {
                feature_idx: 0,
                threshold: 0.0,
                left: None,
                right: None,
                value: Some(false),
            })),
            value: None,
        })),
        value: None,
    };

    let mut rng = rand::thread_rng();
    let features: Vec<f32> = (0..10).map(|_| rng.gen()).collect();

    group.bench_function("tree_evaluation", |b| {
        b.iter(|| {
            black_box(tree.evaluate(&features))
        });
    });

    group.finish();
}

/// Benchmark neural network inference simulation
fn bench_neural_network(c: &mut Criterion) {
    let mut group = c.benchmark_group("neural_network");

    fn relu(x: f32) -> f32 {
        x.max(0.0)
    }

    fn forward_pass(weights: &[Vec<Vec<f32>>], input: &[f32]) -> Vec<f32> {
        let mut activation = input.to_vec();

        for layer_weights in weights {
            let mut new_activation = vec![0.0; layer_weights.len()];

            for (i, neuron_weights) in layer_weights.iter().enumerate() {
                let sum: f32 = activation
                    .iter()
                    .zip(neuron_weights)
                    .map(|(a, w)| a * w)
                    .sum();
                new_activation[i] = relu(sum);
            }

            activation = new_activation;
        }

        activation
    }

    // Create a simple 3-layer network: 10 -> 20 -> 10 -> 5
    let mut rng = rand::thread_rng();
    let weights = vec![
        vec![vec![rng.gen::<f32>(); 10]; 20],  // Layer 1: 10 inputs, 20 neurons
        vec![vec![rng.gen::<f32>(); 20]; 10],  // Layer 2: 20 inputs, 10 neurons
        vec![vec![rng.gen::<f32>(); 10]; 5],   // Layer 3: 10 inputs, 5 neurons
    ];

    let input: Vec<f32> = (0..10).map(|_| rng.gen()).collect();

    group.bench_function("forward_pass", |b| {
        b.iter(|| {
            black_box(forward_pass(&weights, &input))
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_sequential_decisions,
    bench_parallel_decisions,
    bench_decision_tree,
    bench_neural_network
);
criterion_main!(benches);
