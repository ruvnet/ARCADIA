# ARCADIA Test Suite

Comprehensive testing infrastructure for the ARCADIA game engine.

## Test Structure

```
tests/
├── unit/              # Unit tests for individual modules
├── integration/       # Integration tests for multi-component workflows
├── fixtures/          # Test data, configurations, and mock objects
└── README.md         # This file

benches/
├── vector_index.rs   # Vector index performance benchmarks
├── ai_systems.rs     # AI systems performance benchmarks
├── code_dna.rs       # CodeDNA performance benchmarks
└── game_elements.rs  # Game elements performance benchmarks
```

## Running Tests

### All Tests
```bash
cargo test
```

### Unit Tests Only
```bash
cargo test --lib
```

### Integration Tests Only
```bash
cargo test --test '*'
```

### Specific Test Module
```bash
cargo test --test game_workflow_test
cargo test --test validation_integration_test
```

### With Output
```bash
cargo test -- --nocapture
```

### Single Test
```bash
cargo test test_code_dna_creation
```

## Running Benchmarks

### All Benchmarks
```bash
cargo bench
```

### Specific Benchmark Suite
```bash
cargo bench --bench vector_index
cargo bench --bench ai_systems
cargo bench --bench code_dna
cargo bench --bench game_elements
```

### Generate HTML Reports
```bash
cargo bench -- --save-baseline main
```

Benchmark reports are generated in `target/criterion/`.

## Code Coverage

### Generate Coverage Report
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --verbose --all-features --workspace --timeout 120
```

### Generate HTML Coverage
```bash
cargo tarpaulin --out Html
```

Coverage reports are generated in `target/tarpaulin/`.

### Coverage Goals
- Overall: >80%
- Core modules (code_dna, vector_index, authentication): >90%
- AI systems: >85%
- Game elements: >85%
- Validation framework: >95%

## Test Categories

### Unit Tests

Located in each module file (e.g., `src/code_dna.rs`):
- `test_code_dna_creation()` - Tests CodeDNA instantiation
- `test_code_dna_validation()` - Tests validation logic
- `test_vector_index_insert()` - Tests vector insertion
- `test_authentication_workflow()` - Tests auth flow

### Integration Tests

Located in `tests/integration/`:
- **game_workflow_test.rs** - End-to-end game initialization and workflows
- **validation_integration_test.rs** - Complete validation system tests

### Performance Tests

Located in `benches/`:
- Vector index operations (insertion, search, similarity)
- AI decision-making speed
- Game element updates
- Memory usage patterns

## Test Fixtures

### Using Fixtures

```rust
use arcadia::tests::fixtures;

#[test]
fn my_test() {
    let game = fixtures::create_test_game();
    let vectors = fixtures::create_test_vectors(100, 128);
    // ... test code
}
```

### Available Fixtures

- `create_test_vector_config()` - Test vector index configuration
- `create_test_auth_config()` - Test authentication configuration
- `create_scifi_dna()` - Sci-fi CodeDNA preset
- `create_fantasy_dna()` - Fantasy CodeDNA preset
- `create_test_vectors(count, dimension)` - Generate test vectors
- `create_test_component(id)` - Create test game component
- `create_populated_game(count)` - Create game with N components
- `create_test_vector_index(count)` - Create index with N vectors

## Validation Framework

### Running Validation Tests

```rust
use arcadia::validation::*;

let dna = code_dna::CodeDNA::default_scifi();
let report = config_validator::ConfigValidator::validate_code_dna(&dna);

if report.has_errors() {
    println!("Errors: {}", report.error_count());
    println!("Warnings: {}", report.warning_count());
    println!("{}", report.summary());
}
```

### Validation Categories

1. **Component Validation** - Validates game elements
2. **Config Validation** - Validates configurations
3. **Data Integrity** - Validates data correctness
4. **Security Validation** - Validates security settings

## Continuous Integration

Tests run automatically on:
- Push to main/develop branches
- Pull requests
- Multiple Rust versions (stable, beta, nightly)

See `.github/workflows/test.yml` for CI configuration.

## Performance Benchmarks

### Current Benchmarks

#### Vector Index
- Insertion: 1000 vectors in ~50ms
- Search (10k index): ~2ms per query
- Cosine similarity (128d): ~1μs

#### AI Systems
- Neo-cortex decision: ~500ns
- Evolutionary evolve (100 traits): ~20μs
- Autopoetic update (100 components): ~10μs

#### Game Elements
- Component creation: ~100ns
- Game update (1000 components): ~500μs
- Entropy update (1000 objects): ~200μs

### Benchmark History

Track performance over time:
```bash
cargo bench -- --save-baseline before-optimization
# ... make changes ...
cargo bench -- --baseline before-optimization
```

## Writing New Tests

### Unit Test Template

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_name() {
        // Arrange
        let input = create_test_input();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected_value);
    }

    #[test]
    #[should_panic(expected = "error message")]
    fn test_error_condition() {
        // Test error handling
    }
}
```

### Integration Test Template

```rust
use arcadia::*;

#[test]
fn test_integration_scenario() {
    // Setup
    let component1 = setup_component1();
    let component2 = setup_component2();
    
    // Execute integration
    let result = component1.interact_with(component2);
    
    // Verify
    assert!(result.is_ok());
}
```

### Benchmark Template

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_function(c: &mut Criterion) {
    c.bench_function("function_name", |b| {
        let input = setup_input();
        b.iter(|| {
            function_to_benchmark(black_box(&input));
        });
    });
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
```

## Test Best Practices

1. **Isolation** - Tests should not depend on each other
2. **Repeatability** - Tests should produce same results every time
3. **Speed** - Unit tests should be fast (<1ms each)
4. **Clarity** - Test names should describe what they test
5. **Coverage** - Aim for high code coverage
6. **Edge Cases** - Test boundary conditions and error cases

## Troubleshooting

### Tests Failing

1. Check test output: `cargo test -- --nocapture`
2. Run specific test: `cargo test test_name -- --exact`
3. Check for race conditions in concurrent tests
4. Verify test fixtures are correct

### Benchmarks Unstable

1. Run on idle system
2. Increase iteration count
3. Use `--save-baseline` for comparison
4. Check for background processes

### Coverage Issues

1. Ensure tarpaulin is installed
2. Run with `--verbose` flag
3. Check for platform-specific code
4. Exclude test code from coverage

## Contributing

When adding new features:
1. Write unit tests first (TDD approach)
2. Add integration tests for workflows
3. Add benchmarks for performance-critical code
4. Update this README if needed
5. Ensure all tests pass before submitting PR

## Resources

- [Rust Testing Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Criterion.rs Benchmarking](https://github.com/bheisler/criterion.rs)
- [Tarpaulin Code Coverage](https://github.com/xd009642/tarpaulin)
