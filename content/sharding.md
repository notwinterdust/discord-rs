# Multi-Shard Auto Scaling

When a bot grows past 2,500 guilds, Discord requires traffic to be partitioned across multiple gateway WebSocket connections called **Shards**.

`discord-rust` provides a complete multi-shard orchestration architecture with **`ShardManager`** and **`ConcurrentShardRuntime`**.

---

## How Sharding Works

Each shard is identified by a tuple `[shard_id, shard_count]`:
- `shard_id`: The 0-based index of this specific connection.
- `shard_count`: The total number of shards operating in parallel.

Discord uses a deterministic formula to map a guild to a shard:
```text
shard_id = (guild_id >> 22) % shard_count
```

---

## Automatic Shard Recommendation

Instead of hardcoding shard counts, `discord-rust` can query Discord's `/gateway/bot` endpoint to dynamically calculate the optimal number of shards:

```rust
use discord_rust::clients::shard::ShardManager;

// 1. Automatically calculate from Discord's recommendation
// In this example: 10 total shards, max 1 concurrency per bucket
let manager = ShardManager::from_gateway_recommendation(10, 1)
    .expect("Valid shard configuration");

println!("Calculated total shards: {}", manager.total_shards());
```

---

## Spawning Concurrent Shard Workers

The [`ConcurrentShardRuntime`](#) spawns a dedicated native OS thread for each shard:

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use discord_rust::clients::shard::ConcurrentShardRuntime;

let runtime = ConcurrentShardRuntime::new(manager);

// Spawn background workers
let handles = runtime.spawn(|shard_id, stop_flag: Arc<AtomicBool>| {
    println!("Shard {} started on thread {:?}", shard_id, std::thread::current().id());

    while !stop_flag.load(Ordering::SeqCst) {
        // Shard gateway loop polling frames and dispatching events
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    println!("Shard {} cleanly stopped.", shard_id);
    Ok(())
});

// Wait for all shards to finish or handle termination signals
for handle in handles {
    let _ = handle.join();
}
```

> [!TIP]
> Because shards run on isolated OS threads, high CPU load or heavy message traffic on one shard will **never** stall or lag other shards.
