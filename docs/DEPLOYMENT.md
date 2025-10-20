# ARCADIA Deployment Guide

## Overview

ARCADIA can be deployed in multiple environments including local development, cloud sandboxes, and production game engines.

## Local Development

### Prerequisites

```bash
# Rust 1.75+
rustup update

# Dependencies (Linux)
sudo apt-get install build-essential pkg-config libssl-dev

# Dependencies (macOS)
brew install openssl pkg-config
```

### Build and Test

```bash
# Clone repository
git clone https://github.com/ruvnet/ARCADIA.git
cd ARCADIA

# Build
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench

# Build documentation
cargo doc --open
```

## Cloud Sandbox Deployment

### E2B (Recommended for Testing)

E2B provides cloud-based code execution environments with full network access.

#### Option 1: E2B CLI

```bash
# Install E2B CLI
npm install -g @e2b/cli

# Authenticate
e2b auth login

# Create Rust sandbox
e2b sandbox create --template rust

# Upload ARCADIA
e2b sandbox cp ./ARCADIA sandbox_id:/workspace/

# Build and test
e2b sandbox exec sandbox_id "cd /workspace/ARCADIA && cargo build --release"
e2b sandbox exec sandbox_id "cargo test --manifest-path /workspace/ARCADIA/Cargo.toml"
e2b sandbox exec sandbox_id "cargo bench --manifest-path /workspace/ARCADIA/Cargo.toml"

# Run examples
e2b sandbox exec sandbox_id "cargo run --manifest-path /workspace/ARCADIA/Cargo.toml --example goap_npc_behavior"
```

#### Option 2: E2B Python SDK

```python
from e2b import Sandbox

# Create sandbox
sandbox = Sandbox(template="rust")

try:
    # Upload ARCADIA
    sandbox.filesystem.write("/workspace/ARCADIA", local_path="./ARCADIA")
    
    # Build
    build_result = sandbox.process.start(
        "cd /workspace/ARCADIA && cargo build --release"
    )
    print("Build output:", build_result.stdout)
    
    # Run tests
    test_result = sandbox.process.start(
        "cd /workspace/ARCADIA && cargo test"
    )
    print("Test results:", test_result.stdout)
    
    # Run benchmarks
    bench_result = sandbox.process.start(
        "cd /workspace/ARCADIA && cargo bench"
    )
    print("Benchmark results:", bench_result.stdout)
    
finally:
    sandbox.close()
```

#### Option 3: E2B Code Interpreter

```python
from e2b import CodeInterpreter

with CodeInterpreter() as interpreter:
    # Upload project
    interpreter.upload("./ARCADIA", "/home/user/ARCADIA")
    
    # Build and test in one command
    result = interpreter.process.start("""
        cd /home/user/ARCADIA
        cargo build --release 2>&1
        cargo test 2>&1
        cargo bench --no-run 2>&1
        cargo run --example goap_npc_behavior 2>&1
    """)
    
    print(result.stdout)
```

### Flow-Nexus Sandboxes (Advanced)

If you have Flow-Nexus access:

```bash
# Authenticate
npx flow-nexus@latest login

# Create sandbox
npx flow-nexus@latest sandbox create \
    --name "arcadia-test" \
    --runtime rust \
    --memory 4096

# Upload project
npx flow-nexus@latest sandbox upload \
    --sandbox-id <id> \
    --source ./ARCADIA \
    --destination /workspace/ARCADIA

# Execute build and tests
npx flow-nexus@latest sandbox exec \
    --sandbox-id <id> \
    --command "cd /workspace/ARCADIA && cargo build --release && cargo test && cargo bench"

# Stream logs
npx flow-nexus@latest sandbox logs --sandbox-id <id> --follow
```

## Docker Deployment

### Build Docker Image

```dockerfile
# Dockerfile
FROM rust:1.75 as builder

WORKDIR /usr/src/arcadia
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/arcadia/target/release/libarcadia.rlib /usr/local/lib/
COPY --from=builder /usr/src/arcadia/target/release/examples/* /usr/local/bin/

CMD ["/usr/local/bin/goap_npc_behavior"]
```

```bash
# Build image
docker build -t arcadia:latest .

# Run tests
docker run --rm arcadia:latest cargo test

# Run benchmarks
docker run --rm arcadia:latest cargo bench

# Run example
docker run --rm arcadia:latest /usr/local/bin/goap_npc_behavior
```

## Game Engine Integration

### Unreal Engine 5

Using Rust FFI:

