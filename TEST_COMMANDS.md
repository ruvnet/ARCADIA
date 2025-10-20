# ARCADIA Test Commands Quick Reference

## Basic Testing

```bash
# Run all tests
cargo test

# Run all tests with output
cargo test -- --nocapture

# Run all tests verbosely
cargo test --verbose

# Run tests in release mode (faster)
cargo test --release
```

## Unit Tests

```bash
# Run only library unit tests
cargo test --lib

# Run specific module tests
cargo test code_dna
cargo test vector_index
cargo test authentication
cargo test ai_systems
cargo test game_elements
cargo test validation

# Run specific test
cargo test test_code_dna_creation -- --exact
cargo test test_vector_insert -- --exact
```

## Integration Tests

```bash
# Run all integration tests
cargo test --test '*'

# Run specific integration test file
cargo test --test game_workflow_test
cargo test --test validation_integration_test

# Run specific integration test
cargo test test_complete_game_initialization
cargo test test_authentication_workflow
```

## Validation Tests

```bash
# Run all validation tests
cargo test validation

# Run specific validator tests
cargo test component_validator
cargo test config_validator
cargo test data_integrity
cargo test security_validator
```

## Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark suite
cargo bench --bench vector_index
cargo bench --bench ai_systems
cargo bench --bench code_dna
cargo bench --bench game_elements

# Run benchmarks with baseline
cargo bench -- --save-baseline main

# Compare against baseline
cargo bench -- --baseline main

# Generate HTML reports
cargo bench --bench vector_index -- --save-baseline production
```

## Code Coverage

```bash
# Install tarpaulin (first time only)
cargo install cargo-tarpaulin

# Generate coverage report (text)
cargo tarpaulin

# Generate HTML coverage report
cargo tarpaulin --out Html

# Generate coverage with specific output directory
cargo tarpaulin --out Html --output-dir target/coverage

# Verbose coverage
cargo tarpaulin --verbose --all-features --workspace

# Coverage for specific package
cargo tarpaulin --packages arcadia
```

## Code Quality

```bash
# Check code formatting
cargo fmt -- --check

# Format code
cargo fmt

# Run clippy
cargo clippy

# Run clippy with warnings as errors
cargo clippy -- -D warnings

# Run clippy for all targets
cargo clippy --all-targets --all-features -- -D warnings
```

## Build Commands

```bash
# Build project
cargo build

# Build in release mode
cargo build --release

# Build with all features
cargo build --all-features

# Build documentation
cargo doc

# Build and open documentation
cargo doc --open
```

## Test Runner Script

```bash
# Make executable (first time)
chmod +x run_tests.sh

# Run all tests
./run_tests.sh

# Run tests with benchmarks
./run_tests.sh --bench

# Run tests with coverage
./run_tests.sh --coverage
```

## Filtering Tests

```bash
# Run tests matching pattern
cargo test vector

# Run tests NOT matching pattern
cargo test --test '*' -- --skip slow

# Run ignored tests
cargo test -- --ignored

# Run all tests including ignored
cargo test -- --include-ignored
```

## Parallel Execution

```bash
# Run tests in parallel (default)
cargo test

# Run tests sequentially
cargo test -- --test-threads=1

# Run with specific thread count
cargo test -- --test-threads=4
```

## Documentation Tests

```bash
# Run doc tests
cargo test --doc

# Run doc tests for specific module
cargo test --doc code_dna
```

## Watch Mode (requires cargo-watch)

```bash
# Install cargo-watch
cargo install cargo-watch

# Watch and run tests on change
cargo watch -x test

# Watch and run specific tests
cargo watch -x "test vector_index"

# Watch with clear screen
cargo watch -c -x test
```

## Advanced Testing

```bash
# Run tests with backtraces
RUST_BACKTRACE=1 cargo test

# Run tests with full backtraces
RUST_BACKTRACE=full cargo test

# Run tests with specific log level
RUST_LOG=debug cargo test -- --nocapture

# Run tests with timings
cargo test -- --show-output

# Run tests and show execution time
cargo test -- --report-time
```

## CI/CD Simulation

```bash
# Simulate CI build
cargo build --verbose && \
cargo test --verbose && \
cargo test --test '*' --verbose && \
cargo clippy -- -D warnings && \
cargo fmt -- --check

# Full CI simulation with benchmarks
cargo build --all-features && \
cargo test --all-features && \
cargo bench --no-run && \
cargo doc --no-deps
```

## Performance Testing

```bash
# Run benchmarks and save results
cargo bench -- --save-baseline before-optimization

# Make code changes, then compare
cargo bench -- --baseline before-optimization

# List all benchmarks
cargo bench -- --list

# Run benchmark with specific iterations
cargo bench -- --sample-size 1000
```

## Test Organization

```bash
# List all tests
cargo test -- --list

# List integration tests
cargo test --test '*' -- --list

# Count tests
cargo test -- --list | wc -l

# Show test binary locations
cargo test --no-run --message-format=json
```

## Debugging Tests

```bash
# Run test with debugger (gdb)
rust-gdb --args target/debug/deps/arcadia-* test_name

# Run test with lldb
rust-lldb --args target/debug/deps/arcadia-* test_name

# Print test output always
cargo test -- --nocapture --test-threads=1
```

## Clean and Rebuild

```bash
# Clean build artifacts
cargo clean

# Clean and rebuild
cargo clean && cargo build

# Clean and run tests
cargo clean && cargo test
```

## Useful Combinations

```bash
# Fast iteration during development
cargo test --lib -- --nocapture

# Pre-commit checks
cargo fmt && cargo clippy && cargo test

# Full quality check
cargo fmt -- --check && \
cargo clippy -- -D warnings && \
cargo test --all-features && \
cargo bench --no-run

# Release readiness check
cargo build --release && \
cargo test --release && \
cargo bench && \
cargo tarpaulin --out Html
```

## Environment Variables

```bash
# Set test timeout
RUST_TEST_TIMEOUT=300 cargo test

# Run tests with specific features
cargo test --features "full"

# Disable default features
cargo test --no-default-features

# Test with all feature combinations
cargo test --all-features
```

---

**Tip**: Add commonly used commands to your shell aliases:

```bash
# Add to ~/.bashrc or ~/.zshrc
alias t='cargo test'
alias tb='cargo test --lib'
alias ti='cargo test --test "*"'
alias tw='cargo watch -x test'
alias tc='cargo tarpaulin --out Html'
alias bench='cargo bench'
```

---

**Last Updated**: 2025-10-20  
**For**: ARCADIA Test Suite
