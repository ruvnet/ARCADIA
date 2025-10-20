## ARCADIA Integration Guide

### Overview

This guide provides step-by-step instructions for integrating ARCADIA with game engines, particularly Unreal Engine 5.

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [Unreal Engine 5 Integration](#unreal-engine-5-integration)
3. [C++ Bridge](#c-bridge)
4. [Blueprint Integration](#blueprint-integration)
5. [Advanced Usage](#advanced-usage)
6. [Performance Tuning](#performance-tuning)
7. [Troubleshooting](#troubleshooting)

---

## Quick Start

### Installation

1. **Add ARCADIA to your project**:

```toml
# Cargo.toml
[dependencies]
arcadia = { path = "../ARCADIA" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

2. **Basic setup**:

```rust
use arcadia::{ArcadiaEngine, ArcadiaConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let config = ArcadiaConfig::default();

    // Initialize engine
    let engine = ArcadiaEngine::new(config).await?;
    engine.initialize().await?;

    // Your game logic here

    // Shutdown
    engine.shutdown().await?;

    Ok(())
}
```

---

## Unreal Engine 5 Integration

### Architecture Overview

```
┌─────────────────────────────────────────────┐
│         Unreal Engine 5                     │
│  ┌──────────────┐     ┌──────────────┐    │
│  │  Blueprints  │────▶│  C++ Wrapper │    │
│  └──────────────┘     └──────────────┘    │
└─────────────────────────────────────────────┘
                 │
                 ▼
         ┌──────────────┐
         │  FFI Bridge  │
         └──────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────┐
│         ARCADIA Engine (Rust)               │
│  ┌─────────┐         ┌─────────┐          │
│  │ VIVIAN  │         │ PARIS   │          │
│  └─────────┘         └─────────┘          │
└─────────────────────────────────────────────┘
```

### Prerequisites

- Unreal Engine 5.1 or higher
- Rust toolchain
- C++ compiler (MSVC on Windows, Clang on macOS/Linux)

### Step 1: Build ARCADIA as a C Library

Create `src/ffi.rs`:

```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(C)]
pub struct ArcadiaEngineHandle {
    ptr: *mut ArcadiaEngine,
}

#[no_mangle]
pub extern "C" fn arcadia_create_engine() -> *mut ArcadiaEngineHandle {
    // Create engine with default config
    let config = ArcadiaConfig::default();

    let rt = tokio::runtime::Runtime::new().unwrap();
    let engine = rt.block_on(async {
        ArcadiaEngine::new(config).await.unwrap()
    });

    let handle = Box::new(ArcadiaEngineHandle {
        ptr: Box::into_raw(Box::new(engine)),
    });

    Box::into_raw(handle)
}

#[no_mangle]
pub extern "C" fn arcadia_initialize(handle: *mut ArcadiaEngineHandle) -> bool {
    if handle.is_null() {
        return false;
    }

    let handle = unsafe { &*handle };
    let engine = unsafe { &*handle.ptr };

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        engine.initialize().await.is_ok()
    })
}

#[no_mangle]
pub extern "C" fn arcadia_run_cycle(handle: *mut ArcadiaEngineHandle) -> bool {
    if handle.is_null() {
        return false;
    }

    let handle = unsafe { &*handle };
    let engine = unsafe { &*handle.ptr };

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        engine.run_adaptive_cycle().await.is_ok()
    })
}

#[no_mangle]
pub extern "C" fn arcadia_add_vector(
    handle: *mut ArcadiaEngineHandle,
    id: *const c_char,
    vector: *const f32,
    dimension: usize,
) -> bool {
    if handle.is_null() || id.is_null() || vector.is_null() {
        return false;
    }

    let handle = unsafe { &*handle };
    let engine = unsafe { &*handle.ptr };
    let id_str = unsafe { CStr::from_ptr(id).to_str().unwrap() };
    let vector_slice = unsafe { std::slice::from_raw_parts(vector, dimension) };

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let vivian = engine.vivian().read().await;
        let vector_index = vivian.vector_index();
        let mut vi_guard = vector_index.write().await;

        let embedding = VectorEmbedding {
            id: id_str.to_string(),
            vector: vector_slice.to_vec(),
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now().timestamp_millis(),
        };

        vi_guard.add_embedding("ue5_assets", embedding).await.is_ok()
    })
}

#[no_mangle]
pub extern "C" fn arcadia_search_vectors(
    handle: *mut ArcadiaEngineHandle,
    query: *const f32,
    dimension: usize,
    k: usize,
    results_out: *mut *mut c_char,
) -> i32 {
    // Implementation for vector search
    // Returns number of results found
    0
}

