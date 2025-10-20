# ARCADIA AgentDB WASM Browser Example

This example demonstrates ARCADIA's AgentDB capabilities running entirely in the browser using WebAssembly.

## Features

- **Persistent Memory**: Store agent experiences in the browser using IndexedDB
- **Experience Replay**: Sample and replay past experiences for learning
- **Vector Similarity**: Query similar experiences using cosine similarity
- **Real-time Stats**: Monitor agent learning metrics
- **Browser-based**: No server required, runs entirely client-side

## Running the Example

### Method 1: Using Make

```bash
# From the ARCADIA root directory
make build-wasm
cd examples/agentdb_wasm
python3 -m http.server 8080
```

Then open http://localhost:8080 in your browser.

### Method 2: Using wasm-pack directly

```bash
# From the ARCADIA root directory
wasm-pack build --target web --out-dir examples/agentdb_wasm/pkg
cd examples/agentdb_wasm
python3 -m http.server 8080
```

### Method 3: Using npm

```bash
# From the ARCADIA root directory
npm run build
npm run serve
```

## Usage

### Adding Experiences

1. Enter an Agent ID (e.g., "agent_1")
2. Specify an action (e.g., "move_forward", "attack", "defend")
3. Set a reward value (positive for good outcomes, negative for bad)
4. Click "Add Experience"

### Querying Similar Experiences

1. Enter an action to query
2. Click "Query Similar"
3. View the top 5 most similar experiences

### Sampling for Training

Click "Sample Replay" to simulate sampling a batch of experiences for reinforcement learning training.

## Architecture

```
┌─────────────────────────────────────┐
│         Browser (JavaScript)         │
│  - User Interface                    │
│  - Experience Collection             │
│  - Visualization                     │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│      WASM Module (Rust)              │
│  - AgentDB Core Logic                │
│  - Vector Similarity                 │
│  - Experience Replay Buffer          │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│      Browser Storage                 │
│  - IndexedDB (persistent)            │
│  - Memory Cache (fast access)        │
└─────────────────────────────────────┘
```

## API Reference

### WasmAgentDb

```javascript
// Initialize
const config = new WasmAgentDbConfig();
const agentDb = new WasmAgentDb(config);
await agentDb.initialize();

// Store experience
await agentDb.store_experience(agentId, experienceObject);

// Query similar
const results = await agentDb.query_similar(vectorArray, limit);

// Get stats
const stats = agentDb.get_stats();

// Clear data
await agentDb.clear();
```

### Experience Object Structure

```javascript
{
    id: "exp_1234567890",
    agent_id: "agent_1",
    state_vector: [/* 1536 floats */],
    action: "move_forward",
    reward: 1.0,
    next_state_vector: [/* 1536 floats */],
    done: false,
    metadata: {},
    timestamp: 1234567890
}
```

## Browser Compatibility

- Chrome 87+
- Firefox 89+
- Safari 15+
- Edge 88+

All modern browsers with WebAssembly support.

## Performance Notes

- Initial WASM load: ~100ms
- Experience storage: <1ms
- Vector similarity search: ~10ms for 1000 experiences
- IndexedDB persistence: ~5ms per write

## Limitations

- Vector dimension fixed at 1536 (OpenAI embedding size)
- Browser memory limits apply (~2GB typical)
- No server-side persistence (data stored locally)

## Next Steps

- Try the Node.js example for server-side usage
- Integrate with ARCADIA's PARIS learning framework
- Connect to actual game AI agents
- Export/import experiences across sessions