```rust
// arcadia_ue5.rs
use arcadia::ai::IntegratedAISystem;
use std::ffi::{CString, c_char};

#[no_mangle]
pub extern "C" fn arcadia_create_ai(name: *const c_char) -> *mut IntegratedAISystem {
    let name_str = unsafe { std::ffi::CStr::from_ptr(name) }
        .to_str()
        .unwrap_or("UnnamedNPC");
    
    let system = IntegratedAISystem::new(
        uuid::Uuid::new_v4(),
        name_str.to_string()
    );
    
    Box::into_raw(Box::new(system))
}

#[no_mangle]
pub extern "C" fn arcadia_plan_goap(
    system: *mut IntegratedAISystem,
    goal_name: *const c_char
) -> *const c_char {
    // GOAP planning logic...
}
```

Build as dynamic library:

```bash
cargo build --release --lib
# Produces: target/release/libarcadia.so (Linux) or .dylib (macOS)
```

Link in UE5 C++ project.

### Unity

Using C# interop:

```csharp
using System.Runtime.InteropServices;

public class ArcadiaAI : MonoBehaviour 
{
    [DllImport("arcadia")]
    private static extern IntPtr arcadia_create_ai(string name);
    
    [DllImport("arcadia")]
    private static extern string arcadia_plan_goap(IntPtr system, string goal);
    
    private IntPtr aiSystem;
    
    void Start() {
        aiSystem = arcadia_create_ai("UnityNPC");
    }
}
```

### Godot

Using GDNative:

```rust
// arcadia_godot.rs
use gdnative::prelude::*;
use arcadia::ai::IntegratedAISystem;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct ArcadiaAI {
    system: IntegratedAISystem,
}

#[methods]
impl ArcadiaAI {
    fn new(_owner: &Node) -> Self {
        ArcadiaAI {
            system: IntegratedAISystem::default()
        }
    }
    
    #[export]
    fn plan_goap(&self, _owner: &Node, goal: String) -> GodotString {
        // GOAP planning...
        GodotString::from("plan")
    }
}
```

## WebAssembly Deployment

For browser-based games:

```bash
# Install wasm-pack
cargo install wasm-pack

# Build for web
wasm-pack build --target web --out-dir pkg

# Use in JavaScript
```

```javascript
import init, { IntegratedAISystem } from './pkg/arcadia.js';

async function main() {
    await init();
    const ai = new IntegratedAISystem("WebNPC");
    const plan = ai.plan_goap("defeat_enemy");
    console.log(plan);
}
```

## Production Deployment

### Performance Optimization

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = 'abort'
strip = true
```

### Monitoring

Use Prometheus metrics (already integrated):

```rust
use arcadia::metrics::MetricsCollector;

let metrics = MetricsCollector::new();
// Metrics automatically collected and exposed on :9090/metrics
```

### Logging

```bash
# Run with logging
RUST_LOG=arcadia=info cargo run --release

# Structured logging
RUST_LOG=arcadia=debug,arcadia::ai::goap=trace cargo run
```

## CI/CD Pipeline

### GitHub Actions

```yaml
name: ARCADIA CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Build
        run: cargo build --release
      
      - name: Test
        run: cargo test --all
      
      - name: Benchmark
        run: cargo bench --no-fail-fast
      
      - name: Publish to crates.io
        if: github.ref == 'refs/heads/main'
        run: cargo publish --token ${{ secrets.CARGO_TOKEN }}
```

## Troubleshooting

### Network Restrictions

If crates.io is blocked:

1. **Use E2B Sandbox** (recommended - has full network access)
2. **Vendor dependencies**: `cargo vendor` and commit to repo
3. **Use mirror**: Set up crates.io mirror in `.cargo/config.toml`
4. **Offline build**: Download dependencies on unrestricted machine, copy

### Build Errors

```bash
# Clean build
cargo clean && cargo build

# Update dependencies
cargo update

# Check for issues
cargo check
```

### Performance Issues

```bash
# Profile with perf
cargo build --release
perf record ./target/release/examples/goap_npc_behavior
perf report

# Memory profiling
valgrind --tool=massif ./target/release/examples/goap_npc_behavior
```

## Support

- **Documentation**: https://docs.rs/arcadia
- **GitHub Issues**: https://github.com/ruvnet/ARCADIA/issues
- **E2B Support**: https://e2b.dev/docs
- **Flow-Nexus**: https://flow-nexus.ruv.io

---

**Last Updated**: 2024
**ARCADIA Version**: 0.1.0
**Tested Platforms**: Linux, macOS, Windows, E2B, Docker
