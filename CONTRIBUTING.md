# Contributing to ARCADIA

Thank you for your interest in contributing to ARCADIA! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Setup](#development-setup)
4. [Development Workflow](#development-workflow)
5. [Coding Standards](#coding-standards)
6. [Testing Guidelines](#testing-guidelines)
7. [Performance Considerations](#performance-considerations)
8. [Documentation](#documentation)
9. [Pull Request Process](#pull-request-process)
10. [Release Process](#release-process)

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inspiring community for all. Please be respectful and constructive in all interactions.

### Our Standards

- **Be respectful**: Treat everyone with respect and kindness
- **Be collaborative**: Work together to improve the project
- **Be professional**: Maintain professionalism in all communications
- **Be constructive**: Provide helpful feedback and suggestions

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Git
- Familiarity with async/await in Rust
- Understanding of game engine architecture (helpful but not required)

### Finding Work

1. Check the [Issues](https://github.com/yourusername/arcadia/issues) page
2. Look for issues labeled `good first issue` or `help wanted`
3. Comment on an issue to let others know you're working on it
4. Ask questions if anything is unclear

## Development Setup

### 1. Fork and Clone

```bash
# Fork the repository on GitHub
# Then clone your fork
git clone https://github.com/YOUR_USERNAME/arcadia.git
cd arcadia

# Add upstream remote
git remote add upstream https://github.com/yourusername/arcadia.git
```

### 2. Install Dependencies

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install additional tools
cargo install cargo-watch
cargo install cargo-expand
cargo install cargo-audit
```

### 3. Build the Project

```bash
# Build in debug mode
cargo build

# Build in release mode with all features
cargo build --release --all-features

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### 4. Set Up Environment

Create a `.env` file for local development:

```bash
OPENAI_API_KEY=sk-your-key-here
QDRANT_URL=http://localhost:6334
RUST_LOG=arcadia=debug
```

## Development Workflow

### Branching Strategy

- `main`: Production-ready code
- `develop`: Integration branch for features
- `feature/*`: Feature branches
- `fix/*`: Bug fix branches
- `perf/*`: Performance improvement branches

### Creating a Feature Branch

```bash
# Update your fork
git checkout main
git pull upstream main

# Create feature branch
git checkout -b feature/your-feature-name
```

### Making Changes

```bash
# Make your changes
# ...

# Format code
cargo fmt

# Check lints
cargo clippy -- -D warnings

# Run tests
cargo test

# Commit changes
git add .
git commit -m "feat: add awesome feature"
```

### Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
```
feat(vector-index): add batch embedding support

Implements batch embedding generation to reduce API calls
and improve performance.

Closes #123
```

```
perf(cache): optimize cache eviction algorithm

Reduces cache eviction overhead by 40% using a more
efficient LRU implementation.
```

## Coding Standards

### Rust Style Guide

Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/):

1. **Use `rustfmt`**:
   ```bash
   cargo fmt
   ```

2. **Use `clippy`**:
   ```bash
   cargo clippy -- -D warnings
   ```

3. **Naming Conventions**:
   - Types: `PascalCase`
   - Functions: `snake_case`
   - Constants: `SCREAMING_SNAKE_CASE`
   - Modules: `snake_case`

### Code Organization

```rust
// Module structure
pub mod module_name {
    // Imports
    use std::collections::HashMap;

    // Constants
    const MAX_SIZE: usize = 1000;

    // Types
    pub struct MyStruct {
        field: String,
    }

    // Implementation
    impl MyStruct {
        pub fn new() -> Self {
            // ...
        }
    }

    // Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_something() {
            // ...
        }
    }
}
```

### Documentation

All public APIs must be documented:

```rust
/// Generates a text embedding using the configured model.
///
/// # Arguments
///
/// * `text` - The input text to embed
///
/// # Returns
///
/// A vector of f32 values representing the embedding
///
/// # Errors
///
/// Returns `VectorIndexError` if the API call fails
///
/// # Examples
///
/// ```
/// use arcadia::vector_index::VectorIndex;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let index = VectorIndex::new(config).await?;
/// let embedding = index.embed_text("Hello, world!").await?;
/// # Ok(())
/// # }
/// ```
pub async fn embed_text(&self, text: &str) -> Result<Vec<f32>, VectorIndexError> {
    // Implementation
}
```

### Error Handling

Use custom error types with `thiserror`:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Operation failed: {0}")]
    OperationFailed(#[from] std::io::Error),
}
```

## Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let result = function_to_test();
        assert_eq!(result, expected_value);
    }

    #[tokio::test]
    async fn test_async_functionality() {
        let result = async_function().await.unwrap();
        assert!(result.is_valid());
    }
}
```

### Integration Tests

Create files in `tests/`:

```rust
// tests/integration_test.rs
use arcadia::vector_index::{VectorIndex, VectorIndexConfig};

#[tokio::test]
async fn test_full_workflow() {
    let config = VectorIndexConfig::default();
    let index = VectorIndex::new(config).await.unwrap();

    // Test complete workflow
}
```

### Property-Based Testing

Use `proptest`:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_property(s in "\\PC*") {
        // Test property holds for all inputs
        assert!(invariant(&s));
    }
}
```

### Test Coverage

Aim for >80% test coverage:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage
```

## Performance Considerations

### Benchmarking

Add benchmarks for performance-critical code:

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_function(c: &mut Criterion) {
    c.bench_function("my_function", |b| {
        b.iter(|| {
            my_function(black_box(input))
        });
    });
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
```

Run benchmarks:

```bash
cargo bench
```

### Performance Checklist

- [ ] Use appropriate data structures (Vec vs SmallVec, HashMap vs DashMap)
- [ ] Avoid unnecessary allocations
- [ ] Use references instead of cloning when possible
- [ ] Consider using `Cow` for conditional cloning
- [ ] Profile code with `cargo flamegraph`
- [ ] Check memory usage with valgrind/massif
- [ ] Use SIMD when applicable
- [ ] Implement caching for expensive operations
- [ ] Use async/await for I/O operations
- [ ] Leverage Rayon for data parallelism

### Profiling

```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph --bench my_benchmark

# Install cargo-watch for continuous testing
cargo install cargo-watch
cargo watch -x test
```

## Documentation

### Code Documentation

- Document all public APIs
- Include examples in documentation
- Explain non-obvious behavior
- Document safety requirements
- Add links to related functionality

### Markdown Documentation

- Keep language clear and concise
- Use code examples liberally
- Include diagrams where helpful
- Link to related documents
- Keep documentation up-to-date with code changes

### Building Documentation

```bash
# Build API documentation
cargo doc --no-deps --all-features

# Build and open in browser
cargo doc --no-deps --all-features --open
```

## Pull Request Process

### Before Submitting

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Benchmarks show no regressions
- [ ] Documentation is updated
- [ ] Commit messages follow conventions
- [ ] Code is formatted with `rustfmt`
- [ ] No clippy warnings
- [ ] Added tests for new functionality

### Submitting a PR

1. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. Create a Pull Request on GitHub

3. Fill out the PR template:
   ```markdown
   ## Description
   Brief description of changes

   ## Type of Change
   - [ ] Bug fix
   - [ ] New feature
   - [ ] Performance improvement
   - [ ] Documentation update

   ## Testing
   How was this tested?

   ## Checklist
   - [ ] Tests added/updated
   - [ ] Documentation updated
   - [ ] No performance regressions
   ```

4. Wait for review and address feedback

### Review Process

1. Automated checks must pass (CI/CD)
2. At least one maintainer approval required
3. All conversations must be resolved
4. Documentation must be complete

### After Approval

Maintainers will merge your PR. Thank you for contributing!

## Release Process

### Version Numbering

Follow [Semantic Versioning](https://semver.org/):

- MAJOR: Breaking changes
- MINOR: New features (backward compatible)
- PATCH: Bug fixes

### Release Checklist

- [ ] Update version in `Cargo.toml`
- [ ] Update CHANGELOG.md
- [ ] Run full test suite
- [ ] Run benchmarks
- [ ] Update documentation
- [ ] Create git tag
- [ ] Publish to crates.io

## Getting Help

- **Discord**: [Join our server](#)
- **GitHub Discussions**: [Ask questions](https://github.com/yourusername/arcadia/discussions)
- **Email**: arcadia-dev@example.com

## Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md
- Release notes
- Project website

Thank you for contributing to ARCADIA!
