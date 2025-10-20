.PHONY: help build build-wasm build-native test clean install-deps

help:
	@echo "ARCADIA AgentDB Integration - Build Commands"
	@echo ""
	@echo "make build          - Build everything (native + WASM)"
	@echo "make build-wasm     - Build WASM packages"
	@echo "make build-native   - Build native Rust library"
	@echo "make test           - Run all tests"
	@echo "make test-wasm      - Run WASM tests"
	@echo "make clean          - Clean build artifacts"
	@echo "make install-deps   - Install dependencies"
	@echo "make examples       - Build and run examples"

install-deps:
	@echo "Installing dependencies..."
	cargo install wasm-pack
	rustup target add wasm32-unknown-unknown
	@echo "Dependencies installed!"

build: build-native build-wasm
	@echo "Build complete!"

build-native:
	@echo "Building native Rust library..."
	cargo build --release
	@echo "Native build complete!"

build-wasm:
	@echo "Building WASM packages..."
	wasm-pack build --target web --out-dir pkg
	wasm-pack build --target nodejs --out-dir pkg-node
	wasm-pack build --target bundler --out-dir pkg-bundler
	@echo "WASM build complete!"

test:
	@echo "Running Rust tests..."
	cargo test
	@echo "Tests complete!"

test-wasm:
	@echo "Running WASM tests..."
	wasm-pack test --headless --firefox
	@echo "WASM tests complete!"

clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf pkg pkg-node pkg-bundler
	rm -rf target
	@echo "Clean complete!"

examples:
	@echo "Building examples..."
	cargo build --examples
	@echo "Examples built! Run with: cargo run --example <example_name>"

bench:
	@echo "Running benchmarks..."
	cargo bench
	@echo "Benchmarks complete!"

fmt:
	@echo "Formatting code..."
	cargo fmt
	@echo "Format complete!"

lint:
	@echo "Running linter..."
	cargo clippy -- -D warnings
	@echo "Lint complete!"

check:
	@echo "Checking code..."
	cargo check
	@echo "Check complete!"

doc:
	@echo "Building documentation..."
	cargo doc --no-deps --open
	@echo "Documentation complete!"

all: fmt lint test build
	@echo "All tasks complete!"