#[no_mangle]
pub extern "C" fn arcadia_destroy_engine(handle: *mut ArcadiaEngineHandle) {
    if !handle.is_null() {
        unsafe {
            let handle = Box::from_raw(handle);
            let _engine = Box::from_raw(handle.ptr);
            // Engine will be dropped here
        }
    }
}
```

### Step 2: Build as Dynamic Library

Update `Cargo.toml`:

```toml
[lib]
name = "arcadia"
crate-type = ["cdylib", "rlib"]
```

Build:

```bash
cargo build --release
```

This creates:
- Windows: `target/release/arcadia.dll`
- macOS: `target/release/libarcadia.dylib`
- Linux: `target/release/libarcadia.so`

### Step 3: Create Unreal Engine C++ Wrapper

Create `Source/YourGame/ArcadiaWrapper.h`:

```cpp
#pragma once

#include "CoreMinimal.h"
#include "UObject/NoExportTypes.h"
#include "ArcadiaWrapper.generated.h"

// Forward declarations
struct ArcadiaEngineHandle;

UCLASS(BlueprintType)
class YOURGAME_API UArcadiaEngine : public UObject
{
    GENERATED_BODY()

public:
    UArcadiaEngine();
    ~UArcadiaEngine();

    UFUNCTION(BlueprintCallable, Category = "ARCADIA")
    bool Initialize();

    UFUNCTION(BlueprintCallable, Category = "ARCADIA")
    bool RunAdaptiveCycle();

    UFUNCTION(BlueprintCallable, Category = "ARCADIA")
    bool AddVectorEmbedding(const FString& ID, const TArray<float>& Vector);

    UFUNCTION(BlueprintCallable, Category = "ARCADIA")
    TArray<FString> SearchSimilarVectors(const TArray<float>& Query, int32 K);

    UFUNCTION(BlueprintCallable, Category = "ARCADIA")
    void Shutdown();

private:
    ArcadiaEngineHandle* EngineHandle;
    bool bIsInitialized;

    // DLL handle
    void* DLLHandle;

    // Function pointers
    typedef ArcadiaEngineHandle* (*CreateEngineFunc)();
    typedef bool (*InitializeFunc)(ArcadiaEngineHandle*);
    typedef bool (*RunCycleFunc)(ArcadiaEngineHandle*);
    typedef bool (*AddVectorFunc)(ArcadiaEngineHandle*, const char*, const float*, size_t);
    typedef int (*SearchFunc)(ArcadiaEngineHandle*, const float*, size_t, size_t, char**);
    typedef void (*DestroyFunc)(ArcadiaEngineHandle*);

    CreateEngineFunc CreateEngine;
    InitializeFunc InitializeEngine;
    RunCycleFunc RunCycle;
    AddVectorFunc AddVector;
    SearchFunc SearchVectors;
    DestroyFunc DestroyEngine;

    bool LoadDLL();
    void UnloadDLL();
};
```

Create `Source/YourGame/ArcadiaWrapper.cpp`:

```cpp
#include "ArcadiaWrapper.h"
#include "HAL/PlatformProcess.h"

UArcadiaEngine::UArcadiaEngine()
    : EngineHandle(nullptr)
    , bIsInitialized(false)
    , DLLHandle(nullptr)
{
    LoadDLL();
}

UArcadiaEngine::~UArcadiaEngine()
{
    Shutdown();
    UnloadDLL();
}

bool UArcadiaEngine::LoadDLL()
{
#if PLATFORM_WINDOWS
    FString DLLPath = FPaths::Combine(FPaths::ProjectPluginsDir(), TEXT("ARCADIA/Binaries/Win64/arcadia.dll"));
#elif PLATFORM_MAC
    FString DLLPath = FPaths::Combine(FPaths::ProjectPluginsDir(), TEXT("ARCADIA/Binaries/Mac/libarcadia.dylib"));
#elif PLATFORM_LINUX
    FString DLLPath = FPaths::Combine(FPaths::ProjectPluginsDir(), TEXT("ARCADIA/Binaries/Linux/libarcadia.so"));
#endif

    DLLHandle = FPlatformProcess::GetDllHandle(*DLLPath);

    if (!DLLHandle)
    {
        UE_LOG(LogTemp, Error, TEXT("Failed to load ARCADIA DLL"));
        return false;
    }

    // Load function pointers
    CreateEngine = (CreateEngineFunc)FPlatformProcess::GetDllExport(DLLHandle, TEXT("arcadia_create_engine"));
    InitializeEngine = (InitializeFunc)FPlatformProcess::GetDllExport(DLLHandle, TEXT("arcadia_initialize"));
    RunCycle = (RunCycleFunc)FPlatformProcess::GetDllExport(DLLHandle, TEXT("arcadia_run_cycle"));
    AddVector = (AddVectorFunc)FPlatformProcess::GetDllExport(DLLHandle, TEXT("arcadia_add_vector"));
    SearchVectors = (SearchFunc)FPlatformProcess::GetDllExport(DLLHandle, TEXT("arcadia_search_vectors"));
    DestroyEngine = (DestroyFunc)FPlatformProcess::GetDllExport(DLLHandle, TEXT("arcadia_destroy_engine"));

    return CreateEngine && InitializeEngine && RunCycle && DestroyEngine;
}

