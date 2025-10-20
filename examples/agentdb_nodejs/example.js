/**
 * ARCADIA AgentDB Node.js Integration Example
 *
 * Demonstrates using AgentDB in a Node.js environment for
 * server-side agent learning and memory persistence.
 */

const { WasmAgentDb, WasmAgentDbConfig } = require('../../pkg-node/arcadia.js');

async function main() {
    console.log('🚀 ARCADIA AgentDB Node.js Example\n');

    // Create configuration
    console.log('📝 Creating AgentDB configuration...');
    const config = new WasmAgentDbConfig();
    config.set_db_name('arcadia_nodejs_agents');
    config.set_vector_dim(1536);

    // Initialize AgentDB
    console.log('🔧 Initializing AgentDB...');
    const agentDb = new WasmAgentDb(config);
    await agentDb.initialize();
    console.log('✅ AgentDB initialized!\n');

    // Simulate agent training episode
    console.log('🎮 Simulating agent training episode...\n');

    const agentId = 'training_agent_1';
    const actions = ['move_forward', 'turn_left', 'turn_right', 'attack', 'defend'];

    // Generate random experiences
    for (let i = 0; i < 100; i++) {
        const action = actions[Math.floor(Math.random() * actions.length)];
        const reward = (Math.random() - 0.3) * 2; // Biased towards positive rewards

        const experience = {
            id: `exp_${Date.now()}_${i}`,
            agent_id: agentId,
            state_vector: generateRandomVector(1536),
            action: action,
            reward: reward,
            next_state_vector: generateRandomVector(1536),
            done: i % 20 === 19, // Episode done every 20 steps
            metadata: {
                episode: Math.floor(i / 20).toString(),
                step: (i % 20).toString()
            },
            timestamp: Date.now()
        };

        await agentDb.store_experience(agentId, experience);

        if (i % 20 === 0) {
            console.log(`  Step ${i}: Action=${action}, Reward=${reward.toFixed(2)}`);
        }
    }

    console.log('\n✅ Stored 100 experiences\n');

    // Query similar experiences
    console.log('🔍 Querying similar experiences...');
    const queryVector = generateRandomVector(1536);
    const similarExperiences = await agentDb.query_similar(queryVector, 5);

    console.log(`Found ${similarExperiences.length} similar experiences:`);
    similarExperiences.forEach((exp, idx) => {
        console.log(`  ${idx + 1}. Action: ${exp.action}, Reward: ${exp.reward.toFixed(2)}`);
    });
    console.log('');

    // Get statistics
    console.log('📊 AgentDB Statistics:');
    const stats = agentDb.get_stats();
    console.log(`  - Total Experiences: ${stats.total_experiences}`);
    console.log(`  - Database Name: ${stats.db_name}`);
    console.log(`  - Vector Dimension: ${stats.vector_dim}`);
    console.log(`  - Initialized: ${stats.initialized ? '✅' : '❌'}`);
    console.log('');

    // Demonstrate high-reward experience filtering
    console.log('🏆 Analyzing agent performance...');
    const allExperiences = await agentDb.get_agent_experiences(agentId);
    const highRewardExperiences = allExperiences.filter(exp => exp.reward > 1.0);
    const avgReward = allExperiences.reduce((sum, exp) => sum + exp.reward, 0) / allExperiences.length;

    console.log(`  - Average Reward: ${avgReward.toFixed(2)}`);
    console.log(`  - High Reward Experiences: ${highRewardExperiences.length}`);
    console.log(`  - Success Rate: ${(highRewardExperiences.length / allExperiences.length * 100).toFixed(1)}%`);
    console.log('');

    // Demonstrate learning patterns
    console.log('🧠 Analyzing action patterns...');
    const actionStats = {};
    allExperiences.forEach(exp => {
        if (!actionStats[exp.action]) {
            actionStats[exp.action] = { count: 0, totalReward: 0 };
        }
        actionStats[exp.action].count++;
        actionStats[exp.action].totalReward += exp.reward;
    });

    console.log('  Action Performance:');
    Object.entries(actionStats).forEach(([action, stats]) => {
        const avgReward = stats.totalReward / stats.count;
        console.log(`    ${action}: ${stats.count} times, avg reward: ${avgReward.toFixed(2)}`);
    });
    console.log('');

    // Best action recommendation
    const bestAction = Object.entries(actionStats)
        .map(([action, stats]) => ({
            action,
            avgReward: stats.totalReward / stats.count
        }))
        .sort((a, b) => b.avgReward - a.avgReward)[0];

    console.log(`💡 Recommended Action: ${bestAction.action} (avg reward: ${bestAction.avgReward.toFixed(2)})`);
    console.log('');

    // Cleanup demo (optional)
    console.log('🧹 Cleaning up...');
    await agentDb.clear();
    console.log('✅ Data cleared\n');

    console.log('🎉 Example completed successfully!');
}

/**
 * Generate a random vector of specified dimension
 */
function generateRandomVector(dim) {
    return Array(dim).fill(0).map(() => Math.random() * 2 - 1);
}

/**
 * Error handler
 */
main().catch(error => {
    console.error('❌ Error:', error);
    process.exit(1);
});
