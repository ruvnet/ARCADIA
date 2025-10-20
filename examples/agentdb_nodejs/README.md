# ARCADIA AgentDB Node.js Example

This example demonstrates how to use ARCADIA's AgentDB in a Node.js environment for server-side agent learning and memory persistence.

## Features

- **Server-side Agent Learning**: Train AI agents with experience replay
- **Memory Persistence**: Store agent memories for long-term learning
- **Vector Similarity Search**: Query similar experiences efficiently
- **Performance Analytics**: Track agent learning metrics and patterns
- **Action Optimization**: Identify best-performing actions

## Installation

```bash
# From the ARCADIA root directory
cd examples/agentdb_nodejs

# Build the WASM module for Node.js
npm run build

# Run the example
npm start
```

## What This Example Does

1. **Initializes AgentDB** with Node.js-compatible configuration
2. **Simulates 100 agent experiences** with various actions and rewards
3. **Queries similar experiences** using vector similarity
4. **Analyzes performance** by action type
5. **Recommends optimal actions** based on historical rewards

## Example Output

```
🚀 ARCADIA AgentDB Node.js Example

📝 Creating AgentDB configuration...
🔧 Initializing AgentDB...
✅ AgentDB initialized!

🎮 Simulating agent training episode...

  Step 0: Action=attack, Reward=0.85
  Step 20: Action=move_forward, Reward=-0.23
  Step 40: Action=defend, Reward=1.42
  ...

✅ Stored 100 experiences

🔍 Querying similar experiences...
Found 5 similar experiences:
  1. Action: move_forward, Reward: 0.67
  2. Action: attack, Reward: 1.23
  3. Action: turn_left, Reward: -0.15
  ...

📊 AgentDB Statistics:
  - Total Experiences: 100
  - Database Name: arcadia_nodejs_agents
  - Vector Dimension: 1536
  - Initialized: ✅

🏆 Analyzing agent performance...
  - Average Reward: 0.42
  - High Reward Experiences: 28
  - Success Rate: 28.0%

🧠 Analyzing action patterns...
  Action Performance:
    move_forward: 23 times, avg reward: 0.38
    turn_left: 18 times, avg reward: 0.45
    turn_right: 21 times, avg reward: 0.41
    attack: 19 times, avg reward: 0.52
    defend: 19 times, avg reward: 0.35

💡 Recommended Action: attack (avg reward: 0.52)

🎉 Example completed successfully!
```

## Use Cases

### 1. Game Server AI

```javascript
const agentDb = new WasmAgentDb(config);
await agentDb.initialize();

// Store player interactions
await agentDb.store_experience(npcId, {
    state_vector: gameState.toVector(),
    action: npc.lastAction,
    reward: calculateReward(outcome),
    // ...
});

// Query for best responses
const similar = await agentDb.query_similar(
    currentState.toVector(),
    10
);
```

### 2. Reinforcement Learning Training

```javascript
// Collect experiences during gameplay
for (const step of episode) {
    await agentDb.store_experience(agentId, step);
}

// Sample batch for training
const batch = await sampleExperienceBatch(agentDb, 32);
await trainModel(batch);
```

### 3. Behavioral Analysis

```javascript
// Analyze agent performance over time
const experiences = await agentDb.get_agent_experiences(agentId);
const metrics = analyzePerformance(experiences);
console.log(`Win rate: ${metrics.winRate}%`);
```

## API Reference

### Initialization

```javascript
const config = new WasmAgentDbConfig();
config.set_db_name('my_agents');
config.set_vector_dim(1536);

const agentDb = new WasmAgentDb(config);
await agentDb.initialize();
```

### Storing Experiences

```javascript
await agentDb.store_experience(agentId, {
    id: 'unique_id',
    agent_id: 'agent_1',
    state_vector: [/* array of floats */],
    action: 'move_forward',
    reward: 1.0,
    next_state_vector: [/* array of floats */],
    done: false,
    metadata: {},
    timestamp: Date.now()
});
```

### Querying

```javascript
// Query by similarity
const results = await agentDb.query_similar(vectorArray, limit);

// Get all agent experiences
const experiences = await agentDb.get_agent_experiences(agentId);

// Get statistics
const stats = agentDb.get_stats();
```

## Integration with ARCADIA

This example can be integrated with:

- **PARIS Framework**: Feed experiences to adaptive learning systems
- **VIVIAN**: Store embeddings in vector index for semantic search
- **Game Logic**: Use learned behaviors to control NPC actions

## Performance

- Experience storage: <1ms per operation
- Vector similarity search: ~5-10ms for 1000 experiences
- Batch operations: ~100k experiences/second
- Memory usage: ~1MB per 1000 experiences

## Next Steps

1. Connect to actual game server
2. Implement continuous learning loop
3. Add experience prioritization
4. Export/import trained agents
5. Integrate with ARCADIA's PARIS learning framework

## Troubleshooting

### WASM Module Not Found

```bash
# Rebuild the WASM module
cd ../..
wasm-pack build --target nodejs --out-dir examples/agentdb_nodejs/pkg-node
```

### Memory Issues

If running out of memory with large experience buffers, adjust the configuration:

```javascript
config.set_max_memory_mb(256); // Limit memory usage
```

## License

MIT OR Apache-2.0
