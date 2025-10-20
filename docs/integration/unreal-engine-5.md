# Unreal Engine 5 Integration Guide

This guide walks you through integrating ARCADIA with Unreal Engine 5 to create AI-driven, procedurally generated, and emotionally adaptive games.

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Integration Architecture](#integration-architecture)
4. [Setup Instructions](#setup-instructions)
5. [API Integration](#api-integration)
6. [Example Use Cases](#example-use-cases)
7. [Performance Optimization](#performance-optimization)
8. [Troubleshooting](#troubleshooting)

## Overview

ARCADIA integrates with Unreal Engine 5 through:
- **Rust Plugin**: C++ FFI bindings for ARCADIA
- **Blueprint Nodes**: Visual scripting support
- **C++ API**: Direct integration for advanced users
- **HTTP API**: REST endpoints for loose coupling

## Prerequisites

### Software Requirements

- Unreal Engine 5.1 or higher
- Rust 1.70+
- Visual Studio 2022 (Windows) or Xcode (macOS)
- CMake 3.20+

### ARCADIA Setup

```bash
# Clone ARCADIA
git clone https://github.com/yourusername/arcadia.git
cd arcadia

# Build the library
cargo build --release --features unreal-integration

# Build C++ bindings
cargo build --release -p arcadia-unreal-bindings
```

## Integration Architecture

```
┌──────────────────────────────────────────────────┐
│         Unreal Engine 5 Game                      │
│                                                    │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐ │
│  │ Blueprints │  │  C++ Code  │  │ Game Logic │ │
│  └─────┬──────┘  └─────┬──────┘  └─────┬──────┘ │
│        │               │               │         │
│        └───────────────┴───────────────┘         │
│                        │                         │
│               ┌────────▼────────┐                │
│               │ ARCADIA Plugin  │                │
│               │  (C++ Wrapper)  │                │
│               └────────┬────────┘                │
│                        │                         │
│               ┌────────▼────────┐                │
│               │   FFI Layer     │                │
│               │  (Rust <-> C++) │                │
│               └────────┬────────┘                │
└────────────────────────┼──────────────────────────┘
                         │
                ┌────────▼────────┐
                │ ARCADIA Engine  │
                │   (Rust Core)   │
                └─────────────────┘
```

## Setup Instructions

### Step 1: Create Unreal Project

1. Launch Unreal Engine 5
2. Create a new C++ project
3. Name it (e.g., "ARCADIAGame")
4. Choose project location

### Step 2: Add ARCADIA Plugin

Create plugin structure:

```
ARCADIAGame/
├── Plugins/
│   └── ARCADIA/
│       ├── ARCADIA.uplugin
│       ├── Source/
│       │   └── ARCADIA/
│       │       ├── ARCADIA.Build.cs
│       │       ├── Private/
│       │       │   ├── ARCADIAModule.cpp
│       │       │   └── ARCADIAFunctions.cpp
│       │       └── Public/
│       │           ├── ARCADIAModule.h
│       │           └── ARCADIAFunctions.h
│       └── Binaries/
│           └── Win64/
│               └── arcadia.dll
```

### Step 3: Configure Plugin Descriptor

**ARCADIA.uplugin:**

```json
{
    "FileVersion": 3,
    "Version": 1,
    "VersionName": "0.1.0",
    "FriendlyName": "ARCADIA",
    "Description": "AI-driven game engine integration",
    "Category": "AI",
    "CreatedBy": "Your Name",
    "CreatedByURL": "https://github.com/yourusername/arcadia",
    "DocsURL": "",
    "MarketplaceURL": "",
    "SupportURL": "",
    "CanContainContent": true,
    "IsBetaVersion": true,
    "Installed": false,
    "Modules": [
        {
            "Name": "ARCADIA",
            "Type": "Runtime",
            "LoadingPhase": "PreDefault",
            "PlatformAllowList": [
                "Win64",
                "Linux",
                "Mac"
            ]
        }
    ]
}
```

### Step 4: Build Configuration

**ARCADIA.Build.cs:**

```csharp
using UnrealBuildTool;
using System.IO;

public class ARCADIA : ModuleRules
{
    public ARCADIA(ReadOnlyTargetRules Target) : base(Target)
    {
        PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;

        PublicDependencyModuleNames.AddRange(new string[] {
            "Core",
            "CoreUObject",
            "Engine",
            "HTTP",
            "Json",
            "JsonUtilities"
        });

        PrivateDependencyModuleNames.AddRange(new string[] {
            "Slate",
            "SlateCore"
        });

        // Add ARCADIA library
        string LibPath = Path.Combine(ModuleDirectory, "../../Binaries/Win64");
        PublicAdditionalLibraries.Add(Path.Combine(LibPath, "arcadia.lib"));

        // Add runtime dependencies
        RuntimeDependencies.Add(Path.Combine(LibPath, "arcadia.dll"));

        // Include paths
        PublicIncludePaths.Add(Path.Combine(ModuleDirectory, "Public"));
        PrivateIncludePaths.Add(Path.Combine(ModuleDirectory, "Private"));
    }
}
```

## API Integration

### C++ Header (ARCADIAFunctions.h)

```cpp
#pragma once

#include "CoreMinimal.h"
#include "Kismet/BlueprintFunctionLibrary.h"
#include "ARCADIAFunctions.generated.h"

// FFI declarations for Rust functions
extern "C" {
    void* arcadia_create_vector_index(const char* api_key);
    void arcadia_destroy_vector_index(void* index);
    float* arcadia_embed_text(void* index, const char* text, int* size);
    void arcadia_free_embedding(float* embedding);
}

USTRUCT(BlueprintType)
struct FARCADIAEmbedding
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadWrite)
    TArray<float> Values;
};

UCLASS()
class ARCADIA_API UARCADIAFunctions : public UBlueprintFunctionLibrary
{
    GENERATED_BODY()

public:
    /**
     * Initialize ARCADIA with API key
     */
    UFUNCTION(BlueprintCallable, Category = "ARCADIA")
    static bool InitializeARCADIA(const FString& ApiKey);

    /**
     * Generate text embedding
     */
    UFUNCTION(BlueprintCallable, Category = "ARCADIA|VectorIndex")
    static FARCADIAEmbedding GenerateEmbedding(const FString& Text);

    /**
     * Find similar content
     */
    UFUNCTION(BlueprintCallable, Category = "ARCADIA|VectorIndex")
    static TArray<FString> FindSimilar(const FString& Query, int32 Limit);

    /**
     * Create AI NPC with personality
     */
    UFUNCTION(BlueprintCallable, Category = "ARCADIA|AI")
    static bool CreateNPC(const FString& NPCName, const FString& Personality);

    /**
     * Generate procedural world section
     */
    UFUNCTION(BlueprintCallable, Category = "ARCADIA|Procedural")
    static TArray<FVector> GenerateWorldSection(FVector Center, float Radius);

private:
    static void* VectorIndex;
};
```

### C++ Implementation (ARCADIAFunctions.cpp)

```cpp
#include "ARCADIAFunctions.h"

void* UARCADIAFunctions::VectorIndex = nullptr;

bool UARCADIAFunctions::InitializeARCADIA(const FString& ApiKey)
{
    if (VectorIndex != nullptr)
    {
        UE_LOG(LogTemp, Warning, TEXT("ARCADIA already initialized"));
        return false;
    }

    std::string ApiKeyStd(TCHAR_TO_UTF8(*ApiKey));
    VectorIndex = arcadia_create_vector_index(ApiKeyStd.c_str());

    if (VectorIndex == nullptr)
    {
        UE_LOG(LogTemp, Error, TEXT("Failed to initialize ARCADIA"));
        return false;
    }

    UE_LOG(LogTemp, Log, TEXT("ARCADIA initialized successfully"));
    return true;
}

FARCADIAEmbedding UARCADIAFunctions::GenerateEmbedding(const FString& Text)
{
    FARCADIAEmbedding Result;

    if (VectorIndex == nullptr)
    {
        UE_LOG(LogTemp, Error, TEXT("ARCADIA not initialized"));
        return Result;
    }

    std::string TextStd(TCHAR_TO_UTF8(*Text));
    int Size = 0;
    float* Embedding = arcadia_embed_text(VectorIndex, TextStd.c_str(), &Size);

    if (Embedding != nullptr && Size > 0)
    {
        Result.Values.SetNum(Size);
        for (int i = 0; i < Size; i++)
        {
            Result.Values[i] = Embedding[i];
        }
        arcadia_free_embedding(Embedding);
    }

    return Result;
}

TArray<FString> UARCADIAFunctions::FindSimilar(const FString& Query, int32 Limit)
{
    TArray<FString> Results;

    // Implementation here
    // Call ARCADIA search function

    return Results;
}
```

## Example Use Cases

### 1. AI-Driven NPC Dialogue

**Blueprint Example:**

```
Event BeginPlay
  ↓
[Initialize ARCADIA] (API Key: "sk-...")
  ↓
[Create NPC] (Name: "Trader", Personality: "Friendly merchant")
```

**C++ Example:**

```cpp
void AMyNPC::BeginPlay()
{
    Super::BeginPlay();

    // Initialize ARCADIA
    FString ApiKey = TEXT("sk-...");
    UARCADIAFunctions::InitializeARCADIA(ApiKey);

    // Create NPC with AI personality
    FString Personality = TEXT("Friendly trader who loves rare artifacts");
    UARCADIAFunctions::CreateNPC(GetName(), Personality);
}

void AMyNPC::OnPlayerInteract()
{
    // Generate contextual dialogue
    FString Context = GetConversationContext();
    FARCADIAEmbedding Embedding = UARCADIAFunctions::GenerateEmbedding(Context);

    // Find similar past conversations
    TArray<FString> SimilarConversations =
        UARCADIAFunctions::FindSimilar(Context, 5);

    // Use AI to generate response
    GenerateDialogue(SimilarConversations);
}
```

### 2. Procedural World Generation

```cpp
void AWorldGenerator::GenerateChunk(FVector ChunkLocation)
{
    // Generate terrain features
    TArray<FVector> Features = UARCADIAFunctions::GenerateWorldSection(
        ChunkLocation,
        1000.0f  // Radius in units
    );

    // Place features in the world
    for (const FVector& Location : Features)
    {
        SpawnFeature(Location);
    }
}
```

### 3. Emotion-Adaptive Gameplay

```cpp
void AGameMode::AdjustDifficulty()
{
    // Get player stress level (from biometric sensors or gameplay)
    float StressLevel = GetPlayerStressLevel();

    // Generate embedding for current state
    FString GameState = FString::Printf(
        TEXT("Player stress: %.2f, Level: %d"),
        StressLevel,
        CurrentLevel
    );

    FARCADIAEmbedding StateEmbedding =
        UARCADIAFunctions::GenerateEmbedding(GameState);

    // Find similar states and their optimal difficulty settings
    TArray<FString> SimilarStates =
        UARCADIAFunctions::FindSimilar(GameState, 3);

    // Adjust gameplay parameters
    AdjustParameters(SimilarStates);
}
```

## Performance Optimization

### Threading Considerations

```cpp
// Async embedding generation
AsyncTask(ENamedThreads::AnyBackgroundThreadNormalTask, [this]()
{
    FARCADIAEmbedding Embedding =
        UARCADIAFunctions::GenerateEmbedding(LargeText);

    // Return to game thread
    AsyncTask(ENamedThreads::GameThread, [Embedding]()
    {
        ProcessEmbedding(Embedding);
    });
});
```

### Caching Strategy

```cpp
class ARCADIA_API UARCADIACache : public UObject
{
    GENERATED_BODY()

private:
    TMap<FString, FARCADIAEmbedding> EmbeddingCache;

public:
    FARCADIAEmbedding GetOrGenerateEmbedding(const FString& Text)
    {
        if (FARCADIAEmbedding* Cached = EmbeddingCache.Find(Text))
        {
            return *Cached;
        }

        FARCADIAEmbedding New = UARCADIAFunctions::GenerateEmbedding(Text);
        EmbeddingCache.Add(Text, New);
        return New;
    }
};
```

### Memory Management

```cpp
// Use smart pointers for ARCADIA objects
TSharedPtr<FARCADIAVectorIndex> VectorIndex;

// Clean up on shutdown
void Shutdown()
{
    if (VectorIndex.IsValid())
    {
        VectorIndex.Reset();
    }
}
```

## Troubleshooting

### Common Issues

**Issue**: Plugin fails to load
- **Solution**: Check that `arcadia.dll` is in the correct Binaries folder
- **Verify**: Plugin is enabled in Edit → Plugins

**Issue**: Linking errors
- **Solution**: Ensure `arcadia.lib` matches your build configuration (Debug/Release)
- **Check**: Platform architecture (x64)

**Issue**: Slow performance
- **Solution**: Enable caching, use async tasks
- **Monitor**: Check ARCADIA metrics for bottlenecks

**Issue**: API key errors
- **Solution**: Verify API key is valid and has sufficient credits
- **Check**: Network connectivity to OpenAI

### Debug Logging

```cpp
// Enable verbose logging
UE_LOG(LogTemp, Verbose, TEXT("ARCADIA: Generating embedding for: %s"), *Text);

// Log performance
double StartTime = FPlatformTime::Seconds();
FARCADIAEmbedding Embedding = GenerateEmbedding(Text);
double Duration = FPlatformTime::Seconds() - StartTime;
UE_LOG(LogTemp, Log, TEXT("Embedding took %.2f ms"), Duration * 1000.0);
```

## Best Practices

1. **Initialize Once**: Create ARCADIA instance at game start
2. **Use Async**: Run heavy operations on background threads
3. **Cache Aggressively**: Store embeddings and AI results
4. **Batch Operations**: Group similar requests together
5. **Monitor Metrics**: Track performance with ARCADIA metrics
6. **Error Handling**: Always check return values and log errors

## Next Steps

- [Example Game Project](../examples/unreal-game.md)
- [Advanced AI Integration](../tutorials/advanced-ai-unreal.md)
- [Performance Profiling](../tutorials/profiling-unreal.md)

## Resources

- [Unreal Engine Documentation](https://docs.unrealengine.com/)
- [ARCADIA API Reference](../api/)
- [Community Examples](https://github.com/yourusername/arcadia-examples)
