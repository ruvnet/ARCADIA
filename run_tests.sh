#!/bin/bash

# ARCADIA Test Runner Script
# Comprehensive test execution and reporting

set -e

echo "========================================="
echo "ARCADIA Test & Validation Suite"
echo "========================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    print_error "Cargo not found. Please install Rust toolchain."
    exit 1
fi

print_status "Cargo found: $(cargo --version)"
echo ""

# Build project
echo "Building project..."
if cargo build --all-features; then
    print_status "Build successful"
else
    print_error "Build failed"
    exit 1
fi
echo ""

# Run unit tests
echo "Running unit tests..."
if cargo test --lib; then
    print_status "Unit tests passed"
else
    print_error "Unit tests failed"
    exit 1
fi
echo ""

# Run integration tests
echo "Running integration tests..."
if cargo test --test '*'; then
    print_status "Integration tests passed"
else
    print_error "Integration tests failed"
    exit 1
fi
echo ""

# Run doc tests
echo "Running documentation tests..."
if cargo test --doc; then
    print_status "Documentation tests passed"
else
    print_warning "Documentation tests failed or not found"
fi
echo ""

# Check formatting
echo "Checking code formatting..."
if cargo fmt -- --check; then
    print_status "Code formatting is correct"
else
    print_warning "Code needs formatting. Run: cargo fmt"
fi
echo ""

# Run clippy
echo "Running Clippy lints..."
if cargo clippy -- -D warnings; then
    print_status "Clippy checks passed"
else
    print_warning "Clippy found issues"
fi
echo ""

# Run benchmarks (optional, as they take time)
if [ "$1" == "--bench" ]; then
    echo "Running benchmarks..."
    cargo bench
    print_status "Benchmarks completed"
    echo ""
fi

# Generate coverage (optional)
if [ "$1" == "--coverage" ]; then
    echo "Generating code coverage..."
    if command -v cargo-tarpaulin &> /dev/null; then
        cargo tarpaulin --out Html --output-dir target/coverage
        print_status "Coverage report generated in target/coverage/"
    else
        print_warning "cargo-tarpaulin not installed. Install with: cargo install cargo-tarpaulin"
    fi
    echo ""
fi

echo "========================================="
print_status "All tests completed successfully!"
echo "========================================="
echo ""
echo "Usage:"
echo "  ./run_tests.sh              - Run all tests"
echo "  ./run_tests.sh --bench      - Run tests and benchmarks"
echo "  ./run_tests.sh --coverage   - Run tests and generate coverage"