void UArcadiaEngine::UnloadDLL()
{
    if (DLLHandle)
    {
        FPlatformProcess::FreeDllHandle(DLLHandle);
        DLLHandle = nullptr;
    }
}

bool UArcadiaEngine::Initialize()
{
    if (bIsInitialized)
    {
        return true;
    }

    if (!CreateEngine)
    {
        UE_LOG(LogTemp, Error, TEXT("ARCADIA DLL not loaded"));
        return false;
    }

    EngineHandle = CreateEngine();

    if (!EngineHandle)
    {
        UE_LOG(LogTemp, Error, TEXT("Failed to create ARCADIA engine"));
        return false;
    }

    bIsInitialized = InitializeEngine(EngineHandle);

    if (bIsInitialized)
    {
        UE_LOG(LogTemp, Log, TEXT("ARCADIA engine initialized successfully"));
    }

    return bIsInitialized;
}

bool UArcadiaEngine::RunAdaptiveCycle()
{
    if (!bIsInitialized || !EngineHandle)
    {
        return false;
    }

    return RunCycle(EngineHandle);
}

bool UArcadiaEngine::AddVectorEmbedding(const FString& ID, const TArray<float>& Vector)
{
    if (!bIsInitialized || !EngineHandle)
    {
        return false;
    }

    return AddVector(EngineHandle, TCHAR_TO_ANSI(*ID), Vector.GetData(), Vector.Num());
}

TArray<FString> UArcadiaEngine::SearchSimilarVectors(const TArray<float>& Query, int32 K)
{
    TArray<FString> Results;

    if (!bIsInitialized || !EngineHandle)
    {
        return Results;
    }

    // Implementation for search
    // ...

    return Results;
}

void UArcadiaEngine::Shutdown()
{
    if (EngineHandle && DestroyEngine)
    {
        DestroyEngine(EngineHandle);
        EngineHandle = nullptr;
    }

    bIsInitialized = false;
}
```

---

## Blueprint Integration

### Creating Blueprint Nodes

1. The C++ wrapper above automatically creates Blueprint nodes
2. In your Blueprint, add an "ARCADIA Engine" variable
3. Call "Initialize" in BeginPlay
4. Call "Run Adaptive Cycle" in your game loop
5. Use "Add Vector Embedding" for game assets
6. Use "Search Similar Vectors" for content generation

### Example Blueprint Flow

```
BeginPlay
  ↓
Create ARCADIA Engine
  ↓
Initialize
  ↓
Event Tick
  ↓
Run Adaptive Cycle (every N frames)
  ↓
[Your game logic]
```

---

## Advanced Usage

### Asynchronous Operations

For better performance, run ARCADIA operations asynchronously:

```cpp
UFUNCTION(BlueprintCallable, Category = "ARCADIA", meta = (Latent, LatentInfo = "LatentInfo"))
void RunAdaptiveCycleAsync(FLatentActionInfo LatentInfo);
```

### Custom Feedback Integration

```cpp
UFUNCTION(BlueprintCallable, Category = "ARCADIA")
bool SubmitPlayerFeedback(float Engagement, float Difficulty, float Satisfaction);
```

---

## Performance Tuning

### Recommended Settings

| Setting | Development | Production |
|---------|------------|------------|
| Cache Size | 256MB | 1024MB |
| Vector Dimension | 256 | 512-1024 |
| Batch Size | 32 | 128 |
| Update Frequency | Every frame | Every 10 frames |

### Profiling

Use Unreal's profiling tools:
```cpp
SCOPE_CYCLE_COUNTER(STAT_ArcadiaAdaptiveCycle);
```

---

## Troubleshooting

### Common Issues

**Issue**: DLL not loading
- **Solution**: Check DLL path, ensure correct architecture (x64)

**Issue**: Crash on initialization
- **Solution**: Verify Rust dependencies are built with correct target

**Issue**: Slow performance
- **Solution**: Run adaptive cycle less frequently, increase batch sizes

---

## Next Steps

1. Review example projects in `examples/`
2. Join the ARCADIA community Discord
3. Check out advanced tutorials

---

**Document Version**: 1.0
**Last Updated**: 2025-10-20
